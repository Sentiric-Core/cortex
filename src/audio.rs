/*
 * MODÜL: AUDIO (SENSE)
 * Sorumluluk: Mikrofonu dinlemek ve veriyi ana döngüye iletmek.
 * Mimari: Asenkron Stream (Thread-based)
 */

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::thread;

pub struct AudioPacket {
    pub samples: Vec<f32>,
    pub timestamp: u64,
}

pub struct Ear {
    // Stream, scope dışına çıktığında kapanır, bu yüzden struct içinde tutuyoruz.
    _stream: cpal::Stream, 
    receiver: mpsc::Receiver<AudioPacket>,
}

impl Ear {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("[AUDIO] Ses sürücüleri taranıyor...");

        // 1. Host (İşletim Sistemi Ses Sunucusu) Bağlantısı
        let host = cpal::default_host();

        // 2. Varsayılan Giriş Cihazını (Mikrofon) Bul
        let device = host.default_input_device()
            .ok_or("Mikrofon bulunamadı! Bir giriş cihazı bağlı mı?")?;
        
        println!("[AUDIO] Cihaz bulundu: {}", device.name()?);

        // 3. Cihaz Konfigürasyonu (Varsayılan ayarları al)
        let config = device.default_input_config()?;
        println!("[AUDIO] Config: Kanal: {}, Sample Rate: {}", config.channels(), config.sample_rate().0);

        // 4. Veri Kanalı (Thread'ler arası iletişim)
        let (sender, receiver) = mpsc::channel();

        // 5. Stream Oluşturma (Arka planda sürekli çalışır)
        let err_fn = move |err| {
            eprintln!("[AUDIO ERROR] Ses akış hatası: {}", err);
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Veriyi kopyalayıp ana motora gönderiyoruz
                    // Not: Production'da RingBuffer kullanılmalı, şimdilik Vec.
                    if !data.is_empty() {
                        let packet = AudioPacket {
                            samples: data.to_vec(),
                            timestamp: 0, // TODO: Gerçek zaman damgası eklenecek
                        };
                        // Hata olursa (Receiver kapalıysa) sessizce yut
                        let _ = sender.send(packet);
                    }
                },
                err_fn,
                None
            )?,
            _ => return Err("Desteklenmeyen ses formatı (Sadece F32 desteklenir)".into()),
        };

        // 6. Dinlemeye Başla
        stream.play()?;
        println!("[AUDIO] Dinleme aktif (Stream Started).");

        Ok(Ear {
            _stream: stream,
            receiver,
        })
    }

    // Ana döngüden çağrılır: "Yeni duyduğun bir şey var mı?"
    pub fn listen(&self) -> Option<AudioPacket> {
        // Bloklamadan (Non-blocking) veriyi al
        match self.receiver.try_recv() {
            Ok(packet) => Some(packet),
            Err(_) => None, // Veri yok
        }
    }
}