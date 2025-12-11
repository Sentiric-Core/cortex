use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

pub struct AudioPacket {
    pub samples: Vec<f32>,
}

pub struct Ear {
    _stream: cpal::Stream,
    receiver: mpsc::Receiver<AudioPacket>,
}

impl Ear {
    pub fn new() -> anyhow::Result<Self> {
        // ... (Bu kısım öncekiyle aynı, sadece listen fonksiyonunun altına ekleme yapıyoruz)
        // Kodu kısaltmamak kuralı gereği tamamını veriyorum:
        
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or(anyhow::anyhow!("Mikrofon bulunamadı"))?;
        let config = device.default_input_config()?;
        
        let (sender, receiver) = mpsc::channel();
        
        let input_rate = config.sample_rate().0 as f32;
        let output_rate = 16000.0;
        let channels = config.channels() as usize;
        let ratio = input_rate / output_rate;

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if data.is_empty() { return; }

                // Stereo -> Mono
                let mut mono_samples = Vec::with_capacity(data.len() / channels);
                for frame in data.chunks(channels) {
                    let sum: f32 = frame.iter().sum();
                    mono_samples.push(sum / channels as f32);
                }

                // Resampling
                let mut resampled_samples = Vec::with_capacity((mono_samples.len() as f32 / ratio) as usize + 1);
                let mut i = 0;
                while (i as f32 * ratio) < mono_samples.len() as f32 {
                    let index = (i as f32 * ratio) as usize;
                    if index < mono_samples.len() {
                        resampled_samples.push(mono_samples[index]);
                    }
                    i += 1;
                }
                
                if !resampled_samples.is_empty() {
                    let _ = sender.send(AudioPacket { samples: resampled_samples });
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

    // YENİ: Kulağın duyduğu her şeyi unutmasını sağlar
    pub fn clear_buffer(&self) {
        // Kanalda biriken tüm paketleri çek ve yok et
        while let Ok(_) = self.receiver.try_recv() {}
    }
}