# WP2-P1 — Lucide ikon modülü (durum)

> **Durum:** çekirdek tamamlandı · **Tarih:** 2026-10-27 · **Sonraki:** P2 bileşen kiti

## Ne yapıldı

- `kitapliklar/arayuz-egui/src/ikonlar/mod.rs`:
  - `ArayuzIkonu` — 43 genel arayüz ikonu; her biri bir Lucide adına bağlı (`ad()`).
  - `glyph()` — `iconflow` Lucide setinden codepoint; ilk çözümlemede `OnceLock<HashMap>` ile önbellek.
  - `fontlari_ekle(&mut FontDefinitions)` — Lucide TTF'ini ilk kareden önce font tanımlarına ekler.
  - `ciz(painter, merkez, ikon, boyut, renk)` — `Painter` tabanlı yüzeyler için ortalanmış glyph.
  - Testler: her ikonun Lucide'de bulunduğu + adların tekil olduğu (2 test).
- `tema::install_fonts`: ikon fontu kurulumu ilk kareden önce.
- `cerceve::status_bar`: 14 durum çubuğu anahtarı el çizimi ikonlardan Lucide'e geçti
  (grid-3x3, magnet, square, compass, box, crosshair, circle-dot, weight, blend,
  text-cursor-input, sliders-horizontal, eye, layout-dashboard, settings).
- `Cargo.toml`: `iconflow 2.1` (`pack-lucide`) normal bağımlılık; spike testi `#[ignore]`.
- P0 spike: `tests/ikon_spike.rs` (elle çalıştırılır) + `belgeler/gorseller/wp2/ikon-spike-*.png`.

## Doğrulama

| Komut | Sonuç |
|---|---|
| `cargo test -p spcad-ui-egui --lib ikonlar` | 2 passed |
| `cargo clippy -p spcad-ui-egui --lib` | 0 uyarı |
| `cargo build -p spcad` | başarılı |
| Ekran görüntüsü | `belgeler/gorseller/wp2/gercek-p1.png` (durum çubuğu Lucide) |

## Kalanlar (P2–P5)

- P2: bileşen kiti (panel başlığı/satır, düğme, alan, sekme, tooltip, diyalog) + denge metrikleri.
- P3: kompakt şerit, komut güvertesi, durum çubuğu düzeni, araç rafı, ViewCube.
- P4: 136 CAD ikonunun 24/1,5 kuralıyla yeniden çizimi (önce şeritteki ~40).
- P5: golden (dark/light × 15 dil × 3 genişlik), a11y, ham renk allow listesinin kaldırılması.
