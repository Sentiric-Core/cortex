#!/bin/bash

# Renkler
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Klasör Yolları
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."
BIN_CORTEX="$PROJECT_ROOT/target/release/cortex"
BIN_LLAMA="$PROJECT_ROOT/assets/llama.cpp/build/bin/llama-server"
MODEL_LLAMA="$PROJECT_ROOT/assets/models/Llama-3.2-1B-Instruct-Q4_K_M.gguf"

# Kontrol: Kurulum yapılmış mı?
if [ ! -f "$BIN_CORTEX" ] || [ ! -f "$BIN_LLAMA" ]; then
    echo -e "${RED}[ERROR] Binary dosyaları bulunamadı.${NC}"
    echo "Lütfen önce kurulumu çalıştırın: ./scripts/setup.sh"
    exit 1
fi

echo -e "${BLUE}=== SENTIRIC CORE STARTUP ===${NC}"

# Temizlik
pkill -f llama-server > /dev/null 2>&1

# 1. Beyni Başlat
echo -e "${GREEN}[SYSTEM] Neural Engine (Brain) başlatılıyor...${NC}"
# Logları gizle, sadece hata varsa göster
"$BIN_LLAMA" -m "$MODEL_LLAMA" --port 8080 -c 2048 > /dev/null 2>&1 &
SERVER_PID=$!

# 2. Bağlantı Bekle
echo -n "[SYSTEM] Synapse bağlantısı bekleniyor"
until curl -s http://127.0.0.1:8080/health > /dev/null; do
    echo -n "."
    sleep 1
    if ! kill -0 $SERVER_PID 2>/dev/null; then
        echo -e "\n${RED}[CRITICAL] Sunucu başlatılamadı!${NC}"
        exit 1
    fi
done
echo -e "\n${GREEN}[SYSTEM] Beyin aktif (PID: $SERVER_PID).${NC}"

# Çıkış Trap'i
cleanup() {
    echo -e "\n${BLUE}[SHUTDOWN] Sistem kapatılıyor...${NC}"
    kill $SERVER_PID
    wait $SERVER_PID 2>/dev/null
    echo -e "${GREEN}[DONE] Güvenli çıkış.${NC}"
}
trap cleanup EXIT

# 3. Bedeni Başlat
echo -e "${GREEN}[SYSTEM] Cortex (Body) başlatılıyor...${NC}"
"$BIN_CORTEX"