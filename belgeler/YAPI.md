# Proje yapısı

Bu belge SP-GROUP CAD deposunun yaşayan yapı otoritesidir; her yeniden yapılandırma görevi ilgili
bölümü güncellemekle yükümlüdür. Kodun bugünkü durumunu anlatır; planlar `belgeler/yol-haritasi.md`
içindedir. Ayrıntılı teknik anlatım `belgeler/mimari.md`, ürün karşılaştırması
`belgeler/hedef-uygulama-paritesi.md` dosyasındadır.

```text
/
├── Cargo.toml                 → Çalışma alanı: 12 kitaplık, 3 uygulama, xtask; sürüm ve profil
├── Cargo.lock                 → Kilit dosyası (depoda tutulur)
├── README.md                  → Kullanıcı tanıtımı, kurulum, indirme ve lisans künyesi
├── YOL-HARITASI.md            → Tek sayfalık yol haritası özeti
├── release/                   → Son Windows beta derlemesi; indirilebilir tek .exe (depoya işlenir)
├── LICENSE-MIT / LICENSE-APACHE / NOTICE → Çift lisans ve zorunlu bildirimler
├── ATTRIBUTION.md             → Dosya bazlı varlık telifi (cargo xtask assets denetler)
├── clippy.toml / rustfmt.toml → Lint ve biçim ayarları
├── .coderabbit.yaml           → CI analiz yapılandırması
│
├── kitapliklar/               → Kitaplıklar; katmanlar `cargo xtask layers` ile zorlanır
│   ├── geometri/              → L0: f64 vektör, matris, yay, eğri, kesişim, bölge izleme
│   ├── dxf/                   → L0: ASCII/ikili DXF etiket okuyucu ve yazıcı
│   ├── renk/                  → L1: AutoCAD renk indeksi, gerçek renk, ByLayer/ByBlock
│   ├── belge/                 → L1: çizim veritabanı, sembol tabloları, bloklar, kopyala-yaz
│   ├── yazitipi/              → L2: tek çizgili yazı tipi, TTF/OTF, TEXT/MTEXT yerleşimi
│   ├── goruntuleme/           → L2: görüntüleme listeleri, tarama, çizgi tipi, ölçü geometrisi
│   ├── kisitlar/              → L2: parametrik çözücü, kısıtlar, ifadeler
│   ├── girdi-cikti/           → L3: DXF eşlemesi, PDF/SVG/PNG dışa aktarma
│   ├── dwg/                   → L3: acadrust (MPL-2.0) üzerinden DWG ⇄ DXF köprüsü
│   ├── motor/                 → L4: Session, komut kaydı, komut satırı, snap, seçim, geri al
│   ├── arayuz-egui/           → L5: değiştirilebilir egui ön yüzü, paneller, iletişim kutuları
│   └── mcp/                   → L5: stdio üzerinden MCP sunucusu
│
├── uygulamalar/
│   ├── spcad/                 → Masaüstü uygulaması (eframe/wgpu); `build.rs` Windows ikonu
│   ├── spcad-cli/             → Başsız `info`, `convert`, `run`, `commands`, `sample`, `mcp`
│   └── spcad-web/             → Aynı arayüzün WASM derlemesi (DWG yok)
│
├── xtask/                     → Çalışma alanı araçları: ci, layers, assets, wasm, parity,
│   │                            version, stats, ico, release
│   └── veri/menu-catalog.txt  → Menü genişliği denetiminin girdisi
│
├── varliklar/uygulama-simgesi/ → Tek ikon kaynağı ve türevleri
│   ├── sp-group-cad.png       → SP-GROUP marka kaynağı (marka varlığı; açık lisans dışı)
│   ├── spcad-1024.png vd.     → Uygulama tarafından gömülen PNG türevleri
│   └── spcad.ico / .icns      → Windows ve macOS paketleme ikonları
│
├── paketleme/                 → Platform paketleme tanımları ve betikleri
│   ├── ikonlar.py / ikonlar.sh → İkonları tek kaynaktan yeniden üretir
│   ├── linux/                 → .desktop, AppStream, nfpm, Flatpak, postinst
│   ├── macos/                 → Info.plist, imzalama, DMG arka planı ve düzeni
│   ├── windows/               → WiX MSI, imzalama, PowerShell paketleme
│   └── web/                   → Statik site başlıkları ve yayın betiği
│
├── belgeler/                  → Sürümde tutulan kalıcı dokümantasyon
│   ├── YAPI.md                → Bu belge
│   ├── mimari.md              → Katmanlar, veri modeli, komutlar, görüntüleme akışı
│   ├── eksikler.md            → Öncelikli iş listesi (buradan iş seçilir)
│   ├── yol-haritasi.md        → Kilometre taşları
│   ├── *-paritesi.md          → Geometri, arayüz, dosya biçimi, donanım, yerelleştirme
│   ├── denetim-protokolu.md   → JSON denetim kanalı sözleşmesi
│   └── gorseller/             → README ekran görüntüleri (kendi uygulamamızdan)
│
├── katkida-bulananlar/        → Hakkında penceresine derlenen katkıcı verisi
├── ornekler/                  → Kodla üretilmiş örnek çizimler
└── Sürüm dışı / yerel
    ├── plan/                  → Yerel plan, durum ve AutoCAD gözlemleri (.gitignore)
    ├── licensing/             → Ticari kullanım ve marka notları (.gitignore)
    ├── AGENTS.md, CLAUDE.md   → Ajan talimatları (.gitignore)
    └── .codegraph/ .gitnexus/ graft/ .claude/ → Ajan indeksleri (yerel)
```

## Katmanlar

Bağımlılık yönü yukarıdan aşağıyadır; alt katman üst katmanı bilmez:

| Katman | Kitaplıklar | Kural |
| --- | --- | --- |
| L0 | `geometri`, `dxf` | Yalnız standart kitaplık ve temel yardımcılar |
| L1 | `renk`, `belge` | Belge modeli; egui/wgpu bilmez |
| L2 | `yazitipi`, `goruntuleme`, `kisitlar` | Görüntü ve çözüm üretimi |
| L3 | `girdi-cikti`, `dwg` | Dosya ve dışa aktarma |
| L4 | `motor` | Oturum, komutlar, etkileşim; arayüz bilmez |
| L5 | `arayuz-egui`, `mcp` | Sunum ve protokol |
| app | `spcad`, `spcad-cli`, `spcad-web` | Çalıştırılabilirler |

`cargo xtask layers` ihlali derlemeyi durdurur. L5 altındaki hiçbir sandık egui, eframe, winit,
wgpu veya rfd kullanamaz; arayüz bu yüzden değiştirilebilir.

## Temel akışlar

### Komut çalıştırma

1. Kullanıcı komut satırına yazar (`Session::input`) veya menü/toolbar bir komut kimliği çağırır.
2. `kitapliklar/motor/src/komut/mod.rs` içindeki `CommandSpec` kaydı komutu bulur.
3. JSON biçimi doğrudan `run` alanını çalıştırır; etkileşimli biçim `interactive` istem makinesini
   başlatır. Programatik çağrılar iletişim kutusu açmaz.
4. Komut belgeyi değiştirir; geri alma kopyala-yaz depo sayesinde bütün çizim anlık görüntüsüdür.
5. `Session::execute` ve etkileşimli komutun her adımı `catch_unwind` koruması altındadır; panik
   çizimi geri yükler ve hata döndürür.

### Arayüz

`arayuz-egui` yalnız motor durumunu okur ve `app.run(id, params)`, `app.start(id)` veya
`app.cmdline(text)` üzerinden eylem yapar. Renkler `tema::Tokens` kaynağından gelir. Paneller
çizim verisine sahip değildir.

### Çizim ve dosya

- `goruntuleme::build` değişiklik başına görüntüleme listesi üretir; tuval bunu wgpu ile çizer.
- DXF okuma/yazma `girdi-cikti` içindedir; `dwg` yalnız acadrust köprüsünü sarar.
- PDF, SVG ve PNG dışa aktarma `girdi-cikti` katmanındadır.

### Denetim ve ajan erişimi

- Masaüstü uygulaması `--control PORT` ile `127.0.0.1` üzerinde JSON satırları dinler
  (`kitapliklar/arayuz-egui/src/denetim.rs`; sözleşme `belgeler/denetim-protokolu.md`).
- `spcad-cli mcp` ve `kitapliklar/mcp` aynı komutlara MCP üzerinden erişir.
- Komut satırı, menüler, betikler, CLI, denetim kanalı ve MCP aynı `CommandSpec` kaydını kullanır.

## Değişmezler

- **Asla çökme:** üretim kodunda `unwrap`, `expect`, `panic!`, `unsafe` yoktur; hatalar `Result` ile
  yayılır; girdiden gelen sayılar düşmandır; blok özyinelemesi sınırlıdır.
- **Temiz oda:** Autodesk, Adobe veya Avid varlıkları kopyalanmaz; dosya biçimleri kamuya açık
  şartnamelerden ve kendi testlerimizden gelir.
- **Her şey komuttur:** kullanıcıya görünen davranış `CommandSpec` + testtir; yalnız arayüz komutları
  `kitapliklar/arayuz-egui/src/menuler.rs` içindedir.
- **Varlık politikası:** her dosya varlığının `ATTRIBUTION.md` satırı vardır. Tek istisna SP-GROUP
  markasıdır (`varliklar/uygulama-simgesi/sp-group-cad.png` ve türevleri). ArtCraft markaları bu dağıtımdan
  kaldırılmıştır ve geri getirilemez.
- **Yazı tipleri bu depoda değildir:** `storytold/craft-fonts` isteğe bağlı `CRAFT_FONTS_DIR`
  girdisidir; `Cargo.toml` bağımlılığı değildir.
- **WASM kırılmaz:** `cargo xtask wasm` her değişiklikte çalışır.
- **Katmanlar:** `cargo xtask layers` ihlali kabul etmez.

## Adlandırma

- Dizin ve dosya adları Türkçedir; ASCII, dizinlerde kebab-case, Rust modüllerinde snake_case
  kullanılır (`kitapliklar/girdi-cikti`, `kitapliklar/motor/src/komut/olculendirme.rs`).
- Cargo paket adları (`spcad-*`) ve `use spcad_*` kimlikleri İngilizce kalır: bunlar dış API'dir ve
  Cargo'da dizin yolu ile paket adı bağımsızdır. Yeni bir taşımada `cargo metadata --no-deps` ile doğrula.
- Dış sözlük çevrilmez: DXF, DWG, MCP, egui, CLI, web, wasm, xtask, wgpu, `hicolor/.../apps`,
  AutoCAD komut kimlikleri ve dosya biçimi alanları.
- Entegrasyon test dosyaları da Türkçedir; test edilen AutoCAD komutunun kimliği değişmez.

## Doğrulama

Her commit öncesi tek kapı:

```sh
cargo xtask ci
```

Bu komut sırasıyla biçim denetimi, `clippy -D warnings`, testler, varlık telifi, katmanlar ve WASM
denetimini çalıştırır. Tekil denetimler:

```sh
cargo xtask assets     # ATTRIBUTION.md eksiklerini gösterir
cargo xtask layers     # katman ihlallerini gösterir
cargo xtask wasm       # wasm32 hedefi için kontrol
cargo xtask parity     # menü genişliği kontrol listesini yeniler
```

Paketleme ve ikon yenileme:

```sh
python paketleme/ikonlar.py   # varliklar/uygulama-simgesi/sp-group-cad.png → tüm ikonlar
paketleme/linux/package.sh    # deb/rpm/AppImage/Flatpak
paketleme/macos/package.sh    # .app ve DMG
paketleme/windows/package.ps1 # MSI
```

## Yerel dizinler

`plan/`, `licensing/`, ajan talimatları ve ajan indeksleri sürüm dışıdır; depo geçmişine girmez.
`plan/STATUS.md` o oturumun durumunu, `plan/execution-plan.md` görev listesini, `plan/autocad/`
referans uygulama gözlemlerini tutar.
