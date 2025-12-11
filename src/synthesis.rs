/*
 * MODÜL: SYNTHESIS (ACT)
 * Sorumluluk: Metni sese çevirmek (TTS) ve çalmak.
 * Motor: Piper (Subprocess)
 */

use std::process::{Command, Stdio};
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub struct Mouth {
    // Rodio output stream, düşmemesi (drop olmaması) için tutulmalı
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

impl Mouth {
    pub fn new() -> anyhow::Result<Self> {
        // Ses çıkış cihazını başlat
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| anyhow::anyhow!("Hoparlör başlatılamadı: {}", e))?;
        
        println!("[SYNTHESIS] Ses Motoru Aktif (Rodio + Piper).");
        Ok(Mouth {
            _stream,
            _stream_handle: stream_handle,
        })
    }

    pub fn speak(&self, text: &str) {
        println!("[ACT] Konuşuluyor: \"{}\"", text);
        
        let output_file = "temp_speech.wav";
        
        // DEĞİŞİKLİK BURADA: 'mut' kaldırıldı
        let child = Command::new("./assets/piper/piper/piper")
            .arg("--model")
            .arg("assets/piper/en_US-ryan-medium.onnx")
            .arg("--output_file")
            .arg(output_file)
            .stdin(Stdio::piped()) // Metni stdin'den vereceğiz
            .stdout(Stdio::null()) // Logları gizle
            .stderr(Stdio::null())
            .spawn();

        if let Ok(mut child_process) = child {
            // Metni Piper'a gönder
            if let Some(mut stdin) = child_process.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
            }
            // İşlemin bitmesini bekle
            let _ = child_process.wait();

            // 2. Sesi Çal (Rodio)
            self.play_audio(output_file);
        } else {
            eprintln!("[SYNTHESIS ERROR] Piper çalıştırılamadı! Yolu kontrol et.");
        }
    }

    fn play_audio(&self, path: &str) {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return, // Dosya yoksa sessiz kal
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(_) => return,
        };

        // Sink: Sesi yöneten kanal
        if let Ok(sink) = Sink::try_new(&self._stream_handle) {
            sink.append(source);
            sink.sleep_until_end(); // Ses bitene kadar bekle (Senkron Konuşma)
        }
    }
}