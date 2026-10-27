# WP4 — ikon kataloğu ve P2 widget durumları

> **Durum:** katalog/denetim altyapısı tamam; ikonların yeniden çizimi açık · **Tarih:** 2026-10-27

## Eklenenler

- `simgeler::Icon::ALL` — **141 CAD ikonu** (enum'dan üretildi); katalog ve testler bu listeyi gezer.
- `tests/ikon_katalogu.rs`:
  - `her_cad_ikonu_cizilebiliyor` — 141 ikonun tamamı boş sahneye çizilir (panik yok).
  - `katalog_sayfasi` (`#[ignore]`) — 10 sütunlu görsel denetim sayfası üretir:
    [ikon-katalogu.png](gorseller/wp4/ikon-katalogu.png).
- `tema::visuals` (P2): widget kenar çizgileri (pasif: border, hover/aktif: selection), `expansion = 0`
  (sakin durum geçişi), `selection.stroke`, `hyperlink_color`, `warn_fg_color`.

## Katalog bulguları (P4 için girdi)

- Optik boyut farkları belirgin: New/Open/Save gibi sayfa ikonları 24 px'i doldururken
  Spline/Donut/Point gibi simgeler küçük kalıyor; yeniden çizimde 20 px canlı alan + 2 px güvenli boşluk hedeflenmeli.
- Stroke zaten `(genişlik/16).clamp(1,0–1,6)` ile normalize; sorun çizgi kalınlığı değil, geometri ölçeği.
- Etiketler kataloğda taşıyor (ör. "DimBaseline"); yeniden çizim sırasında ikon adları kısaltılmalı.

## Sıradaki (P4)

1. Şeritteki ~40 ikonun 24×24 / 1,5 px kuralıyla yeniden çizimi (Line, Polyline, Circle, Arc, Move, Copy, Rotate, Trim, Hatch, MText, Dimension, Layer, Block, Zoom, Pan…).
2. Kalan simgeler için aynı kural; her partide katalog sayfası + gözle onay.
3. Katalog testi görsel regresyon kapısına bağlanır (P5 golden).
