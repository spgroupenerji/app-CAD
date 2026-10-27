# WP2–WP4 planı — ikon sistemi ve görsel denge

> **Durum:** plan (kod değişmedi) · **Tarih:** 2026-10-27 · **Hedef:** shadcn/Lucide estetiğinde keskin ikonlar + ölçülebilir görsel denge · **Karar:** iki katmanlı ikon sistemi

## 1. Sorun tanısı (WP1 gerçek ekran görüntüsünden)

| Belirti | Ölçüm / kanıt |
|---|---|
| Sol araç rafı dev ve boş görünüyor | 250 px panel, 12 araç; ikon+etiket dikey ritmi 46 px |
| Şerit dengesiz | 94 px gövde, 9 grup; büyük düğme etiketi 10–11,5 px; gruplar arası boşluk değişken |
| Panel başlıkları “levha” gibi | düz şerit, 11 px büyük harf, aksiyon ikonları 14 px |
| Komut girişi görünmüyor | taze profilde yalnız “Command:” etiketi; giriş satırı çizilmiyor |
| İkonlar tutarsız | 136 el çizimi ikon, 996 satır; stroke 1,0–2,2 arası değişken; optik hizalama yok |
| Diyalog düz | başlık/gövde/footer aynı yüzeyde; kart/gölge/odak hiyerarşisi yok |
| Tipografi dağınık | şerit 9,5–11,5; panel 11–12,5; başlık 14–17 px (token var, kullanım kısmi) |

## 2. İkon paketleri — doğrulama (crates.io, 27 Eki 2026)

| Paket | Sürüm | egui uyumu | Lisans | İçerik | Bizim için |
|---|---|---|---|---|---|
| **iconflow** | 2.1.0 | GUI bağımlılığı yok (font + codepoint) | MIT (Lucide ISC) | 14 paket; **Lucide 1.665 ikon, 0,41 MiB**; per-pack feature | ✅ **Birincil öneri (genel UI)** |
| **egui-phosphor** | 0.14.0 | **egui ^0.36** ✅ | MIT/Apache | Phosphor, 5 ağırlık + **subset** | ✅ yedek (stil Lucide değil) |
| egui_material_icons | 0.9.0 | **egui ^0.37** ❌ | MIT | Material Symbols | ❌ 0.36 ile derlenmez |
| egui-remixicon | 0.34.0 | **egui ^0.34** ❌ | Apache-2.0 | Remix Icon | ❌ 0.36 ile derlenmez |
| egui_phosphor_icons | 0.5.0 | doğrulanmalı | — | Phosphor çatalı | ⚠️ gereksiz çoğaltma |
| shadcn_egui ekosistemi | 0.6.0 | egui ^0.34 ❌ | MIT | 15 bileşen | ❌ (WP0 raporu) |
| **lucide-icons** (Rust) | 1.61.0 | GUI'siz path/sabitler | MIT AND ISC | Lucide tanımları | ✅ SVG/vektör yolu için yedek |
| **craft-ui `SvgIconSet`** | mevcut git rev | egui 0.36 + egui_extras SVG loader | MIT/Apache | uygulama SVG'lerini yükler | ✅ alternatif (ölçüm gerekli) |

> Kullanıcı iddiası doğrulandı: iconflow 14 paket (Bootstrap, Phosphor, **Lucide**, Tabler, Remix, Heroicons…), tip güvenli `Pack/Style/Size` API'si, MIT; ancak **ikon başına alt küme yok**, yalnız paket başına. “Her DPI'da sıfır kayıp” pratikte iyi; kesirli ölçeklerde font atlası yeniden rasterleştirir, spike'ta ölçülecek.

## 3. Öneri — iki katmanlı ikon sistemi

**Katman 1 — genel arayüz ikonları (menü, şerit, palet başlığı, durum çubuğu, diyalog, dosya işlemleri):**
- **iconflow `pack-lucide`** ikon fontu; `FontFamily::Name("lucide")` olarak kaydedilir.
- `ikonlar::ArayuzIkonu` enum'u → Lucide adı eşlemesi (derleme zamanı tablo + test).
- Boyutlar 14/16/18/20 px; renk `Tokens` (text_dim / icon / accent); dikey hizalama yardımcısı (`ikon_dondur`).
- Kazanç: shadcn ile birebir set, tek TTF, DPI'dan bağımsız keskinlik, çok küçük kod yüzeyi.
- Neden font: 16–20 px'te en keskin sonuç, kare başına tessellation yok, tema rengi tek satır.

**Katman 2 — CAD komut ikonları (line, trim, fillet, hatch, pline…):**
- Hiçbir pakette CAD semantiği yok; **kendi vektör setimiz** kalır ve yeniden çizilir.
- Kural: 24×24 grid, **1,5 px stroke** (±0,1), yuvarlak uç/birleşim, optik merkez ±0,5 px, 3 ton (mevcut geometri açık, oluşturulan mavi, noktalar turuncu).
- Uygulama: `simgeler.rs`'teki 136 varyant, veri tabanlı path tablosuna taşınır (tek renderer); opsiyonel `xtask` ile SVG dışa aktarım (belge/paketleme).

**Neden her ikisi:** font çok renkli/CAD semantiği taşıyamaz; vektör her ikon için font atlasından daha esnek ama 136 ikonun elle çizimi gerekir. Genel UI'da Lucide fontu, CAD'de vektör = en yüksek kalite/efor oranı.

### Spike'ta karşılaştırılacak (P0)
1. **A:** iconflow Lucide fontu — 20 temsili ikon, 100/125/150/200 % ölçek.
2. **B:** craft-ui `SvgIconSet` + `egui_extras` SVG loader (resvg) — aynı 20 ikon.
3. **C:** mevcut `simgeler.rs` boyama — referans.
Ölçüt: keskinlik (ekran görüntüsü), optik hizalama, açılış süresi, binary boyutu, çok renk esnekliği. Karar P1'in ilk işi.

## 4. Entegrasyon mimarisi

```text
kitapliklar/arayuz-egui/src/ikonlar/
  mod.rs        ArayuzIkonu, boyut tokenları, ikon_dondur()
  lucide.rs     Pack::Lucide eşlemesi (ad → codepoint), testler
  cad/          CAD ikon path tablosu + tek renderer (+SVG dışa aktarım)
```
- `tema.rs`: `IKON_KUCUK 14`, `IKON_ORTA 16`, `IKON_BUYUK 18`, `IKON_SERIT 20`.
- Renk: `Tokens::get().icon / text_dim / accent`; pasif durumda opaklık değil renk.
- Testler: eşleme tamlığı (her enum varyantı Lucide'de var), eksik ikon yok, golden ikon sayfası (dark/light).

## 5. Görsel denge planı (ikon kadar önemli)

| Alan | Hedef |
|---|---|
| Yerleşim | sol ray 52 px; araç rafı 240–320 px (boşsa otomatik daralt); sağ palet 300 px; 1 px gutter |
| Şerit | 84 px gövde, 9 grup, etiket 10,5 px, büyük ikon 20 px; grup arası 8 px |
| Panel | başlık 32 px (11 px büyük harf + 16 px aksiyon ikonları), satır 28 px, iç boşluk 8/12 |
| Komut güvertesi | 36 px giriş + 2 satır geçmiş; ⌘K palet; anahtar sözcük çipleri 24 px |
| Durum çubuğu | 28 px, gruplu; mono koordinat; toggle çipleri 22 px |
| Tipografi | yalnız token ölçeği (10,5/11,5/12,5/14/17); chrome'da ad-hoc boyut yasak |
| Diyalog | 748 px; başlık 54, meta 32, sekme 40, gövde kartları, footer 56; gölge + 1 px kenar |
| Boşluk | 4 pt ızgara; panel içi 8/12; komşu bileşenlerde tek ölçek |

Bunlar **WP2 (bileşen kiti)** ve **WP3 (krom yenileme)** iş paketleriyle uygulanır; ikonlar WP4.

## 6. Aşamalar, efor, kabul

| Faz | İş | Efor (ajan-saat) | Kabul |
|---|---|---|---|
| **P0** | İkon spike (A/B/C), ölçüm, karar | 8–12 | 3 ölçekte görsel rapor + karar kaydı |
| **P1** | `ikonlar` modülü + Lucide eşlemesi + CAD path tablosu iskeleti | 12–18 | Eşleme testleri, ikon sayfası golden |
| **P2** | Bileşen kiti: panel başlığı/satır, düğme, alan, sekme, tooltip, diyalog | 40–60 | Token dışı boyut/renk yok; golden |
| **P3** | Krom: şerit kompakt, komut güvertesi, durum çubuğu, araç rafı, ViewCube | 30–45 | Yerleşim metrikleri + ekran görüntüsü onayı |
| **P4** | 136 CAD ikonunun yeniden çizimi (öncelik: şeritteki ~40) | 20–35 | 24/1,5 kuralı; ikon sayfası onayı |
| **P5** | Denge/golden/a11y/i18n (dark-light × 15 dil × 3 genişlik) | 10–15 | Kapılar yeşil |

**Toplam ≈ 120–185 ajan-saat.** İkonlar tek başına “dengesizliği” çözmez; asıl kazanç P2–P3'te.

## 7. Riskler ve önlemler

| Risk | Önlem |
|---|---|
| iconflow Rust 1.92 ister | UI sandığı zaten 1.95; sorun yok (çalışma alanı 1,90 ayrıca güncellenir) |
| Font atlası kesirli DPI'da bulanır | P0'da 125/150 % ölçüm; gerekirse SVG'ye geçiş (B) |
| Lucide adları CAD komutlarıyla karışır | `ArayuzIkonu` ile `Icon` (CAD) ayrı enum; eşleme testi |
| Çok renkli ikon fontla olmaz | CAD katmanı vektör kalır |
| 136 ikonun çizimi uzar | Önce şeritteki ~40, sonra diğerleri; ikon sayfasıyla onay |
| egui_extras + resvg ağır | Yalnız B seçilirse; ölçümle karar |
| Lisans/atıf | NOTICE/ATTRIBUTION: iconflow MIT, Lucide ISC, Phosphor MIT (yedek) |

## 8. Kaynaklar

- [iconflow (crates.io)](https://crates.io/crates/iconflow) · [README: paketler, lisanslar](https://github.com/FerrisMind/iconflow)
- [egui-phosphor](https://crates.io/crates/egui-phosphor) · [egui-remixicon](https://crates.io/crates/egui-remixicon) · [egui_material_icons](https://crates.io/crates/egui_material_icons)
- [lucide-icons (Rust)](https://crates.io/crates/lucide-icons) · [Lucide](https://lucide.dev)
- Yerel: `craft-ui` `SvgIconSet` (`crates/ui/src/icons/mod.rs`), `egui_extras` SVG loader
