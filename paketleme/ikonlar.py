#!/usr/bin/env python3
"""Uygulama ikonlarını tek kaynaktan üretir.

Kaynak: varliklar/uygulama-simgesi/sp-group-cad.png
Çıktı: varliklar/uygulama-simgesi/ altındaki PNG kümesi, spcad.ico ve spcad.icns.

İkonlar depoda izlendiği için derleme ve paketleme bu betiği çalıştırmaz;
yalnız kaynak logo değiştiğinde yeniden üretmek için kullanılır:

    python paketleme/ikonlar.py
"""

from __future__ import annotations

import struct
import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:  # pragma: no cover - ortam bağımlılığı
    sys.exit("Pillow gerekli: python -m pip install Pillow")

KOK = Path(__file__).resolve().parent.parent
IKON_DIZINI = KOK / "assets" / "app-icon"
KAYNAK = IKON_DIZINI / "sp-group-cad.png"
UYGULAMA_KIMLIGI = "com.spgroup.spcad"
HICOLOR_BOYUTLARI = (16, 24, 32, 48, 64, 128, 256, 512)
ICO_BOYUTLARI = ((16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256))
ICNS_TURLERI = {16: b"icp4", 32: b"icp5", 64: b"icp6", 128: b"ic07", 256: b"ic08", 512: b"ic09", 1024: b"ic10"}
MACOS_ICERIK_ORANI = 0.8


def kare_yukle(kaynak: Path, boyut: int) -> Image.Image:
    """Kaynağı kırpmadan saydam zemin üzerinde kare tuvale ortalar."""
    goruntu = Image.open(kaynak).convert("RGBA")
    kenar = max(goruntu.size)
    tuval = Image.new("RGBA", (kenar, kenar), (0, 0, 0, 0))
    tuval.paste(goruntu, ((kenar - goruntu.width) // 2, (kenar - goruntu.height) // 2), goruntu)
    return tuval.resize((boyut, boyut), Image.Resampling.LANCZOS)


def macos_ana(kaynak: Path) -> Image.Image:
    """Apple ızgarası: 512 tuvalin %80'ini dolduran içerik."""
    kenar = 512
    icerik = round(kenar * MACOS_ICERIK_ORANI)
    ic_pay = (kenar - icerik) // 2
    tuval = Image.new("RGBA", (kenar, kenar), (0, 0, 0, 0))
    tuval.paste(kare_yukle(kaynak, icerik), (ic_pay, ic_pay))
    return tuval


def pngleri_yaz(kaynak: Path) -> Image.Image:
    ana_goruntu = kare_yukle(kaynak, 1024)
    ana_goruntu.save(IKON_DIZINI / "spcad-1024.png", optimize=True)
    for boyut in (64, 256):
        ana_goruntu.resize((boyut, boyut), Image.Resampling.LANCZOS).save(IKON_DIZINI / f"spcad-{boyut}.png", optimize=True)
    macos_ana(kaynak).save(IKON_DIZINI / "spcad-macos-512.png", optimize=True)
    for boyut in HICOLOR_BOYUTLARI:
        hedef = IKON_DIZINI / "hicolor" / f"{boyut}x{boyut}" / "apps" / f"{UYGULAMA_KIMLIGI}.png"
        hedef.parent.mkdir(parents=True, exist_ok=True)
        ana_goruntu.resize((boyut, boyut), Image.Resampling.LANCZOS).save(hedef, optimize=True)
    return ana_goruntu


def icoyu_yaz(ana_goruntu: Image.Image) -> None:
    ana_goruntu.save(IKON_DIZINI / "spcad.ico", format="ICO", sizes=ICO_BOYUTLARI)


def icns_yaz(kaynak: Path) -> None:
    parcalar = bytearray()
    for boyut, tur in ICNS_TURLERI.items():
        yol = IKON_DIZINI / f".icns-{boyut}.png"
        kare_yukle(kaynak, boyut).save(yol, optimize=True)
        veri = yol.read_bytes()
        yol.unlink()
        parcalar += tur + struct.pack(">I", len(veri) + 8) + veri
    govde = b"icns" + struct.pack(">I", len(parcalar) + 8) + bytes(parcalar)
    (IKON_DIZINI / "spcad.icns").write_bytes(govde)


def ana() -> int:
    if not KAYNAK.is_file():
        print(f"kaynak logo bulunamadı: {KAYNAK}", file=sys.stderr)
        return 1
    ana_goruntu = pngleri_yaz(KAYNAK)
    icoyu_yaz(ana_goruntu)
    icns_yaz(KAYNAK)
    print(f"ikonlar yazıldı: {IKON_DIZINI}")
    return 0


if __name__ == "__main__":
    raise SystemExit(ana())
