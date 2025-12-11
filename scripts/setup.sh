#!/bin/bash
set -e # Hata olursa dur

# Renkler
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}=== SENTIRIC CORTEX: SETUP & BUILD ===${NC}"

# 1. Proje Kök Dizinini Bul
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."
ASSETS_DIR="$PROJECT_ROOT/assets"
MODELS_DIR="$ASSETS_DIR/models"
PIPER_DIR="$ASSETS_DIR/piper"
LLAMA_CPP_DIR="$ASSETS_DIR/llama.cpp"

# 2. Sistem Bağımlılıklarını Kontrol Et (Debian/Ubuntu)
echo -e "${GREEN}[1/5] Sistem bağımlılıkları kontrol ediliyor...${NC}"
if command -v apt-get &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y build-essential cmake libasound2-dev curl libclang-dev git pkg-config
else
    echo "Uyarı: apt-get bulunamadı. Lütfen 'cmake', 'libasound2-dev' ve 'build-essential' paketlerinin kurulu olduğundan emin olun."
fi

# 3. Klasörleri Oluştur
mkdir -p "$MODELS_DIR"
mkdir -p "$PIPER_DIR"

# 4. Modelleri İndir
echo -e "${GREEN}[2/5] AI Modelleri indiriliyor...${NC}"

# 4.1 Whisper (STT)
if [ ! -f "$MODELS_DIR/ggml-base.en.bin" ]; then
    echo "Whisper Modeli indiriliyor..."
    wget -O "$MODELS_DIR/ggml-base.en.bin" https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
fi

# 4.2 Llama (LLM) - GÜNCEL LİNK (Bartowski)
if [ ! -f "$MODELS_DIR/Llama-3.2-1B-Instruct-Q4_K_M.gguf" ]; then
    echo "Llama 3.2 Modeli indiriliyor..."
    wget -O "$MODELS_DIR/Llama-3.2-1B-Instruct-Q4_K_M.gguf" https://huggingface.co/bartowski/Llama-3.2-1B-Instruct-GGUF/resolve/main/Llama-3.2-1B-Instruct-Q4_K_M.gguf
fi

# 4.3 Piper (TTS)
if [ ! -f "$PIPER_DIR/piper" ]; then
    echo "Piper TTS indiriliyor..."
    cd "$PIPER_DIR"
    wget https://github.com/rhasspy/piper/releases/download/2023.11.14-2/piper_linux_x86_64.tar.gz
    tar -xvf piper_linux_x86_64.tar.gz
    rm piper_linux_x86_64.tar.gz
    
    # Piper Voice (Ryan)
    wget -O en_US-ryan-medium.onnx https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/ryan/medium/en_US-ryan-medium.onnx
    wget -O en_US-ryan-medium.onnx.json https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/ryan/medium/en_US-ryan-medium.onnx.json
fi

# 5. Llama.cpp Derle (Beyin)
echo -e "${GREEN}[3/5] Llama.cpp Sunucusu derleniyor...${NC}"
if [ ! -d "$LLAMA_CPP_DIR" ]; then
    git clone https://github.com/ggerganov/llama.cpp "$LLAMA_CPP_DIR"
fi

cd "$LLAMA_CPP_DIR"
# Temiz bir build için
rm -rf build
mkdir build
cd build
# CURL kapalı (Zero-dependency)
cmake .. -DLLAMA_CURL=OFF
cmake --build . --config Release -j$(nproc)

# 6. Cortex Derle (Beden)
echo -e "${GREEN}[4/5] Sentiric Cortex (Rust) derleniyor...${NC}"
cd "$PROJECT_ROOT"
cargo build --release

echo -e "${GREEN}[5/5] KURULUM TAMAMLANDI!${NC}"
echo "Çalıştırmak için: ./scripts/launch.sh"