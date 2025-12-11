# Sentiric Cortex

[![CI](https://github.com/Sentiric-Core/cortex/actions/workflows/release.yml/badge.svg)](https://github.com/Sentiric-Core/cortex/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v1.0.0-green.svg)](https://github.com/Sentiric-Core/cortex/releases)
[![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)](https://www.rust-lang.org/)

**Sentiric Cortex**, yerel donanım üzerinde çalışan, bulut bağımsız, düşük gecikmeli bir Bilişsel Sinyal İşleme (Computational Signal Processing) motorudur. DOOM oyun motorunun felsefesiyle (Bare-metal, Zero-bloat) tasarlanmıştır.

## 🧠 Mimari (The Sidecar Pattern)

Sistem iki ana lobdan oluşur:

1.  **Cortex (Rust):** Beden. Donanım kontrolü, ses I/O, VAD (Voice Activity Detection).
2.  **Brain (C++):** Zeka. `llama.cpp` tabanlı LLM sunucusu.

```mermaid
graph LR
    A[Microphone] -->|PCM Audio| B(Cortex / Ear)
    B -->|Resampled Audio| C{Whisper STT}
    C -->|Text| D[Mind / Logic]
    D -->|JSON| E((Llama-3.2 Server))
    E -->|JSON Response| D
    D -->|Text| F[Mouth / TTS]
    F -->|Audio Stream| G[Speaker]
```

## 🚀 Hızlı Başlangıç (Linux)

Tek komutla kurulum ve çalıştırma:

```bash
# 1. Repoyu klonlayın
git clone https://github.com/Sentiric-Core/cortex.git
cd cortex

# 2. Kurulum (Bağımlılıklar + Modeller + Derleme)
chmod +x scripts/setup.sh
./scripts/setup.sh

# 3. Başlatma
chmod +x scripts/launch.sh
./scripts/launch.sh
```

## 🛠️ Teknoloji Yığını

- **Dil:** Rust (Edition 2021) & C++
- **STT:** Whisper.cpp (Gömülü)
- **LLM:** Llama-3.2-1B-Instruct (Quantized Q4_K_M)
- **TTS:** Piper (Neural Text-to-Speech)
- **İletişim:** HTTP/JSON (Localhost)

## ⚖️ Lisans

Bu proje **AGPLv3** ile lisanslanmıştır.
