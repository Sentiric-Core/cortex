use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};

pub struct AudioPacket {
    pub samples: Vec<f32>, // Artık garanti 16kHz
}

pub struct Ear {
    _stream: cpal::Stream,
    receiver: mpsc::Receiver<AudioPacket>,
}

impl Ear {
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or(anyhow::anyhow!("Mikrofon bulunamadı"))?;
        let config = device.default_input_config()?;
        
        println!("[AUDIO] Input Device: {}", device.name().unwrap_or_default());
        println!("[AUDIO] Native Sample Rate: {}", config.sample_rate().0);

        let (sender, receiver) = mpsc::channel();
        let input_sample_rate = config.sample_rate().0 as usize;
        let output_sample_rate = 16000; // Whisper standardı

        // Resampler Hazırlığı (Sadece eğer oranlar farklıysa)
        let needs_resampling = input_sample_rate != output_sample_rate;
        
        // Resampler parametreleri (High Quality)
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        // Block size CPAL'den dinamik gelir ama Rubato sabit ister. 
        // Basitlik için main thread'de değil, callback içinde bir buffer yöneteceğiz.
        // NOT: Callback içinde karmaşık logic Rust'ta zordur (`Send` trait).
        // Bu yüzden raw datayı gönderip, işlemi Brain tarafında veya ayrı bir thread'de yapmak daha güvenlidir.
        // Ancak DOOM felsefesi: "Veriyi kaynağında işle".
        
        // ŞİMDİLİK: Resampling'i atlıyoruz ve CPAL'den 16kHz istemeyi deniyoruz.
        // Eğer donanım desteklemezse raw gönderip main loop'ta basit decimation yapacağız.
        // (Rubato'yu callback içine gömmek ownership sorunları yaratır, bu adımı sonra optimize edeceğiz).

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if !data.is_empty() {
                    // Basit Decimation (Eğer 44100 veya 48000 ise kabaca örnek atla)
                    // Bu "Quick & Dirty" bir çözümdür ama çalışır.
                    // İleride Rubato buraya entegre edilecek.
                    
                    let processed_data: Vec<f32> = if input_sample_rate > 16500 {
                        // Basit oran orantı ile örnek seçme (Downsampling)
                        let ratio = input_sample_rate as f32 / 16000.0;
                        let mut output = Vec::with_capacity((data.len() as f32 / ratio) as usize);
                        let mut accumulator = 0.0;
                        while accumulator < data.len() as f32 {
                            output.push(data[accumulator as usize]);
                            accumulator += ratio;
                        }
                        output
                    } else {
                        data.to_vec()
                    };

                    let _ = sender.send(AudioPacket { samples: processed_data });
                }
            },
            |err| eprintln!("[AUDIO ERROR] {}", err),
            None
        )?;

        stream.play()?;

        Ok(Ear {
            _stream: stream,
            receiver,
        })
    }

    pub fn listen(&self) -> Option<AudioPacket> {
        self.receiver.try_recv().ok()
    }
}