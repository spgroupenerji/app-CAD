# egui Tasarım Sistemleri — Birincil Kaynak Doğrulaması

> Kapsam: `shadcn_egui` ve benzeri adayların **app-CAD** (egui 0.36.2 + craft-ui) mimarisine uygulanabilirliği.
> Yöntem: crates.io API, GitHub REST API, `raw.githubusercontent` README/manifest, docs.rs ve yerel kaynak taraması.
> Ölçüm tarihi: **2026-10-27**. Tüm sayılar bu tarihteki API yanıtlarıdır; doğrulanmamış hiçbir alan rapora girmedi.

## 0. Yönetici özeti

**Kritik bulgu:** doğrulanan dört hazır "shadcn for egui" projesinin **hiçbiri egui 0.36'ya bağlı değil** (0.33 / 0.34). Caret (`^0.34`) semantik kuralı gereği 0.36 ile birleşmez; `cargo` ağaca **ikinci bir egui kopyası** ekler ve craft-ui/uygulama tipleriyle uyuşmaz. Ayrıca brief'teki üç eşleme **yanlış** çıktı (bkz. §5).

## 1. Aday kimlik doğrulaması (crates.io + GitHub)

| Aday | crates.io | Son sürüm | İndirme | GitHub deposu (gerçek) | ★ | Lisans | egui isteği | 0.36 uyum |
|---|---|---|---|---|---|---|---|---|
| shadcn-egui | `shadcn-egui` | 0.6.0 · 2026-10-18 | 110 | [oetiker/shadcn-egui](https://github.com/oetiker/shadcn-egui) | 1 | MIT | `^0.34` | ❌ |
| shadcn-egui-core | `shadcn-egui-core` | 0.6.0 · 2026-10-18 | 356 | aynı | 1 | MIT | bağımlılıksız | ✅ (veri tipi) |
| shadcn-egui-font-geist-mono | `shadcn-egui-font-geist-mono` | 0.6.0 · 2026-10-18 | 136 | aynı | 1 | MIT | — | ✅ (font) |
| egui-shadcn | `egui-shadcn` | 0.5.0 · 2026-03-11 | 1.700 | [FerrisMind/shadcn-rs](https://github.com/FerrisMind/shadcn-rs) | 90 | MIT | `^0.33` | ❌ |
| egui-shadcn (pjankiewicz) | **yayınlanmamış** | 0.1.0 | — | [pjankiewicz/egui-shadcn](https://github.com/pjankiewicz/egui-shadcn) | 21 | MIT | `egui = "0.33"` | ❌ |
| shadcn-rs | `shadcn-rs` | 0.1.0 · 2026-01-07 | 4.976 | [hughdbrown/shadcn-rs](https://github.com/hughdbrown/shadcn-rs) | 3 | MIT OR Apache-2.0 | **yew/gloo — egui değil** | ❌ |
| shadcn-rs-cli | `shadcn-rs-cli` | 0.5.0 · 2026-03-12 | 32 | FerrisMind/shadcn-rs | 90 | MIT OR Apache-2.0 | clap/toml (kod üretir) | ⚠️ |
| ouroboros-ui | **yayınlanmamış** | 0.1.0 | — | [Type-zero-labs/ouroboros-ui](https://github.com/Type-zero-labs/ouroboros-ui) — **archived** | 8 | MIT | `egui = "0.34.1"` | ❌ |

Kaynaklar: [crates.io/shadcn-egui](https://crates.io/api/v1/crates/shadcn-egui) · [shadcn-egui bağımlılıkları](https://crates.io/api/v1/crates/shadcn-egui/0.6.0/dependencies) · [crates.io/egui-shadcn](https://crates.io/api/v1/crates/egui-shadcn) · [egui-shadcn bağımlılıkları](https://crates.io/api/v1/crates/egui-shadcn/0.5.0/dependencies) · [crates.io/shadcn-rs](https://crates.io/api/v1/crates/shadcn-rs) · [shadcn-rs bağımlılıkları](https://crates.io/api/v1/crates/shadcn-rs/0.1.0/dependencies) · [crates.io/shadcn-rs-cli](https://crates.io/api/v1/crates/shadcn-rs-cli).

## 2. Kapsam ve entegrasyon biçimi

| Aday | Biçim | Kapsam | Bileşen | Eksikler |
|---|---|---|---|---|
| shadcn-egui | crate (7 parça: core + font + ikon) | shadcn v4 presetleri (vega/nova/maia/lyra/mira/luma/rhea), açık-koyu mod, preset kodu içe aktarma, Lucide/Hugeicons | **15** (Button, Badge, Input, Label, Card, Tabs, Switch, Checkbox, Select, Slider, Separator, Tooltip, Popover, Dialog, Icon) | tablo, docking, animasyon, erişilebilirlik katmanı, i18n |
| egui-shadcn (FerrisMind) | crate | props-first API, `shadcn-common` + `twill` tema motoru | ~40 (README) | — |
| egui-shadcn (pjankiewicz) | **git / kopyala-yapıştır** (crates.io'da yok) | 60+ widget: DatePicker, Command, Table, Sidebar, PropertyGrid, 1600+ Lucide | **60+** | yayınlanmamış; crate adı başkasına ait |
| shadcn-rs-cli | **copy-paste CLI** (gerçek shadcn felsefesi) | `npx shadcn` muadili; egui + iced hedefli bileşen iskeleti üretir | bileşen bazlı | üretilen kodun bakımı kullanıcıda |
| ouroboros-ui | **git bağımlılığı** (crates.io'da yok) | token-first, 3 katmanlı token (core→semantic→component), 60+ bileşen, node-editor katmanı, **governance testleri** | 60+ | **arşivlenmiş** |

Kaynaklar: [oetiker README](https://github.com/oetiker/shadcn-egui) · [pjankiewicz README](https://github.com/pjankiewicz/egui-shadcn) · [FerrisMind README](https://github.com/FerrisMind/shadcn-rs) · [ouroboros-ui README](https://github.com/Type-zero-labs/ouroboros-ui).

## 3. Olgunluk ve risk

| Aday | İlk yayın | Son commit | Sürüm | Bakım | Risk |
|---|---|---|---|---|---|
| shadcn-egui | **2026-10-12** (15 günlük) | 2026-10-24 | 3 | tek kişi (oetiker), CI var | **Yüksek** — 0.6.0'a 6 günde ulaşmış; API oturmamış |
| egui-shadcn (FerrisMind) | 2025-12-15 | 2026-09-20 | 3 | ekip, RU/PT-BR README | **Yüksek** — README'de açık uyarı: *"API is currently unstable … including breaking changes. Always pin exact crate versions"* |
| pjankiewicz/egui-shadcn | 2026-02-15 | 2026-06-13 | yayın yok | tek kişi | **Çok yüksek** — 4 aydır commit yok, crates.io kimliği yok |
| shadcn-rs (hughdbrown) | 2026-01-07 | 2026-09-25 | 1 | tek kişi | **Alakasız** — Yew/WASM kütüphanesi, egui bağımlılığı yok |
| shadcn-rs-cli | 2026-03-12 | 2026-09-20 | 1 | FerrisMind ekibi | **Orta** — kod üretici, runtime bağımlılığı değil |
| ouroboros-ui | 2026-06-05 | 2026-10-05 | yayın yok | **arşivlenmiş** | **Kullanılamaz** — GitHub `archived: true` |
| craft-ui (mevcut) | 2026-10-09 | **2026-10-26** | git rev | 350★ / 675 fork, SonarCloud, 9 uygulama | **Düşük** — zaten üretimde |

## 4. Depo gerçekleri (yerel doğrulama)

| Ölçüm | Değer | Kanıt |
|---|---|---|
| Kilitli egui | **0.36.2** | `Cargo.lock` |
| Arayüz boyutu | 94 dosya / **29.545 satır** | `kitapliklar/arayuz-egui` |
| craft-ui kullanımı | **11 çağrı noktası** — `docking`, `layout`, `buttons` | `kenetleme.rs`, `simgeler.rs` |
| egui_dock / egui_extras / egui-phosphor | **0 / 0 / 0** | depo taraması |
| egui_kittest | **44** referans (snapshot + wgpu) | `arayuz-egui` dev-deps |
| i18n | **15 TSV katalog** (ar,cs,de,el,es,fr,hu,it,ko,pt_br,ru,sv,tr,uk,zh) + `mod.rs` + 51,6 KB test | `src/i18n/` |
| Tema katmanı | `tema.rs` 34,9 KB, `ctx.set_theme` + `ctx.all_styles_mut` | `tema.rs:710-711` |
| Lisans | MIT OR Apache-2.0 | `Cargo.toml` |

**craft-ui upstream drift:** pinlediğimiz rev `5c874d5` **egui 0.36** kullanıyor (`egui_extras 0.36`, `egui_kittest 0.36`), ancak [craft-libs/main](https://github.com/storytold/craft-libs) bugün **epaint 0.37 / egui 0.37**'ye geçmiş. egui 0.37.0 2026-10-15'te yayınlandı ([crates.io/egui](https://crates.io/api/v1/crates/egui)); yükseltme baskısı yakında.

## 5. Kimlik çakışmaları (brief'teki hatalar düzeltildi)

| Brief iddiası | Doğrulanan gerçek |
|---|---|
| `egui-shadcn` → pjankiewicz | `egui-shadcn` crates.io adı **FerrisMind**'e ait (repo alanı `FerrisMind/shadcn-rs`). pjankiewicz'in projesi **yayınlanmamış**; `cargo add egui-shadcn` yanlış projeyi getirir. |
| `shadcn-rs` ★90 = FerrisMind | ★90 FerrisMind'in; **`shadcn-rs` crate'i hughdbrown'a ait** ve Yew/WASM. İki farklı proje, aynı ad. |
| shadcn-egui = alltheseas / eavelabs | Yayınlanan crate **oetiker**'in. alltheseas **GPL-3.0** (MIT/Apache uyumsuz, 2025-12-29'da durmuş); eavelabs 25 KB'lik **iskelet**. |
| craft-ui = storytold | crates.io'daki `craft-ui` 1.4.7 **aphrody-labs/craft-libs** (farklı org, 15 indirme). Bizimki git+rev. **Sürümle asla eklenmemeli.** |

## 6. egui ekosistemi (0.36 uyumlu alternatifler)

| crate | Sürüm | egui | İndirme | ★ | Lisans | Not |
|---|---|---|---|---|---|---|
| egui_extras | 0.37.0 / **0.36.2** | `^0.36` ✅ | 12,8 M | — | MIT/Apache | tablo, tarih seçici; **kullanmıyoruz** |
| egui_dock | 0.21.1 | `^0.36` ✅ | 4,86 M | 645 | MIT | craft-ui `docking` ile **çakışır** — gereksiz |
| egui-phosphor | 0.14.0 | `^0.36` ✅ | 380 B | 171 | Apache-2.0 | ikon seti; craft-ui `icons` zaten SVG katalog sunuyor |
| egui_icons | 0.0.1 | `^0.36` ✅ | 1.548 | — | — | tek sürüm, 0.0.1 → **riskli** |
| egui_kittest | 0.37.0 / **0.36.2** | `^0.36` ✅ | 1,54 M | — | MIT/Apache | **zaten 44 yerde kullanıyoruz** |
| egui (stil API) | **0.37.0** | — | 28,4 M | — | MIT/Apache | 0.36.2'de `set_theme`, `all_styles_mut`, `style_mut_of` mevcut ([docs.rs](https://docs.rs/egui/0.36.2/egui/struct.Context.html)) |

egui'nin kendi stil API'si (tema + `Style`/`Visuals` + `all_styles_mut`) zaten **yeterli bir token taşıyıcısı**; eksik olan bileşen katmanını craft-ui sağlıyor.

## 7. "f16 HDR / Catmull-Rom / SDR-HDR compositing" iddiası — **YANLIŞ**

`Select-String` ile tüm depo (`target`, `.git` hariç) tarandı:

| Terim | Eşleşme | Gerçek anlamı |
|---|---|---|
| `rgba16` | **0** | — |
| `Rgba16Float` | **0** | — |
| `catmull` | **0** | — |
| `SDR` | **0** | — |
| `HDR` | 11 | Hepsi DXF **header** değişkeni (`d.header.f64("DONUTID")`, `FILLETRAD`, `HPNAME`) ve PNG **`IHDR`** chunk assert'i |
| `f16` | 5 | Hepsi `encode_utf16` / `len_utf16` — **UTF-16**, yarım float değil |

Karşı-kanıt (kod): köşe formatı `2 × f32 konum + 4 × u8 sRGB renk` ([gpu.rs:26](kitapliklar/arayuz-egui/src/gpu.rs#L26)), hedef `target.format.is_srgb()` ([gpu.rs:595](kitapliklar/arayuz-egui/src/gpu.rs#L595)). **8-bit sRGB**; lineer ışık, kayan noktalı framebuffer veya ton eşleme yok. İddia asılsızdır.

## 8. Seçenekler

| | A — Hazır crate | B — shadcn felsefesiyle öz-tasarım | C — craft-ui'yu tasarım sistemi yükselt |
|---|---|---|---|
| İş yükü | düşük görünür, **gerçekte bloke** | yüksek (~60 bileşen yeniden) | **orta, kademeli** |
| egui 0.36 | ❌ hiçbiri uyumlu değil; ikinci egui | ✅ | ✅ (pin bizde) |
| craft-ui çakışması | yüksek (docking/ikon/tema ikilenir) | orta | yok |
| 15 dil / Türkçe | ❌ dışa kapalı | kendi metin katmanımız | ✅ mevcut `i18n` korunur |
| CAD kromu (tuval 134 KB) | ❌ dar | ✅ | ✅ |
| Bakım riski | yüksek (tek kişi, 15 günlük) | bize bağlı | düşük (350★, 9 uygulama) |
| Lisans | MIT ✅ / GPL-3.0 ❌ | bizim | Apache-2.0 ✅ |

## 9. Öneri: **C** (craft-ui'yu tasarım sistemi olarak yükselt), B'den sınırlı ödünçleme

**Gerekçe:** (1) Hiçbir hazır crate egui 0.36'da değil — A seçeneği bugün teknik olarak imkânsız; 0.37 yükseltmesinden sonra bile craft-ui ile docking/ikon çakışması doğar. (2) craft-ui zaten **docking + layout + icon button + font + localized** katmanlarını sağlıyor ve 29.545 satırlık arayüzün 11 noktasında kullanılıyor. (3) Eksik olan **tema/token** katmanını `tema.rs` (34,9 KB) zaten kurmuş; eksik yalnız *semantik token sözlüğü*. (4) i18n ve CAD kromu dışa devredilemez.

**Somut adımlar (bağımlılık eklemeden):**
1. `tema.rs` içine shadcn'in **semantik token adlarını** (`background/foreground/primary/muted/border/ring`) yerleştir; ham değerleri `cizim_stilleri.rs`'ten türet.
2. ouroboros-ui'nin **governance testi** fikrini ödünç al: `tema.rs` dışında ham renk/boşluk kullanımını clippy `disallowed_methods` + bir testle kır.
3. Görsel regresyon için mevcut `egui_kittest` snapshot altyapısını (44 referans) bileşen galerisine bağla.
4. `egui_extras` yalnızca **tablo** için (0.36.2) değerlendirilsin; docking için `egui_dock` **eklenmesin** (craft-ui `docking` yeterli).
5. egui 0.37 + craft-ui yeni rev yükseltmesi ayrı bir iş olarak planlansın (upstream zaten 0.37'de).

**Kaçınılacaklar:** `shadcn-egui`, `egui-shadcn` (FerrisMind), `ouroboros-ui` (arşiv), `alltheseas/shadcn-egui` (GPL-3.0), `cargo add craft-ui` (yanlış org).
