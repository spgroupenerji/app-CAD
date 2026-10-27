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
## Lucide eşlemesi (P4 dilimi)

- `ikonlar::cad_esleme` — genel amaçlı 35 CAD ikonu Lucide glyph'ine bağlandı:
  New/Open/Save/SaveAs, Undo/Redo, Plot/Publish/PageSetup/Import/Export/Attach/Preview,
  ZoomWindow/ZoomExtents/Pan, Layers/Help/Table, Erase, Lock/Unlock, Snowflake,
  Grid/Snap/Ortho/Polar/Osnap/OTrack, Lineweight/Transparency/Isodraft/DynInput/QuickProps,
  Gear/Annotation/Workspace/Plus/Close/Search, Measure/Distance.
- `simgeler::paint` eşlenen ikonlarda doğrudan Lucide glyph'i çizer; CAD semantiği taşıyan
  106 ikon (Line, Trim, Fillet, Hatch, Dimension…) kendi çiziminde kalır.
- Test: eşlenen her ikonun Lucide'de bulunduğu (`cad_karsiliklari_lucide_de_bulunuyor`).
- Şerit görüntüsü: [gercek-p4.png](gorseller/wp4/gercek-p4.png).
- Katalog: [ikon-katalogu.png](gorseller/wp4/ikon-katalogu.png) — Lucide ve CAD ikonları yan yana.
## Ek eşleme (P4)

- Rectangle → `rectangle-horizontal`, Polygon → `pentagon`, RevCloud → `cloud`,
  Text/MText → `type`, BlockCreate → `package`, Insert → `square-plus`,
  BlockEdit → `square-pen`, Hatch → `paint-bucket`, Point → `dot`, Spline → `spline`.
- `ArayuzIkonu` 53 role çıktı; adlar tekil testi geçiyor; katalog yenilendi.
## İlk çizim partisi (P4)

- Point: artı kolları 5–19 → **2–22**; merkez nokta korundu.
- Ray: 5–19 → **2–22**; XLine zaten 1–23.
- Donut: dış yarıçap 8 → **9,5**, iç 4,5 → **5,5**.
- Boundary: 4–20 → **2–22**; iç daire 6 → 7.
- Region: poligon **2–22** aralığına genişletildi.
- Wipeout: 3–16 → **2–18**.
- Table: 3–21 → **2–22**; satır çizgileri 7/12/17.
- Katalog: [ikon-katalogu.png](gorseller/wp4/ikon-katalogu.png).
## Stroke normalizasyonu (P4)

- Kalınlık formülü `genişlik/16 clamp(1,0–1,6)` → `genişlik/14 clamp(1,1–1,6)`:
  20 px ikonda ~**1,43 px**, 16 px'te ~1,14 px; shadcn/Lucide 1,5 px hedefine yaklaştı.
- Katalog yenilendi: [ikon-katalogu.png](gorseller/wp4/ikon-katalogu.png).
