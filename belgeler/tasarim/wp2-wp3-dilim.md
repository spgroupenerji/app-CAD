# WP2/WP3 ilk dilim — ölçü ve durum çubuğu

> **Durum:** kısmi (P2/P3'ün ilk dilimi) · **Tarih:** 2026-10-27 · **Sonraki:** bileşen kiti, şerit, komut güvertesi, CAD ikonları

## Bu dilimde değişen

| Alan | Önce | Sonra |
|---|---|---|
| Durum çubuğu yüksekliği | 26 px | **28 px** |
| Durum çubuğu anahtarları | 20 px el çizimi ikon | **22 px Lucide** (P1) + 4 px köşe, %30 ton |
| Komut satırı satır yüksekliği | 16 px | **18 px** |
| Komut giriş barı | 26 px | **34 px** |
| Options diyalogu | 720 px / sekme 30 / iç 14-10 | **760 px / sekme 36 / iç 18-14** |
| Araç rafı satır/başlık | 28 px | **30 px** |
| Ölçek testleri | — | 4 pt ızgara + artan tipografi/yarıçap/hareket (P5 kapısı) |

## Doğrulama

- `cargo test -p spcad-ui-egui --lib`: **205 passed, 0 failed**.
- `cargo clippy -p spcad-ui-egui --lib`: 0 uyarı; `rustfmt`: temiz.
- `cargo build -p spcad`: başarılı.
- Ekran görüntüleri: [çizim](gorseller/wp3/gercek-p2.png) · [Options](gorseller/wp3/gercek-p2-diyalog.png).

## Kalanlar

- **P2 tam bileşen kiti:** panel başlığı/satırı, düğme, alan, sekme, tooltip ve diyalog iskeleti
  (büyük kısmı `craft-ui` docking çiziminde; upstream rev yükseltmesi veya yerel sarmalayıcı gerekir).
- **P3:** şerit 84 px kompakt gövde, komut güvertesi yeniden yazımı, durum çubuğu gruplama, ViewCube.
- **P4:** 136 CAD ikonunun 24×24 / 1,5 px kuralıyla yeniden çizimi.
- **P5:** dark/light × 15 dil × 3 genişlik golden testleri, a11y, ham renk allow listesinin kaldırılması.
