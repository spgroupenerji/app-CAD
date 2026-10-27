# WP2-P0 — ikon spike sonucu

> **Durum:** tamamlandı (spike; üretim kodu değişmedi) · **Tarih:** 2026-10-27 · **Karar:** Tier 1 = iconflow/Lucide fontu, Tier 2 = kendi CAD vektör seti

## 1. Yöntem

- Araç: `egui_kittest` + `wgpu` headless renderer; 1500×420 sahne; 1× / 1,25× / 1,5× / 2× `pixels_per_point`.
- A: `iconflow 2.1` `pack-lucide` (Lucide 1.665 ikon, 0,41 MiB, MIT/ISC), 14/16/18/20 px.
- C: mevcut `simgeler.rs` (136 çizim) 20 px.
- Çıktı: `belgeler/gorseller/wp2/ikon-spike-{1x,1_25x,1_5x,2x}.png`.
- Spike kodu: `kitapliklar/arayuz-egui/tests/ikon_spike.rs` + dev-dependency `iconflow` (geçici; P1'de üretime taşınacak ya da kaldırılacak).

## 2. Bulgular

| # | Bulgu | Kanıt |
|---|---|---|
| 1 | Lucide fontu 14–20 px'te tutarlı, 2×'te cam gibi keskin | `ikon-spike-2x.png` |
| 2 | 1×'te ince (1,5 px) stroke'lar hafif soluk; egui AA kabul edilebilir, ama ikon fontu için boyut alt sınırı ~14 px | `ikon-spike-1x.png` |
| 3 | Mevcut `simgeler.rs` stroke 1,0–2,2 arası değişiyor, optik hizalama yok; Lucide satırıyla yan yana “cılız/kaçık” duruyor | 1×/2× sayfalarının alt satırı |
| 4 | Lucide adları CAD adlarıyla birebir değil; ör. `line` yok (`pen-line`, `minus`), eşleme tablosu + test şart | `IKON line -> IconNotFound` |
| 5 | Font ailesi bağlama zamanlaması kritik: `FontFamily::Name("Lucide Regular")` ilk kare sonrası kurulunca kittest'te bağlanmadı; üretimde fontlar ilk kareden önce kurulmalı (fallback kuyruğu ile de çalıştı) | spike paniği + düzeltme |
| 6 | Paket maliyeti küçük: yalnız Lucide TTF 0,41 MiB; pack başına feature, ikon başına alt küme yok | iconflow README |
| 7 | B seçeneği (craft-ui `SvgIconSet` + egui_extras/resvg) ölçülmedi: A yeterli kalitede; resvg binary/açılış maliyeti ve Image layout yükü nedeniyle yalnız A başarısız olursa açılacak | — |

## 3. Karar

- **Tier 1 (genel arayüz): iconflow Lucide fontu.** Neden: shadcn ile birebir set, DPI'dan bağımsız keskinlik, tek TTF, düşük kod yüzeyi, MIT/ISC.
- **Tier 2 (CAD komutları): kendi vektör setimiz.** 136 ikon, 24×24 grid / 1,5 px stroke / yuvarlak uç / optik merkez kuralıyla yeniden çizilecek; çok tonlu semantik korunacak.
- **B planı:** yalnız font 1×'te yetersiz kalırsa craft-ui SVG + egui_extras (resvg) değerlendirilecek.

## 4. Sonraki adımlar

1. **P1:** `ikonlar/` modülü — `ArayuzIkonu` ↔ Lucide adı eşlemesi (testli), boyut tokenları (14/16/18/20), dikey hizalama yardımcısı, font kurulumu ilk kareden önce.
2. **P2:** bileşen kiti (panel başlığı/satır, düğme, alan, sekme, tooltip, diyalog) + denge metrikleri.
3. **P3:** krom (kompakt şerit, komut güvertesi, durum çubuğu, araç rafı).
4. **P4:** CAD ikonlarının yeniden çizimi (önce şeritteki ~40).
5. **P5:** golden (dark/light × 15 dil × 3 genişlik), a11y, ham renk allow listesinin kaldırılması.

## 5. Kaynaklar

- [iconflow](https://crates.io/crates/iconflow) · [README (paketler/lisanslar)](https://github.com/FerrisMind/iconflow)
- [Lucide](https://lucide.dev) · [lucide-icons (Rust)](https://crates.io/crates/lucide-icons)
- Yerel: `craft-ui` `SvgIconSet` + `egui_extras` SVG loader (B planı)
