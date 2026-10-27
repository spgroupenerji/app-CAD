# GPUI geçişi ve SPCAD arayüz tasarım dili

> **Durum:** **askıda** — B, 2026-10-27'de askıya alındı; yerine C (egui tasarım sistemi yenilemesi) öneriliyor: [arayuz-tasarim-sistemleri-arastirmasi.md](arayuz-tasarim-sistemleri-arastirmasi.md). Aşağıdaki B planı tarihsel kayıt olarak duruyor. · **Tarih:** 2026-10-27 · **Kapsam:** masaüstü (macOS/Windows/Linux); web ve FreeBSD koşullu · **Kanıt:** context7 + resmî gpui-kit dokümanları + crates.io (26–27 Eki 2026)

Bu belge, mevcut egui ön yüzünün GPUI Kit tabanlı tek hedef ön yüze geçişinin tasarımını ve kabul ölçütlerini tanımlar. Kod yazımı Faz 0 (ölçüm) kapısı geçildikten sonra başlar.

## 1. Karar

| Karar | Değer |
|---|---|
| Hedef çerçeve | `gpui-kit` 0.8.1 (GPUI + gpui-base + gpui-component + varsayılan ikon seti) |
| Geçiş biçimi | **B — tek hedef, tek cutover.** Masaüstünde egui'ye yeni özellik/düzeltme gitmez; cutover'da masaüstü egui yolu silinir |
| Lisans | gpui-kit, gpui-pre, gpui-fast: **Apache-2.0** (doğrulandı); Lucide ISC; Inter/JetBrains Mono OFL |
| Gerekçe | 75+ stilli bileşen, dock/tablo/editör, AccessKit, retained mode, 120 FPS hedefi, tek bağımlılık |

## 2. Doğrulanmış platform ve sürüm matrisi

| Hedef | Durum | Ayrıntı |
|---|---|---|
| macOS | ✅ 15+ | Metal; `font-kit` özelliği gpui-kit tarafından açılır |
| Windows | ✅ 10+ | MSVC + VS 2022 C++ iş yükü + CMake; GNU araç zinciri desteklenmez |
| Linux | ✅ | Ubuntu 24.04 doğrulanmış; **Vulkan sürücüsü**, fontconfig, Wayland/X11, webkit2gtk-4.1, openssl, zstd, ALSA |
| WebAssembly | ⚠️ vitrin | “showcase-only”: nightly araç zinciri + sabit `wasm-bindgen`; tam uygulama dağıtımı, erişilebilirlik, girdi, açılış maliyeti **doğrulanmalı** |
| FreeBSD | ⚠️ koşullu | `gpui-pre-platform`, `gpui-pre-linux`'u `target_os=\"freebsd\"` için de seçiyor → derleme yolu var; pencere/GPU **Faz 0'da doğrulanmalı** |
| iOS / Android | ❌ | Mobile deneysel ve iOS odaklı; topluluk `gpui-mobile` fork'u git bağımlılığı, GPUI =0.3.4 pinli; “mobile birincil hedef değil”, “genel mobil destek garantisi yok”; Android rehber kapsamı dışında |

**Sürüm pinleri:** `gpui-kit =0.8.1`, `gpui-component =0.8.1`, `gpui-base =0.8.1`, `gpui-kit-assets =0.8.1`, `gpui-pre =0.3.10` (gpui-kit 0.8.1'in tam pinlediği snapshot), `gpui-fast =0.1.12` (opsiyonel, deneysel). Rust **≥ 1.92** (çalışma alanı `rust-version` 1.90 → yükseltilecek).

## 3. Kapsam ve kapsam dışı

- **Kapsam:** masaüstü ön yüzün tamamı (menü, şerit, araç rafı, paletler, komut satırı, durum çubuğu, ViewCube, tüm diyaloglar, tuval), JSON denetim sözleşmesi, i18n, erişilebilirlik, paketleme.
- **Kapsam dışı:** Android/iOS uygulaması (ayrı proje: Rust çekirdeği L0–L4 hazır; native UI veya ileride `gpui-mobile`), GPUI web dağıtımı (vitrin seviyesi).
- **Koşullu:** FreeBSD (Faz 0 sonucuna göre GPUI veya egui adaptörü), web (aşağıdaki “imkânsız üçlü”).

### İmkânsız üçlü

Şu üçü aynı anda olamaz: **(1)** tek ön yüz, **(2)** web + FreeBSD'nin korunması, **(3)** sıfır ek bakım.

| Seçenek | Tek ön yüz | Web/FreeBSD | Maliyet |
|---|---|---|---|
| **B1** GPUI her yerde | Evet | GPUI-WASM (vitrin) / FreeBSD doğrulanır | Web riski yüksek |
| **B2** Masaüstü GPUI, web/FreeBSD egui adaptörü | Masaüstünde evet | egui ince adaptör (yalnız derleme düzeltmesi, özellik yok) | Sınırlı çift bakım |
| **B3** Yalnız GPUI | Evet | Web/FreeBSD emekli | Platform kaybı |

**Öneri: B2.** Masaüstü tek ön yüz olur, platform kaybı olmaz; bedeli, web/FreeBSD için dondurulmuş egui adaptörünün bakımıdır.

## 4. Mimari

```text
uygulamalar/spcad-gpui          pencere, platform, oturum başlatma
kitapliklar/arayuz-gpui    L5   ön yüz
  tasarim/     tokenler · tema · tipografi · hareket · ikonlar
  kabuk/       kok · baslik · menu · serit · sekmeler · komut_guvertesi · durum_cubugu
  paneller/    araclar · katmanlar · ozellikler · xref · quickcalc · gecmis
  diyaloglar/  ac · secenekler · tarama · mtext · yazdir · katman_yoneticisi
  tuval/       ressam · etkilesim · osnap · grip · secim · gorunum_kupu
  kopru/       oturum · komut · denetim · kalicilik
kitapliklar/yerellestirme  L5   15 dilli TSV kataloğu (iki ön yüz ortak)
kitapliklar/arayuz-egui    L5   dondurulmuş: web/FreeBSD adaptörü
L0–L4                           DEĞİŞMEZ
```

**Sınırlar:** `arayuz-gpui` yalnız `engine/doc/render/color/geom/fonts` + `gpui-kit`'e bağlıdır; `arayuz-egui`'ye asla bağlanmaz. `xtask layers` gpui'yi yalnızca L5'te kabul edecek şekilde güncellenir. Motor API'si değişmez; zorunlu kalırsa yalnızca eklemeli (additive) ve L4'te incelenir.

**Oturum köprüsü:** Ön yüz, `Session`'ı polling ile okur (16 ms'lik birleştirme), `drawing.uid/revision` ve UI durumu değiştiğinde `cx.notify()` üretir; denetim kanalı istekleri aynı foreground görevinde boşaltılır. Sürükleme/seçim sırasında olay başına bildirim.

**Tuval ressamı:** `spcad_render::DisplayList` (`Polyline`, `Tris`, `Mask`, `Point`, `Infinite`) GPUI ilkellerine çevrilir: renk/saydamlık gruplu `paint_path`, 1 px kıl payı çizgiler piksel hizalı `paint_quad`, maskeler sıralı arka plan dolgusu, sonsuz çizgiler görünüm kırpılarak. Metin zaten üçgenlenmiş gelir; ayrı metin şekillendirme gerekmez. Geometri (belge revizyonu, görünüm, LOD, tema) anahtarıyla önbelleklenir; her karede yalnız boyama yapılır.

**Denetim sözleşmesi:** `denetim.rs`'teki ~30 JSON metot aynı ad ve anlamla yeniden yazılır (`ui.inspect`, `ui.elements`, `ui.pointer`, `ui.screenshot`, `engine.execute`, …). MCP ajan betikleri değişmeden geçmelidir; iki ön yüz için ortak sözleşme testleri.

**Performans bütçesi:** 250.000 çizgi segmenti, 1600×1000, pan/zoom p95 kare ≤ 16,7 ms; giriş gecikmesi ≤ 1 kare. Ölçüm geçmezse Plan B: `gpui-pre`'ye Apache-2.0 uyumlu özel wgpu katmanı yaması (ölçülen ihtiyaç olmadan yapılmaz).

## 5. Tasarım dili

**Tez:** “sakin, hassas bir ölçüm aleti.” AutoCAD yerleşimi ve klavye alışkanlıkları korunur; yüzey, tipografi, ikon ve etkileşim kalitesi yükseltilir.

**Yüzeyler (koyu):** zemin `#0B0D10` · tuval `#12151A` · panel `#171B21` · yükselti `#1D232B` · overlay `#0B0D10/%92` · çizgi `#242B33` / güçlü `#303A45`. **Metin:** `#E8EDF4` / `#A6B2C0` / `#5F6B78`. **Aksan:** `#4C8DFF`. **CAD renkleri ayrı token:** ızgara `#1A2028`, eksen X `#E05A5A`, eksen Y `#4FBF6A`, snap `#FFB020`, grip `#36C2FF`, sıcak grip `#FF5C5C`, crosshair `#E8EDF4`, kâğıt `#F7F5EF` — tema değişse de çizim renkleri değişmez.

**Tipografi:** Inter (OFL) gövde 13 px, başlık 15/20 semibold; JetBrains Mono (OFL) komut satırı ve koordinatlar 12,5 px; tablo sayıları hizalı. **Ölçü:** 4 pt taban, satır 28/32, radius 4/6/8, panel 220–480 px. **Hareket:** 90 ms durum, 150 ms popover, 180 ms panel, 220 ms diyalog; `reduce-motion` → 0 ms. **Odak:** 2 px aksan halkası, 2 px boşluk.

**İmza yüzeyler:** (1) birleşik başlık + belge sekmeleri; (2) otomatik daralan şerit + arama; (3) komut güvertesi (satır içi tamamlama, anahtar sözcük çipleri, geçmiş); (4) sanal tablolu sağ ray (Katmanlar/Özellikler); (5) enstrüman durum çubuğu; (6) kompakt ViewCube; (7) ⌘K komut paleti (295 komut + takma adlar).

**Kalite kapıları:** kontrast AA; %100 klavye erişimi; AccessKit rolleri; durum asla yalnız renkle; 15 dilde taşma testi; dark/light × 3 genişlik golden ekran testleri; kare bütçesi.

## 6. Aşamalar ve kapılar

| Faz | Çıktı | Çıkış ölçütü | Tahmini ajan-saat |
|---|---|---|---|
| **0 Doğrulama** | S1 tuval perf · S2 GPUI-WASM ölçümü · S3 platform/IME/imleç · S4 Türkçe-Arapça/bidi · S5 otomasyon sözleşmesi · spec + görev planı | 250k segment 60 fps, gecikme ≤ 1 kare, MCP e2e yeşil | 40–70 |
| **1 İskelet + tasarım sistemi** | token/tema/font/ikon, başlık+menü+şerit+dock+komut güvertesi, `xtask` katman kuralı, CI golden | görsel onay (dark/light) | 90–140 |
| **2 Tuval** | DisplayList→GPUI ressamı, pan/zoom, crosshair, osnap, grip, seçim | perf bütçesi | 70–110 |
| **3 Komut + otomasyon** | komut satırı/palet, denetim metotları | mevcut MCP betikleri değişmeden | 70–110 |
| **4 Paneller + diyaloglar** | Katmanlar, Özellikler, Araç Rafı, XREF, QuickCalc, layout, ViewCube, plot/mtext/options | `arayuz-paritesi.md` P0 | 140–220 |
| **5 Parite + cutover** | a11y, i18n, golden; `spcad` varsayılan GPUI; egui masaüstü yolu silinir | parite kapısı | 80–120 |
| **6 Temizlik** | web/FreeBSD adaptörü, `craft-ui` çıkışı, belgeler | temiz `cargo xtask ci` | 40–70 |

**Toplam ≈ 450–700 ajan-saat; 3–4 paralel ajanla ≈ 110–175 duvar saati (tahmini, ±%40).** Bu süre boyunca egui'ye yalnız kritik yama; yeni özellik yok.

## 7. Riskler (pre-mortem)

| Risk | Olasılık | Önlem |
|---|---|---|
| Tuval performansı yetmez | Orta-yüksek | Faz 0 bütçesi; Plan B özel wgpu katmanı; aksi hâlde C'ye dönüş |
| İmleç giriş gecikmesi | Orta | Faz 0 ölçümü; present mode/gpui-fast değerlendirmesi |
| Web beklentisi karşılanmaz | Yüksek | B2: web egui adaptöründe kalır |
| Windows'ta görsel regresyon otomatik test edilemez | Yüksek | macOS/Linux golden; Windows smoke + manuel görsel onay |
| Arapça/bidi eksik | Orta | S4; gerekirse yalnız Arapça UI ertelenir |
| gpui-pre snapshot kırılması | Orta | Tam pin; yükseltme kapısı (derleme + golden + perf) |
| API sözleşmesi regresyonu | Orta | Ortak sözleşme testleri, MCP e2e |
| Bağımlılık/derleme ağırlığı (485 → 794 kilit paketi) | Kesin | Kabul; CI önbelleği, sürüm bütçesi |
| Ekip odağı CAD alfa işlerinden kayar | Yüksek | Motor dokunulmaz; paralel ajanlar; cutover tarihi sabit |

## 8. Doğrulama

- `cargo xtask ci` + yeni `xtask ui` kapıları (katman kuralı, golden, perf).
- gpui-kit `test-support` ile headless etkileşim testleri; macOS/Linux'ta görüntü golden'ları (Windows'ta headless renderer yok — bilinen sınır).
- `xtask ui-perf`: apartment.dxf + sentetik 250k segment; p50/p95 kare, giriş gecikmesi.
- Denetim sözleşmesi e2e: `cmdline.input`, `engine.execute`, `ui.*`, `app.*` iki ön yüzde.
- Erişilebilirlik: VoiceOver/NVDA/Orca + klavye matrisi.
- Görsel inceleme: dark/light, 15 dil, 1280/1440/3840.

## 9. Açık kararlar

1. **Web politikası:** B1 / **B2 (öneri)** / B3.
2. **FreeBSD:** Faz 0 sonucu; GPUI çalışmazsa egui adaptörü.
3. **Android:** GPUI planının dışında; ayrı proje (Rust çekirdek + native UI; `gpui-mobile` olgunlaşırsa yeniden değerlendirme).
4. **macOS 15+ / Windows 10+ / Linux Vulkan** asgari hedefleri kabul ediliyor mu? (Mevcut uygulamadan daha dar.)

## 10. Kaynaklar

- [gpui-kit kurulum ve platform gereksinimleri](https://gpui-kit.com/docs/installation)
- [gpui-kit WebAssembly kapsamı](https://gpui-kit.com/docs/webassembly)
- [gpui-kit Mobile kapsamı](https://gpui-kit.com/docs/mobile)
- [gpui-kit README (bileşenler, AccessKit, 120 FPS)](https://github.com/longbridge/gpui-kit)
- [gpui-fast (retained mode ölçümleri)](https://github.com/longbridge/gpui-fast)
- [crates.io: gpui-kit](https://crates.io/crates/gpui-kit) · [gpui-pre](https://crates.io/crates/gpui-pre) · [gpui-pre-platform bağımlılık hedefleri](https://crates.io/crates/gpui-pre-platform/0.3.10/dependencies)
- [docs.rs: gpui-pre Window boyama API'si](https://docs.rs/gpui-pre/0.3.10/gpui/struct.Window.html) · [Canvas](https://docs.rs/gpui-pre/0.3.10/gpui/elements/fn.canvas.html)
