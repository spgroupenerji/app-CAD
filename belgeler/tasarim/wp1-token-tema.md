# WP1 — Token ve tema altyapısı

> **Durum:** tamamlandı (commit yok) · **Tarih:** 2026-10-27 · **Önceki:** [wp0-denetim.md](wp0-denetim.md) · **Sonraki:** WP2 (bileşen kiti)

## 1. Ne değişti

| Dosya | Değişiklik |
|---|---|
| `tema.rs` | **+326 / −67**: `Tokens::DARK`/`LIGHT` artık `Semantik` sözlüğünden türetiliyor; ölçek tokenları; `visuals()`/`apply()` bunları kullanıyor |
| `animasyon.rs` | Animasyon süresi `HAREKET_ORTA` (150 ms) tokenından; test güncellendi |
| `clippy.toml` | `disallowed-methods`: `Color32::from_rgb`, `Color32::from_rgba_unmultiplied_const` |
| 11 eski dosya | `#![allow(clippy::disallowed_methods)]` — ham renk teknik borcu envanteri (WP2/WP3'te kaldırılacak) |

### Ölçek tokenları (`tema.rs`)

- Tipografi: `YAZI_MIKRO 10.5`, `YAZI_KUCUK 11.5`, `YAZI_GOVDE 12.5`, `YAZI_VURGU 14`, `YAZI_BASLIK 17`; `small()/body()/mono()` ve şerit etiketleri bunlardan.
- Boşluk: `BOSLUK_1..6 = 4/8/12/16/24/32`; `SATIR_YUKSEKLIK = 28`; `item_spacing = (8,6)`, `button_padding = (10,5)`, `interact_size.y = 24`.
- Yarıçap: `KOSE_KUCUK 4`, `KOSE_ORTA 6`, `KOSE_BUYUK 8`; widget 6, pencere 8, menü 6.
- Hareket: `HAREKET_HIZLI 0.09`, `HAREKET_ORTA 0.15`, `HAREKET_YAVAS 0.22` sn.
- Gölge: popup `offset [0,8]`, `blur 24`, koyu alfa 140 / açık 50.

### Eşleme kararları

- `chrome ← muted`, `chrome_dark ← secondary`, `panel ← card`, `control ← secondary`, `control_hover ← accent`, `text ← foreground`, `text_dim ← muted_foreground`, `icon ← card_foreground`, `accent ← primary`, `selection/hover/icon_accent ← ring`, `field/cmd_bg/list_bg ← input`, `select_bg/list_row ← accent`, `warn ← warning`.
- Çizim alanı (tuval, ızgara, eksen, snap, grip, crosshair) ayrı CAD tokenları olarak kaldı; tema değişse de çizim rengi değişmez.

## 2. Doğrulama

| Komut | Sonuç |
|---|---|
| `cargo test -p spcad-ui-egui --lib` | **201 passed; 0 failed** |
| `cargo test -p spcad-ui-egui --lib semantik` | 11 passed |
| `cargo test -p spcad-ui-egui --lib egui_stops` | 1 passed (animasyon tokenı) |
| `cargo clippy -p spcad-ui-egui --lib` | 0 uyarı |
| `rustfmt --check tema.rs animasyon.rs` | 0 |
| `cargo build -p spcad` | başarılı (ilk 4 dk 11 sn; sonraki 18 sn) |

## 3. Gerçek uygulama görüntüleri (yeni tokenlarla)

- [Çizim](gorseller/wp1/gercek-01-cizim.png) · [Paletler](gorseller/wp1/gercek-02-paletler.png) · [Diyalog](gorseller/wp1/gercek-03-diyalog.png) — 1600×1000 @2x, temiz çalışma alanı, araç rafı 12 araçla dolu.
- Hedef tasarım (WP0): [sonra-01](gorseller/wp0/sonra-01-cizim.png) · [sonra-02](gorseller/wp0/sonra-02-paletler.png) · [sonra-03](gorseller/wp0/sonra-03-diyalog.png).

**Fark:** renk/yüzey/tipografi/ölçü altyapısı gerçek uygulamaya geçti; tuval daha koyu, panel ve şerit tonları yeni paletten, köşeler ve boşluklar tokenlardan. Buna karşılık panel başlıkları, şerit yerleşimi, komut güvertesi ve diyalog iskeleti hâlâ eski çizim kodunu kullanıyor — bunlar WP2/WP3'ün işi.

## 4. Bilinen sınırlar

- Açık tema görsel olarak doğrulanmadı (yalnız koyu tema ekran görüntüsü alındı).
- Yalnız Windows'ta derlendi/çalıştırıldı; macOS/Linux doğrulaması CI'ya bağlı.
- `ui.set` ile komut satırı görünürlüğü açılsa da taze profilde komut giriş satırı çizilmiyor; komut güvertesi WP3'te yeniden yazılacak.

## 5. WP2 (öneri)

Bileşen kiti: panel başlığı/satırı, düğme, alan, sekme, tooltip, tablo satırı ve diyalog iskeletini tokenlara geçirmek; şeridi 84 px kompakt gövdeye indirmek; `egui_kittest` golden (dark/light) eklemek; ham renk allow listesini dosya dosya kaldırmak. Tahmini 40–60 ajan-saat.
