use whisper_rs::{WhisperContext, FullParams, SamplingStrategy};
use std::path::Path;

pub struct Brain {
    ctx: WhisperContext,
}

impl Brain {
    pub fn new() -> anyhow::Result<Self> {
        println!("[COGNITION] Yükleniyor: assets/models/ggml-base.en.bin");
        
        let path = Path::new("assets/models/ggml-base.en.bin");
        if !path.exists() {
            return Err(anyhow::anyhow!("Model dosyası bulunamadı! Lütfen 'wget' komutunu çalıştırın."));
        }

        // Modeli belleğe yükle (AVX optimizasyonlu)
        let ctx = WhisperContext::new_with_params(
            path.to_str().unwrap(), 
            whisper_rs::WhisperContextParameters::default()
        ).map_err(|e| anyhow::anyhow!("Whisper Init Error: {:?}", e))?;

        println!("[COGNITION] Nöral Ağlar Aktif (Whisper Loaded).");
        Ok(Brain { ctx })
    }

    // Ses verisini al, metne çevir
    pub fn transcribe(&self, audio_data: &[f32]) -> Option<String> {
        // Whisper en az 1 saniyelik veri ister, çok kısa veriyi yoksay
        if audio_data.len() < 16000 { 
            return None; 
        }

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en")); // Şimdilik İngilizce
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Inference (Çıkarım) İşlemi - BU İŞLEM CPU'yu KULLANIR
        let mut state = self.ctx.create_state().ok()?;
        
        if let Err(e) = state.full(params, audio_data) {
            eprintln!("[COGNITION ERROR] Inference failed: {:?}", e);
            return None;
        }

        // Sonuçları topla
        let num_segments = state.full_n_segments().ok()?;
        let mut full_text = String::new();

        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                full_text.push_str(&segment);
            }
        }

        if full_text.trim().is_empty() {
            None
        } else {
            Some(full_text)
        }
    }
}