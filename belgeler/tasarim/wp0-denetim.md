# WP0 — Arayüz denetimi, token prototipi ve önce/sonra görselleri

> **Durum:** tamamlandı (commit yok) · **Tarih:** 2026-10-27 · **Kapsam:** tasarım sistemi yenilemesi (C), WP0 · **Sonraki:** WP1 (token + tema)

## 1. Teslimatlar

| # | Teslimat | Konum |
|---|---|---|
| 1 | Semantik renk sözlüğü + 11 WCAG testi | `kitapliklar/arayuz-egui/src/tema.rs` (+231 satır) |
| 2 | "Önce" ekran görüntüleri (gerçek uygulama, kontrol kanalı `ui.screenshot`) | `belgeler/gorseller/wp0/once-0{1,2,3}-*.png` |
| 3 | "Sonra" tasarım önizlemeleri (HTML/CSS, 1600×1000 @2x) | `belgeler/gorseller/wp0/sonra-0{1,2,3}-*.png` |
| 4 | Önizleme kaynakları (düzenlenebilir) | `belgeler/tasarim/onizleme/` |

Üç ekran: **çizim**, **paletler açık** (QuickCalc + Tool Palette + bağlam menüsü + komut tamamlama), **Options diyalogu**.

## 2. Semantik token prototipi

- 19 rol: `background/foreground/card/card_foreground/popover/primary/primary_foreground/secondary/secondary_foreground/muted/muted_foreground/accent/accent_foreground/border/input/ring/danger/warning/success`.
- `Semantik::DARK`, `Semantik::LIGHT`, `Semantik::of(Theme)`; mevcut `Tokens` API'si değişmedi (yalnızca ekleme).
- Testler: WCAG 2.1 doğrusal/parlaklık/kontrast yardımcıları + 11 test (AAA 7,0 · AA 4,5 · bileşen 3,0 · kenarlık 1,2 · yüzey kademesi).

### Ölçülen kontrastlar (yeni palet)

| Çift | Koyu | Açık | Eşik |
|---|---:|---:|---|
| foreground / background | **16,03** | 16,75 | ≥ 7,0 (AAA) |
| card_foreground / card | 13,09 | 17,31 | ≥ 7,0 |
| muted_foreground / background | 9,65 | 5,40 | ≥ 4,5 |
| primary_foreground / primary | 5,17 | 5,17 | ≥ 4,5 |
| danger / background | 5,73 | 5,38 | ≥ 4,5 |
| warning / background | 7,61 | 5,84 | ≥ 4,5 |
| ring / background | 6,93 | 4,77 | ≥ 3,0 |
| border / background | 1,61 | 1,23 | ≥ 1,2 |
| yüzey adımı (card−bg / popover−card) | 0,0086 / 0,0081 | 0,0808 / 0,0 (popover = card) | ölçülen ≥ 0,006 |

### Mevcut arayüzdeki karşılıkları (WP0 denetimi)

| Sorun | Ölçüm |
|---|---|
| Yüzeyler ayrışmıyor | panel/chrome **1,29:1**, control/panel **1,19:1**, canvas/panel **1,55:1** |
| Soluk metin AA altı | text_faint/panel **3,27:1** |
| Aksan üstü metin AA altı | beyaz/aksan **3,33:1**, metin/aksan **2,63:1** |
| Tipografi ölçeği yok | 9,5 · 10,5 · 11,5 · 12,5 · 13 · 14 · 17 · 18 px (8 farklı boyut); şerit etiketi 9,5–11,5 |
| Boşluk ritmi yok | `item_spacing 6/4`, `button_padding 6/3`, `interact_size 20`; sabitler 16/18/20/21/26/28/30/40/56/94… |
| İkon tutarsızlığı | 995 satır elle çizim; `Stroke::new(self.w)`, `2,2` gibi değişken kalınlık |
| Hareket tokenı yok | yalnız egui varsayılan animasyonu + kaydırma; reduce-motion desteği var |

## 3. Doğrulama (komut + sonuç)

- `cargo test -p spcad-ui-egui --lib semantik` → **11 passed; 0 failed** (190 filtered).
- `cargo check -p spcad-ui-egui` → hatasız.
- `cargo clippy -p spcad-ui-egui --lib` → 0 uyarı, `Finished`.
- `rustfmt --edition 2024 --check kitapliklar/arayuz-egui/src/tema.rs` → 0.

## 4. Tasarım yönü (önizlemelerde gösterilen)

- Üç yüzey + overlay: zemin `#0C0F13`, kart `#171F29`, popover `#1F2935`, tuval `#06080B`.
- Tek aksan `#2563EB` (dolum) / `#4C8DFF` (vurgu); hairline `#2C3846`; iki kademe gölge.
- Kompakt şerit (94 px yerine 84 px gövde; 2×2 küçük düğme ızgarası), belge sekmelerinde aksan alt çizgisi, komut güvertesi (geçmiş + tamamlama + anahtar sözcük çipleri), gruplu durum çubuğu, sade ViewCube.
- Diyalog: 748 px, başlık/meta/sekme/gövde/footer hiyerarşisi, kart içi form grupları, birincil/ikincil düğmeler.

## 5. WP1 kapsamı ve kabul kapıları

1. `Semantik` sözlüğünü `Tokens`/`visuals()` üretimine bağla (tek kaynak; `tema.rs`).
2. Tipografi ölçeği (11/12/13/15/18) + satır yüksekliği; boşluk ölçeği (4/8/12/16/24/32); yarıçap (4/6/8); hareket (90/150/220 ms) tokenları.
3. Yüksek trafikli bileşenleri (panel başlığı, satır, düğme, alan, sekme, komut satırı) tokenlara geçir.
4. Governance: `clippy.toml` `disallowed-methods` ile `tema.rs` dışında ham `Color32::from_rgb` yasağı.
5. Kabul: kontrast testleri yeşil; `egui_kittest` golden (dark/light); üç ekranın gerçek uygulama görüntüsü önizlemelerle eşleşir.

**Açık karar:** "Sonra" görselleri tasarım önizlemesidir; WP1'de gerçek uygulamada uygulanacak. Onay gelmeden WP1 koduna geçilmedi.
