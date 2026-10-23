# Dosya biçimi paritesi

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-17 · **Değişiklik:** küçük (DWG okuma, öksüz kalmış DGN çizgi tipi bileşenlerini dışarıda bırakır) · **Hedef:** Autodesk AutoCAD 2027

AutoCAD 2027'nin okuduğu veya yazdığı her biçim, SPCAD'nin desteği, sadakati ve testleriyle. İş
maddeleri [eksikler.md](eksikler.md#dosya-biçimleri) içinde; ağırlıklı puan
[hedef-uygulama-paritesi.md](hedef-uygulama-paritesi.md) belgesini besler.

**Özet (tahmini):** dosya biçimleri, AutoCAD'inkinin ≈ **30%**'u. En yaygın 2B DXF/DWG çizimlerini
açmak çalışır, ancak kaydetme kayıplıdır: SPCAD'nin modellemediği her şey düşürülür ve hiçbir test
gerçek bir üçüncü taraf dosyası kullanmaz. DXF ve DWG 2000–2018'de kaydeder (varsayılan 2018, #421).
Kalan: **≈ 170–280 Opus 5.5 saat**: DXF/DWG sadakati için ≈ 40–60 saat (beta'yı belirleyen kısım),
aşağıdaki diğer biçimler için ≈ 110–180 saat ve 3B modellemeyi bekleyen 3B değişim biçimleri için
≈ 20–40 saat.

## Bu nasıl ölçüldü

- SPCAD: 2026-10-10'da `kitapliklar/dxf`, `kitapliklar/girdi-cikti` (`dxf_read.rs`, `dxf_write.rs`, `pdf.rs`,
  `svg.rs`, `tests.rs`) ve `kitapliklar/dwg` kaynaklarından okundu. Hiçbir şey çalıştırılmadı (ölçüm
  geçişi derleme yapmadı).
- AutoCAD: Aç/Farklı Kaydet/İçe Aktar/Dışa Aktar/Ekle/Çizdir iletişim kutularındaki biçimler, Autodesk'in
  yayımlanmış belgelerinden ve `xtask/veri/menu-catalog.txt` içindeki menü ağacından. Kurulu
  AutoCAD 2027 for Mac paketi yalnızca adlar listelenerek denetlendi (temiz oda kuralı: içindeki
  hiçbir şey okunmaz): `Resources/Support`, `acad.lin`, `acadiso.lin`, `acad.pat`,
  `acadiso.pat`, `acad.mln`, `.dct` yazım sözlükleri, `.pss` ve `.shx` dosyalarını listeler; bu da
  aşağıdaki destekleyici biçimlerin hedefte var olduğunu doğrular. Hiçbiri kopyalanamaz.
- AutoCAD tarafından üretilmiş hiçbir dosya commit edilmez veya test verisi olarak kullanılmaz.
  Gerçek dosya korpusu, sahibin eylemini gerektirir (bkz. [eksikler.md](eksikler.md)): bize lisanslayan
  kişilerin ürettiği dosyalar, ayrı bir `storytold/cadcraft-corpus` deposunda tutulur.

## DWG, sürümlere göre

AutoCAD 2027, R2.0'dan itibaren her DWG sürümünü açar ve 2018 (AC1032, hâlâ güncel DWG biçimi),
2013, 2010, 2007, 2004, 2000 ve R14 kaydeder. SPCAD, DWG'yi `acadrust` 0.6.3 crate'i ile DXF'e
dönüştürüp o DXF'i okuyarak okur; kendi R2000 DXF'ini yazıp seçilen sürümde bir DWG'ye dönüştürerek
kaydeder (#421): SAVEAS `version`, yoksa çizimin kendi sürümü (dosyanın `$ACADVER` değeri, yeni bir
çizim için 2018), 2000 istenmedikçe en az 2004. Köprü, 2010+ DWG'de iki acadrust 0.6.3 eksikliğini
onarır: birleştirilmiş tablo hücreleri ve tablo stili başlık/üst bilgi bastırması.

| DWG sürümü | Kod | AutoCAD 2027 | SPCAD okuma | SPCAD yazma | Nasıl test edildi |
|---|---|---|---|---|---|
| R1.0–R2.6, R9, R10, R11/R12 | MC0.0–AC1009 | okuma | **hayır** (`is_dwg` yalnızca `AC10xx` sihirli sayısını kabul eder; acadrust R13 öncesini hedeflemez) | hayır | — |
| R13 | AC1012 | okuma | acadrust destek iddia eder; test edilmedi | hayır | yok |
| R14 | AC1014 | okuma, yazma | acadrust destek iddia eder; test edilmedi | **hayır** | yok |
| 2000–2002 | AC1015 | okuma, yazma | köprü üzerinden evet | evet, istendiğinde (gerçek renkler, saydamlık veya tablo stilleri yok: biçimde hiçbiri yok) | sentetik gidiş-dönüşler (`dwg_at_every_version_reads_back`, `dwg_roundtrip_keeps_non_ascii_text`) |
| 2004–2006 | AC1018 | okuma, yazma | köprü üzerinden | evet | `dwg_at_every_version_reads_back`, `dwg_roundtrip_keeps_extension_data` |
| 2007–2009 | AC1021 | okuma, yazma | köprü üzerinden | evet | `dwg_at_every_version_reads_back` |
| 2010–2012 | AC1024 | okuma, yazma | köprü üzerinden | evet | `dwg_at_every_version_reads_back`, `ac1024_dwg_with_many_entities_opens` |
| 2013–2017 | AC1027 | okuma, yazma | köprü üzerinden | evet | `dwg_at_every_version_reads_back` |
| 2018–2027 | AC1032 | okuma, yazma (varsayılan) | köprü üzerinden; gerçek dosyalarda test edilmedi | evet (yeni çizimler için varsayılan) | `dwg_at_every_version_reads_back` (yalnızca sentetik) |
| Web derlemesi | — | — | **hayır** (hata: "save as DXF") | hayır | — |

Aşağıdaki DXF okuma/yazma yollarında kaybedilen her şey DWG üzerinden de kaybedilir; ayrıca
`acadrust`'un kendi DXF çıktısının atladığı şeyler (proxy nesneleri, ACIS verisi, özel nesneler:
ölçülmedi). Kullanıcı bildirimi #57: 10 MB'lık bir DWG yaklaşık iki dakika sonra başarısız oldu;
sınırlar artık kontrolden çıkan dönüşümleri erken durdurur (#175), ancak verim sorunu devam eder.
Hiçbir çizgi tipi tanımının kullanmadığı DGN çizgi tipi bileşenleri (DGN içe aktarmalarının
artıkları, bazı çizimlerde yüz binlerce) okumada bir notla dışarıda bırakılır, böylece bu tür
dosyalar artık DXF sınırını aşmaz (#1482). **DWG ≈ 30% okuma, ≈ 15% yazma.**

## DXF, sürümlere göre

| DXF sürümü | AutoCAD 2027 | SPCAD okuma | SPCAD yazma |
|---|---|---|---|
| R12 (AC1009), ASCII | okuma, yazma | evet: sürüm geçidi yok; eski tarz POLYLINE/VERTEX okunur (çok yüzlü ağlar 3B çoklu çizgilere düzleştirilir) | **hayır** |
| R12 ikili | okuma, yazma | **hayır** (ikili ayrıştırıcı R13+ 2 baytlık grup kodları bekler) | hayır |
| R13–R2018, ASCII | 2000/2004/2007/2010/2013/2018'in her birini okuma, yazma | evet, tüm sürümler için tek okuyucu | evet, 2000–2018'in her biri (#421): o `$ACADVER` ile R2000 yapısı (sonraki sürümler yalnızca isteğe bağlı gruplar ekler); 2007'den itibaren UTF-8 metin; CLASS örneği sayıları yok (91) |
| R13–R2018, ikili | okuma, yazma | evet | evet, 2000–2018'in her biri (SAVEAS `binary`; ikili bir dosya QSAVE'de ikili kalır) |
| Kod sayfaları (`$DWGCODEPAGE`, ANSI_932/936/949/950/125x) | evet | evet (#505): 2007 öncesi ASCII ve ikili DXF kendi kod sayfasından çözülür (ANSI_874/932/936/949/950/1250–1258, DOS437/850/852/866, ISO 8859, Macintosh); geçerli UTF-8 olan bir dosya UTF-8 olarak okunur; Johab ve diğer DOS kod sayfaları ANSI_1252 olarak okunur (`dxf/src/codepage.rs`) | `\U+` kaçışları yazar (#169), bu doğrudur; `ANSI_1252` bildirir |
| `\U+`/`\M+` kaçışları | evet | evet (#505'ten beri `\M+`; Johab `\M+4` harfiyen kalır) | `\U+` evet |

### DXF içeriği: nesne tipine göre okuma ve yazma

| İçerik | Okuma | Yazma | Notlar |
|---|---|---|---|
| LINE, POINT, CIRCLE, ARC, ELLIPSE, LWPOLYLINE (genişlikler, kabarıklıklar, yükseklik), SPLINE, RAY, XLINE | evet | evet | gidiş-dönüş testleri |
| POLYLINE 2D/3D | evet | evet (LWPOLYLINE / 3D POLYLINE olarak) | çok yüzlü ve poligon ağları düzleştirilir: kayıplı |
| TEXT, MTEXT, ATTDEF, INSERT + ATTRIB | evet; çok satırlı nitelikler (gömülü MTEXT) okunur ve çizilir | evet; çok satırlı nitelikler gömülü MTEXT olarak yazılır (DXF 2007 ve sonrası; daha eski sürümler: tek satır) | ATTDEF istemi korunur (#137) |
| DIMENSION (7 türün tümü) | evet | evet, anonim `*D` bloklarıyla | yeniden açılan bir ölçünün `*D` bloğu kendi adı altında yeniden oluşturulur, referanssız olanlar düşürülür (#495); DIMASSOC nesneleri yazılır |
| LEADER | evet | evet | |
| MULTILEADER | evet (#459) | evet, MLEADERSTYLE ile yerel MULTILEADER (#459) | çoklu kılavuz başına birkaç iniş tek bir taneye katlanır; nesne başına geçersiz kılmalar ve notlandırmalı ölçek bağlamları korunmaz |
| HATCH deseni / dolu | evet | evet; ilişkisel taramalar sınır nesnelerini bloklarda da korur ve sınır tarama reactor'ünü taşır (AutoCAD'de esnetilen dinamik bloklar dolguyu sürükler) | |
| HATCH degrade | evet | evet (#162, #80'i düzeltir) | |
| SOLID, TRACE, 3DFACE | evet | evet | |
| ACAD_TABLE | evet | evet (varlık + blok) | |
| VIEWPORT | evet | evet; dondurulmuş katman listeleri korunur (#150), ayrıca diğer programların okuduğu standart grup 341 katman tutamaçları olarak | DWG: DXF→DWG dönüştürücüsü henüz grup 341 yazmaz (dondurulmuş katmanlar bir SPCAD gidiş-dönüşünde xdata ile hayatta kalır); VP katman renk geçersiz kılmaları kaybolur (#99, PR #197) |
| IMAGE | evet, IMAGEDEF ile | evet, IMAGEDEF ve IMAGEDEF_REACTOR ile | henüz iliştirme komutu veya resmin görüntülenmesi yok (FF10) |
| WIPEOUT | evet | evet (#147, #79'u düzeltir) | |
| MLINE (+ MLINESTYLE), HELIX, TOLERANCE | evet (#510, #420) | evet | |
| REGION | diğer programların REGION'ları (ACIS verisi) Unknown olarak korunur | bizim bölgelerimiz SPCAD xdata ile etiketlenmiş kapalı çoklu çizgiler olarak yazılır (#510; ACIS'in genel bir belirtimi yok) | |
| 3DSOLID, BODY, SURFACE, MESH, SHAPE, UNDERLAY, OLE2FRAME, LIGHT, CAMERA, SECTION, POINTCLOUD, proxy varlıkları | Unknown olarak korunur (ham etiketler) | okunduğu gibi geri yazılır, CLASS kayıtlarıyla (#528); **DWG kaydetmede düşürülür** (acadrust) | düzenlemeleri yalnızca katmanları izler |
| Başlık değişkenleri | tümü okunur | 36 yazılır | AutoCAD yaklaşık 280 yazar |
| Tablolar: LAYER (+ saydamlık, açıklama), LTYPE, STYLE, DIMSTYLE, BLOCK_RECORD | evet | evet | |
| Tablolar: VPORT, VIEW, UCS, APPID | evet | evet: kaydedilmiş görünüm penceresi yapılandırmaları (#504), adlandırılmış görünümler, UCS'ler | UCS yüksekliği kaybolur |
| Nesneler: LAYOUT, DICTIONARY, XRECORD, TABLESTYLE, DIMASSOC | evet | evet | |
| Nesneler: GROUP, MLINESTYLE, MLEADERSTYLE, IMAGEDEF, SCALE | evet | evet (gruplar #417, ölçek listesi #468) | |
| Nesneler: VISUALSTYLE, PLOTSETTINGS, malzemeler, alan nesneleri ve diğer modellenmemiş nesneler | ham korunur (#528) | okunduğu gibi geri yazılır | düzenlenemez |
| Yabancı xdata, uzantı sözlükleri, reactor'ler, modellenmemiş nesneler, renk kitapları | ham korunur (#528); bir varlığın xdata'sı varlığın parçasıdır, bu yüzden COPY, MIRROR, ARRAY ve yapıştırma onu kopyalarda korur (EXPLODE parçaları korumaz) | geri yazılır, tutamaçlar yeniden eşlenir, silinen nesnelere referanslar düşürülür | DWG üzerinden: tablo kayıtlarındaki ve bloklardaki xdata, varlık renk kitapları ve reactor'ler, özel sınıf nesneleri kaybolur (acadrust) |
| Nesneler: dinamik blok verisi (ACAD_EVALUATION_GRAPH, BLOCK…PARAMETER, BLOCK…ACTION, tutamaçlar) | bir modele okunur (`dynblock`, `blocks.dynamic`): parametreler, görünürlük durumları, değer kümeleri, esnetme ve arama eylemleri; referanslar: dinamik blokları, değerleri ve özel özellikleri (`blocks.reference`, Properties ▸ Custom, salt okunur); ham korunur (#528) | okunduğu gibi geri yazılır; değiştirilen bir görünürlük durumu AutoCAD'in yazdığı gibi yazılır (`AcDbBlockRepBTag` ile yeni bir anonim blok, nesnelerin `AcDbBlockRepETag` değeri, referansın temsil verisi) | görünürlük durumları (ve yalnızca onları ayarlayan aramalar) değiştirilebilir (`blocks.reference.set`, Properties ▸ Custom); mesafeler, konumlar ve sürdükleri eylemler henüz değil; DWG açma ve kaydetme de bunu korur (bir AutoCAD dinamik bloğuyla kontrol edildi) |
| Tutamaçlar ve sahipler | varlık tutamaçları korunur; diğerleri yeniden oluşturulur | tutarlı | SPCAD yeniden kaydetmelerinde kararlı (#495); diğer programların tablo ve nesne tutamaçları bir kez yeniden numaralandırılır |

**DXF ≈ 50% okuma, ≈ 30% yazma** (tahmini; ağırlıklar: yaygın 2B varlıklar 50%, notlandırma
nesneleri 20%, tablolar/nesneler 15%, yabancı veri koruma 15%).

## Diğer biçimler

| Biçim | AutoCAD 2027 | SPCAD | Pariteye saat (tah.) |
|---|---|---|---|
| DWT şablonları | açma, kaydetme | `.dwt` ve `.dxf` şablonlarından NEW/QNEW, SAVEAS `.dwt` (#552) | 0–1 |
| DWS standartları | açma, kaydetme, CHECKSTANDARDS | hayır | 6–10 |
| PDF çıktısı (PLOT, EXPORTPDF) | çok sayfalı, katmanlar, aranabilir metin, köprü metinleri | çizgi kalınlıklarıyla vektör PDF, gömülü TrueType alt kümelerinde gerçek metin, katmanlar (OCG), dosya başına birkaç sayfa ve köprü metinleri (#567, #571); "Çizim saydamlığı" açıkken nesne saydamlığı PDF alfası olarak (seçenek DXF/DWG'de kaydedilmez: genel bir çizim düzeni bayrağı yok) | 1–2 |
| PDF içe aktarma (PDFIMPORT) ve PDF altlıkları (PDFATTACH) | evet | hayır | 15–25 |
| Raster görüntü iliştirme (PNG, JPEG, TIFF, BMP …) | evet | IMAGE varlığı var, iliştirme/yükleme/görüntüleme yolu yok | 6–10 |
| PNG / JPEG / BMP / TIFF dışa aktarma | PNGOUT, JPGOUT, BMPOUT, TIFOUT | PNG; SVG de (AutoCAD SVG dışa aktarmaz) | 1–2 |
| DWF/DWFx dışa aktarma, DWF altlığı | evet | hayır | 10–15 |
| DGN içe/dışa aktarma/altlık | evet | hayır (genel bir belirtim gerekiyor; sahip kararı) | 25–40 |
| WMF, EPS, ACIS SAT, STL, FBX, IGES/STEP (Windows), 3DS | çeşitli | hayır; çoğu 3B ve M11'i bekler | 20–40 |
| SHX yazı tipleri (`.shx`, büyük yazı tipleri) | okuma | **ayrıştırılmaz**: bizim çizgi yazı tipimiz veya FONTALT yerine geçer, bu yüzden metin ölçüleri AutoCAD'den farklıdır | 8–12 |
| Çizgi tipi (`.lin`) ve tarama deseni (`.pat`) dosyaları | kullanıcı dosyalarını yükleme | kullanıcı dosyaları yüklenir (#552); çizgi tipi şekil (SHX) öğeleri atlanır | 1–2 |
| Çizim stilleri (`.ctb`, `.stb`), çizici yapılandırmaları (`.pc3`), `.pmp` | evet | kendi JSON biçimimizde çizim stili tabloları ([cizim-stilleri.md](cizim-stilleri.md)) ve çizimde saklanır; kâğıtlar ve yazdırılabilir alanlarla çizici aygıtları kendi JSON biçimimizde ([yazicilar.md](yazicilar.md), #564); ikili `.ctb`/`.stb`/`.pc3`/`.pmp` okunmaz (genel bir belirtim yok) | 4–8 |
| Betikler (`.scr`) | SCRIPT | CLI, MCP ve kontrol kanalı üzerinden betik metni; bir `.scr` dosyası okuyan SCRIPT komutu yok | 1–2 |
| Takma adlar (`.pgp`), CUI(x), çalışma alanları | evet | kullanıcı takma ad dosyaları ve ALIASEDIT (#552); CUI(x) veya çalışma alanları yok | 4–8 |
| SVG içe aktarma (`.svg`, `.svgz`) | evet (betikler aracılığıyla) | OPEN şekilleri, yolları, grupları ve Inkscape katmanlarını varlıklara okur (#636); CSS sınıfları, `use`, dolgular, metin veya görüntüler yok (çizimin kaybettiği şey açılışta bir notta adlandırılır, #627) | 2–4 |
| AutoLISP (`.lsp`, `.fas`, `.vlx`), `.dll`/`.arx` eklentileri | evet | `.lsp` yüklenir (APPLOAD, `(load)`, bırakılan bir dosya; #758); Visual LISP ActiveX (`vlax-`), DCL iletişim kutuları veya reactor'ler yok; `.fas`/`.vlx` derlenmiştir ve belgelenmemiştir; `.dll`/`.arx` yok | bkz. [eksikler.md](eksikler.md) (E1) |
| Veri çıkarma (`.csv`, `.xls`) ve veri bağlantıları | evet | DATAEXTRACTION/EATTEXT CSV'ye veya bir tabloya (#433); tablo veri bağlantıları CSV okur (#534); `.xls` yok, `.dxe` şablonları yok | 3–5 |
| eTransmit (`.zip` paketleri) | evet | hayır | 4–6 |

## Testler

`kitapliklar/girdi-cikti/src/testler.rs` 36 teste sahiptir (varlıkların, stillerin, ilişkiselliğin, tabloların,
kısıtların, ASCII dışı metnin gidiş-dönüşleri, DWG gidiş-dönüşleri, düşmanca girdi). **Tüm girdiler
sentetiktir**: satır içi dizeler veya kendi yazıcımızın çıktısı. Hiçbir test başka bir program
tarafından üretilmiş bir dosya açmaz ve CI'da hiçbir harici doğrulayıcı çalışmaz (ezdxf'in denetimi
2026-10-06'da bir kez elle çalıştırıldı). Sonraki adım, dosya başına aç → kaydet → yeniden aç →
karşılaştır testi içeren lisanslı bir gerçek dosya korpusu ve çıktımızı AutoCAD'de kara kutu olarak
kontrol etmektir.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-17 | küçük | DWG okuma: hiçbir çizgi tipi tanımının ulaşmadığı DGN çizgi tipi bileşenleri DXF köprüsünden önce dışarıda bırakılır, bir okuma notuyla; bunlardan 888,483 tanesini içeren 83 MB'lık bir arazi planı (1.2 GB DXF, sınırın üzerinde) artık açılır (#1482) |
| 2026-10-12 | küçük | PDF çıktısı: sayfa kurulumunun veya çizimin "Çizim saydamlığı" ayarı açıkken nesne saydamlığı PDF alfası olarak çizdirilir (`/ExtGState` `/ca` `/CA`); PNG çizimleri bunu harmanlar, SVG `opacity` kullanır; onsuz çizimler bayt bayt değişmez |
| 2026-10-12 | küçük | DXF/DWG kaydetme: Model yerleşiminin çizim ayarları, SPCAD model alanını çizdiriyormuş gibi, çizim sınırları sığacak şekilde ölçeklenerek yazılır (PLOTSETTINGS 74 = 1, 75 = 0); diğer programların Plot iletişim kutularının model alanı için kullanamadığı çizim tipi 5 "Layout information" yerine |
| 2026-10-12 | küçük | VIEWPORT dondurulmuş katmanları ayrıca AutoCAD'in grup 341 katman tutamaçları olarak yazılır ve okunur, böylece üçüncü taraf görüntüleyiciler DXF'te görünüm penceresi başına dondurmayı dikkate alır (DWG dönüştürücü eksiği not edildi) |
| 2026-10-12 | küçük | HATCH: bloklar içindeki ilişkisel taramalar kaynak nesnelerini korur; sınırlar `{ACAD_REACTORS}` taşır |
| 2026-10-12 | küçük | Dinamik blok tutamaçları: tutamaç nesneleri grafik kimliğine göre okunur; bir sürükleme, AutoCAD'in bir Properties değişikliğini kaydettiği gibi kaydedilir |
| 2026-10-12 | küçük | Dinamik blok referansları: görünürlük durumları değiştirilebilir ve AutoCAD'in kaydettiği gibi kaydedilir |
| 2026-10-12 | küçük | Dinamik blok referansları: tanım, değerler ve özel özellikler okunur (`blocks.reference`, Properties ▸ Custom) |
| 2026-10-12 | küçük | Dinamik blok verisi satırı: parametreler ve eylemler bir modele okunur (`blocks.dynamic`); DXF ve DWG kaydetme tarafından korunur |
| 2026-10-12 | küçük | Çok satırlı nitelikler (gömülü MTEXT) metinlerini kaybetmek yerine okunur, çizilir ve geri yazılır |
| 2026-10-11 | küçük | Birleştirilmiş işler için satırlar yenilendi: kod sayfaları ve `\M+` (#505), MULTILEADER (#459), IMAGE, MLINE/HELIX/REGION/TOLERANCE, VPORT/VIEW/UCS tabloları, modellenmiş ve ham nesneler, yabancı veri (#528), DWT şablonları ve kullanıcı `.lin`/`.pat`/`.pgp` dosyaları (#552), PDF çıktısı (#567, #571), çizici aygıtları (#564), veri çıkarma (#433, #534); ağırlıklı yüzdeler yeniden ölçülmedi |
| 2026-10-11 | küçük | DXF (ASCII ve ikili) ve DWG 2000–2018'de, varsayılan 2018 olarak kaydedilir (#421) |
| 2026-10-10 | küçük | WIPEOUT ve degradeler #147/#162'den beri yazılır; diğer kaydetme kayıpları sürer (#201, #204, #208, #209, #213, #223) |
| 2026-10-10 | büyük | İlk dosya biçimi paritesi belgesi: DWG ve DXF sürüm sürüm, nesne başına okuma/yazma, diğer biçimler |
