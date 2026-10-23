#!/usr/bin/env bash
# Türetilmiş tüm uygulama ikonlarını varliklar/uygulama-simgesi/sp-group-cad.png dosyasından üretir.
# Çıktılar depoda izlendiği için derleme ve paketleme bu betiği çalıştırmaz.
#
#   paketleme/ikonlar.sh
set -euo pipefail
KOK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$KOK/paketleme/ikonlar.py"
