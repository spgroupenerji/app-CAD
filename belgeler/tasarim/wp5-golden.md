# WP5 — golden altyapısı

> **Durum:** referans üretimi hazır; CI karşılaştırması açık · **Tarih:** 2026-10-27

## Ne var

- `tests/gorunum_altin.rs` (`#[ignore]`): sabit `CadApp` durumunda üç referans PNG üretir:
  - `cizim` — varsayılan krom, boş çizim
  - `diyalog` — Options açık
  - `katmanlar` — Katman Özellikleri Yöneticisi açık
- Çıktı: `belgeler/gorseller/altin/{cizim,diyalog,katmanlar}.png` (1600×1000, wgpu headless).
- Üretim komutu: `cargo test -p spcad-ui-egui --test gorunum_altin -- --ignored`.

## Neden karşılaştırma testi değil

Golden karşılaştırması platformlar arası font/raster farklarına duyarlıdır (macOS Metal / Linux wgpu).
Bu yüzden önce **referans üretimi** kuruldu; CI'da karşılaştırma için tek platform seçilip toleranslı
diff (piksel oranı) eklenmeli. Bu, P5'in kalan son adımı.

## Sıradaki

1. Tek CI platformunda (Linux wgpu) referans-vs-çıktı diff eşiği (ör. %0,3 fark) ekle.
2. 15 dil × 3 genişlik için ek referanslar üret.
3. Ham renk `allow` listesini dosya dosya kaldır (P2/P3 bileşenleri tokenlara geçtikçe).
