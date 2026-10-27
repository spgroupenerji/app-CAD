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
## Ek dilim — panel sekmeleri

- `kenetleme::show`: `DockStyle.tab_height = 34`, `gap = 1`, `tab_background = chrome_dark` (secondary) —
  Layers/Properties başlıkları artık panel yüzeyinden ayrışıyor.
- Şerit 84 px denemesi **geri alındı**: mevcut yerleşim 84 px'te etiketleri üst üste bindiriyor
  (Multiline Text / Layer Properties); kompakt şerit P3'te yerleşim algoritmasıyla birlikte yapılmalı.
- Görüntü: [gercek-p3.png](gorseller/wp3/gercek-p3.png).
## Komut çubuğu (P3 dilimi)

- Giriş alanı artık `t.field` dolgusu + 1 px `cmd_border` + 4 px köşe ile çerçeveli.
- Boşken "Command:" yanında "Type a command, or press Ctrl+K" ipucu (uk/tr çevirileri eklendi).
- Görüntü: [gercek-p3b.png](gorseller/wp3/gercek-p3b.png).
## ViewCube (P3 dilimi)

- Halka 58 → 50 px, kalınlık 9 → 1,5 px; ikinci halka 0,5 px soluk.
- Yüz 44 → 42 px, 6 px köşe; pasif `control`, hover `accent`; yazı 13 → 11 px.
- Pusula harfleri 17 → 10,5 px ve `text_faint`; Home 20 → 18 px.
- WCS pill 56 → 52 px, merkeze 26 → 22 px.
- Görüntü: [gercek-viewcube.png](gorseller/wp4/gercek-viewcube.png). Test geometrisi yeni halkaya göre güncellendi.
## Şerit sekmeleri (P3 dilimi)

- Aktif sekme: üst köşeleri 6 px yuvarlak `panel` zemini + altında 2 px `accent` çizgisi.
- Pasif hover: 4 px yuvarlak, %35 `control_hover`; sekme yazısı `body()` (12,5 px).
- Hızlı erişim ikonları (New/Open/Save/Undo/Redo/Plot) artık Lucide glyph'leri (P4 eşlemesi).
- Görüntü: [gercek-sekmeler.png](gorseller/wp4/gercek-sekmeler.png).
## Dosya sekmeleri (P3 dilimi)

- Aktif dosya sekmesi: üst köşeler 6 px yuvarlak `tab_active` zemini + 2 px `accent` alt çizgisi.
- Pasif sekmeler düz; hover `chrome`; kapatma ikonu hâlâ Lucide `close`.
- Görüntü: [gercek-dosyasekmeleri.png](gorseller/wp4/gercek-dosyasekmeleri.png).
## Açık tema doğrulaması (P5 dilimi)

- `ui.set {"theme":"light"}` ile açık tema gerçek uygulamada çekildi: [gercek-acik-tema.png](gorseller/wp4/gercek-acik-tema.png).
- Krom/paneller açık, tuval tasarım gereği koyu kalıyor; Lucide ve CAD ikonları okunaklı; komut çubuğu alanı çerçeveli.
- Koyu temada doğrulanan token seti açık temada da tutarlı; ek düzeltme gerekmedi.
## Diyalog iç yüzeyi (P2 dilimi)

- Options içerik alanı `chrome_dark` dolgu ile pencere yüzeyinden ayrıldı (18/14 iç boşluk).
- Aktif sekmenin üstünde 2 px `accent` çizgisi; pasif sekmelerde alt kenarlık korunuyor.
- Görüntü: [gercek-diyalog2.png](gorseller/wp4/gercek-diyalog2.png).
## Komut geçmişi paneli (P3 dilimi)

- Genişletilmiş geçmiş `list_bg` dolgu + `cmd_border` çerçeve ile ayrıldı.
- Satırlar 6 px iç boşluklu; hover `list_row`; en yeni satır `text`, eskiler `text_dim`.
- Görüntü: [gercek-komutgecmisi.png](gorseller/wp4/gercek-komutgecmisi.png).
## Kompakt şerit (P3 kapanış dilimi)

- `GOVDE_Y` 94 → **84**, `TUR_H` 18 → **16**: toplam şerit 124 → **114 px**.
- Etiketler 9,5–11,5 px ölçeğinde kaldı; 1600 px'te taşma/çakışma yok (görüntü doğrulandı).
- Şerit testleri (11) yeşil; build başarılı.
- Görüntü: [gercek-kompakt-serit.png](gorseller/wp4/gercek-kompakt-serit.png).
