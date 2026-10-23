# SPCAD mimarisi

> **Son inceleme:** 2026-10-10 · **Son güncelleme:** 2026-10-10 · **Değişiklik:** büyük (depoya eklenen ilk mimari belge, `main` dalındaki koddan bd2412f revizyonunda yazıldı) · **Hedef:** Autodesk AutoCAD 2027

SPCAD'in bugün nasıl inşa edildiği. Bu belge `main` dalındaki kodu anlatır, planları değil; planlar
[yol-haritasi.md](yol-haritasi.md) içindedir. Depo haritası ve bu şekli koruyan kurallar (katmanlama,
asla çökme, her şey komuttur, temiz oda) [YAPI.md](YAPI.md) dosyasındadır.

## Çalışma alanı

12 kitaplıkta, 3 uygulamada ve `xtask` içinde yaklaşık 57,000 satır Rust (2026-10-10'da sayıldı).

| Katman | Sandık | Satır | Ne yapar |
|---|---|---:|---|
| L0 | `geom` | 2,067 | f64 2B/3B vektörler, `Mat3`/`Mat4`, sınırlar, doğrular, yaylar ve kabarcıklar, daireler, elipsler, fit noktası enterpolasyonlu (rasyonel) B-spline'lar, doğru/yay kesişimleri, düzleştirme, HATCH/BOUNDARY için düzlemsel bölge izleme (`region.rs`) |
| L0 | `dxf` | 571 | Bağımsız DXF etiket okuyucu/yazıcı: ASCII ve ikili, düşman girdiye karşı etiket sayısı sınırları |
| L1 | `color` | 265 | AutoCAD renk indeksi tablosu, gerçek renk, ByLayer/ByBlock |
| L1 | `doc` | 3,054 | Çizim veritabanı: başlık, sembol tabloları (katmanlar, çizgi tipleri, yazı/ölçü/tablo/çoklu kılavuz stilleri), bloklar, model alanı, yerleşimler, kısıtlar; kopyala-yaz `EntityStore`; kendi tarama deseni ve çizgi tipi kitaplığımız (`library.rs`) |
| L2 | `fonts` | 2,062 | Tek çizgili çizim yazı tipimiz, `skrifa` üzerinden TrueType/OpenType/`.ttc` ana hatları, TEXT ve MTEXT yerleşimi ve biçimlendirme kodları, karakter başına geri dönüş ve FONTALT |
| L2 | `render` | 3,300 | Görüntüleme listeleri (varlık tanıtıcılarıyla etiketlenmiş çoklu çizgiler, üçgenler, noktalar): çizgi tipleri, taramalar, degradeler, metin, blok açılımı, ölçü geometrisi, kâğıt alanı kırpması; PNG, çizdirme önizlemesi ve testler için CPU rasterleştirici (`tiny-skia`) |
| L2 | `constraints` | 2,504 | Kendi parametrik çözücümüz: sönümlü Gauss–Newton / Levenberg–Marquardt (`lm.rs`), 12 geometrik ve ölçü kısıtı, ifadeler ve parametreler, AutoConstrain çıkarımı |
| L3 | `io` | 5,124 | DXF ⇄ belge eşlemesi (`dxf_read.rs`, `dxf_write.rs`, `dxf_ext.rs` içinde SPCAD xdata), PDF çizdirme, SVG ve PNG dışa aktarma |
| L3 | `dwg` | 149 | DXF köprüsü üzerinden DWG: `acadrust` sandığı (MPL-2.0, değiştirilmemiş) DWG ⇄ DXF baytlarını dönüştürür; tiplerini başka hiçbir şey görmez. Yalnızca yerel |
| L4 | `engine` | 25,090 | `Session`, komut kaydı (`cmd/*`), komut satırı ayrıştırıcısı ve istem makineleri, nesne yakalamaları ve kutupsal/orto (`snap.rs`), seçim (`select.rs`), tutma noktaları (`grips.rs`), ilişkisellik (`assoc.rs`), sistem değişkenleri, birimler, geri alma, R-tree uzamsal indeksi (`spatial.rs`) |
| L5 | `ui-egui` | 9,849 | Değiştirilebilir egui ön yüzü: GPU tuvali (`gpu.rs`, wgpu), menüler ve kısayollar (`menus.rs`), komut satırı, paletler, Katman Özellikleri Yöneticisi ve diğer iletişim kutuları, ViewCube, kodla çizilen simgeler, temalar, arayüz dizesi kataloğu (`i18n/`, İngilizce ve 15 çeviri dili), JSON denetim kanalı (`control.rs`) |
| L5 | `mcp` | 559 | Başsız oturum üzerinde veya çalışan uygulamaya köprülenmiş MCP sunucusu (stdio üzerinden JSON-RPC 2.0) |
| app | `spcad` | 1,102 | Masaüstü uygulaması (eframe/wgpu; Windows'ta varsayılan olarak DirectX 12) |
| app | `spcad-cli` | 823 | `info`, `convert`, `run` (betikler), `commands`, `sample`, `mcp` |
| app | `spcad-web` | 88 | Aynı arayüzün WASM derlemesi (DWG yok) |
| tool | `xtask` | 1,064 | `ci`, `layers`, `assets`, `wasm`, `parity`, `version`, `stats`, simge üretimi |

`cargo xtask layers` katmanlamayı zorlar: L5 altındaki hiçbir şey egui, eframe, winit,
wgpu veya rfd'ye bağlı değildir.

## Veri modeli

- Bir `Drawing`; başlık değişkenleri, sembol tabloları, blok tanımları, model alanı ve kâğıt alanı
  yerleşimleridir. Varlıklar bir tanıtıcı, katman, renk, çizgi tipi, çizgi kalınlığı, saydamlık ve
  bir `EntityKind` taşır: Line, Point, Circle, Arc, Ellipse, LwPolyline, Polyline3d, Spline, Ray, XLine,
  Text, MText, AttDef, Insert (niteliklerle), Dimension (doğrusal, hizalı, açısal, çap,
  yarıçap, ordinat, yay uzunluğu), Leader, MLeader, Hatch (desen, dolu, degrade), Solid,
  Trace, Face3d, Viewport, Image, Wipeout, Table ve Unknown (ham DXF etiketleri).
- Modelde henüz 3B katı cisimler, yüzeyler, ağlar, bölgeler (ACIS), çoklu çizgiler, tolerans
  çerçeveleri veya altlıklar yok ([eksikler.md](eksikler.md)).
- Varlık koleksiyonları kopyala-yazdır; bu yüzden geri alma az maliyetli bir bütün-çizim anlık
  görüntüsüdür.
- Koordinatlar f64 dünya birimleridir. 2B çalışma WCS'de yapılır; henüz UCS yok.

## Komutlar

Kullanıcıya görünen her eylem bir `CommandSpec`'tir (`kitapliklar/motor/src/komut/mod.rs`): id = AutoCAD
komut adının küçük harfli hâli, etiket, menü yolu, kısayol, takma adlar, bir JSON `run`, bir
`enabled` denetimi ve isteğe bağlı bir etkileşimli istem makinesi. 295 motor komutu kayıtlıdır (128'i
etkileşimli istemli, kaynaktan 2026-10-10'da sayıldı), ayrıca `kitapliklar/arayuz-egui/src/menuler.rs` içinde
yaklaşık 30 yalnızca arayüz komutu vardır. Komut satırı, menüler, araç çubuğu, Araç Takımları, betikler, CLI, denetim
kanalı ve MCP aynı komutlara ulaşır. JSON çağrıları asla iletişim kutusu açmaz.

## Görüntüleme

`render::build` bir uzayı değişiklik başına bir kez görüntüleme listesine dönüştürür; arayüz bunu
wgpu'ya toplu hâlde yükler (`gpu.rs`). Düzleştirme toleransı görünüme göre değişir. R-tree (`spatial.rs`)
düzenlemeden sonra değişen depo parçası başına güncellenir; böylece 200k varlıkta seçim yaklaşık
0.001 ms'de kalır. PNG, PDF, SVG ve çizdirme önizlemesi aynı görüntüleme listesini CPU rasterleştirici
veya vektör yazıcılar üzerinden yeniden kullanır.

## Dosyalar

DXF yerel yoldur: `dxf` etiketleri ayrıştırır, `io::dxf_read` bunları belgeye eşler ve
`io::dxf_write` bir R2000 (AC1015) DXF yazar. DWG, `acadrust` tarafından bellekte DXF'e dönüştürülür ve
aynı yoldan okunur; DWG kaydetmek bizim DXF'imizi yazıp geri dönüştürür. Yalnızca SPCAD'e özgü veriler
(kısıtlar, ilişkisellik ayrıntıları, tablo bayrakları) kendi uygulama adımız altında xdata içinde
taşınır. Ayrıntı ve eksikler: [dosya-bicimi-paritesi.md](dosya-bicimi-paritesi.md).

## Ajan denetimi

- **Denetim kanalı:** `spcad --control PORT` JSON satırları kabul eder (`cmdline.input`,
  `engine.execute`, `ui.screenshot`, `ui.render` …); bkz. [denetim-protokolu.md](denetim-protokolu.md).
- **MCP:** `spcad-cli mcp` (başsız) veya `--connect HOST:PORT` (çalışan uygulamayı yönetir); bkz.
  [mcp.md](mcp.md).
- **CLI:** başsız toplu işler için `spcad-cli run --script … --export out.png`. stdout'u erken
  kapatan bir okuyucu (`… | head`) çıktının geri kalanını düşürür ama işi korur — sonraki
  komutlar, `--save`, `--export` — ve çıkış durumu hâlâ hataları bildirir; başka bir stdout
  yazma hatası bildirilir ve durumu 1 yapar.

## Güvenlik

Üretim sandıkları `unwrap`/`expect`/`panic!` kullanımını reddeder ve `unsafe` yasaklar. `Session::execute` ve
etkileşimli girdi `catch_unwind` altında çalışır; bir panik kaçarsa çizim geri yüklenir. Girdi boyutuna
bağlı ayırmalar sınırlandırılır (DXF etiket sayıları, DWG boyut sınırları, tarama parçası ve ada sınırları,
`MAX_BLOCK_DEPTH`).

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-10 | büyük | Depoya eklenen ilk mimari belge, koddan ölçüldü (sandık boyutları, varlık türleri, komut sayıları) |
