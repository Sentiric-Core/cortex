/*
 * MODÜL: INTELLIGENCE (THINK)
 * Sorumluluk: Yerel Llama Sunucusu ile HTTP üzerinden konuşmak.
 * Yöntem: Curl (Zero-Dependency)
 */

use std::process::Command;
use serde::{Deserialize, Serialize};

// Llama Server İstek Formatı
#[derive(Serialize)]
struct CompletionRequest {
    prompt: String,
    n_predict: i32,
    temperature: f32,
    stop: Vec<String>,
}

// Llama Server Cevap Formatı
#[derive(Deserialize)]
struct CompletionResponse {
    content: String,
}

pub struct Mind;

impl Mind {
    pub fn new() -> anyhow::Result<Self> {
        // Sunucunun ayakta olup olmadığını kontrol etmiyoruz (Lazy init).
        println!("[INTELLIGENCE] Bağlantı Modu: HTTP (localhost:8080)");
        Ok(Mind)
    }

    pub fn think(&self, user_input: &str) -> Option<String> {
        // YENİ SİSTEM KOMUTU
        let prompt = format!(
            "<|start_header_id|>system<|end_header_id|>\n\
            You are Sentiric, an intelligent system interface running locally. \
            Be helpful, precise, and concise. Do not use markdown or emojis.<|eot_id|>\n\
            <|start_header_id|>user<|end_header_id|>\n\
            {}<|eot_id|>\n\
            <|start_header_id|>assistant<|end_header_id|>\n",
            user_input
        );

        let request_body = CompletionRequest {
            prompt,
            n_predict: 64, // Kısa cevap için limit
            temperature: 0.7,
            stop: vec!["<|eot_id|>".to_string()],
        };

        // JSON'a çevir
        let json_body = serde_json::to_string(&request_body).ok()?;

        println!("[MIND] Düşünülüyor (HTTP Request)...");

        // Curl ile HTTP POST isteği at
        let output = Command::new("curl")
            .arg("-s") // Sessiz mod
            .arg("-X").arg("POST")
            .arg("http://127.0.0.1:8080/completion")
            .arg("-H").arg("Content-Type: application/json")
            .arg("-d").arg(&json_body)
            .output();

        match output {
            Ok(result) if result.status.success() => {
                let response_text = String::from_utf8_lossy(&result.stdout);
                
                // Gelen JSON'u parse et
                if let Ok(response_json) = serde_json::from_str::<CompletionResponse>(&response_text) {
                    let clean_response = response_json.content.trim().to_string();
                    // Boş cevap dönerse None ver
                    if clean_response.is_empty() { None } else { Some(clean_response) }
                } else {
                    eprintln!("[MIND ERROR] Anlamsız Cevap: {}", response_text);
                    None
                }
            },
            Ok(result) => {
                eprintln!("[MIND ERROR] Sunucu Hatası: {}", String::from_utf8_lossy(&result.stderr));
                None
            }
            Err(e) => {
                eprintln!("[MIND CRITICAL] Curl çalıştırılamadı: {}. Sunucu açık mı?", e);
                None
            }
        }
    }
}