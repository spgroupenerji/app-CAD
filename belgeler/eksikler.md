# SPCAD AutoCAD karşısında nerede eksik kalıyor

> **Son gözden geçirme:** 2026-10-10 · **Son güncelleme:** 2026-10-16 · **Değişiklik:** küçük (ALIASEDIT için Komut Takma Adları iletişim kutusu: U12 ve E5 daraltıldı) · **Hedef:** Autodesk AutoCAD 2027

Her bilinen eksik, her biri tek satır, her bölüm içinde kullanıcı etkisine göre sıralanmış. Bu iş
listesidir: ajanlar buradan seçer (önce bir bölümün başı) ve onu kapatan aynı PR içinde satırı
kaldırır veya günceller. Sayılar ve ağırlıklar [hedef-uygulama-paritesi.md](hedef-uygulama-paritesi.md) içinde;
saatler Opus 5.5 ajan-saatidir ve orada açıklandığı şekilde kalibre edilmiştir. "Tah." atıfta
bulunulan kanıttan bir yargı anlamına gelir. **[alfa engelleyici]** olarak işaretlenmiş satırlar
[alfa kapısı](yol-haritasi.md#alfa-geçidi) eşiğini geçemeyenlerdir; önce onları yapın.

## Tüm alanlarda ilk 10

| # | Eksik | Neden bu sırada | Tah. | Belge |
|---|---|---|---|---|
| 1 | **[alfa engelleyici]** **DWG kaydetmek modellemediğimiz şeyleri düşürüyor** (DXF artık bilinmeyen varlık ve nesneleri, yabancı xdata'yı, uzantı sözlüklerini, reactor'ları, katman durumlarını ve renk kitaplarını (#528) ve yerel MULTILEADER'ları (#459; henüz DWG üzerinden test edilmedi) geri yazıyor; DWG üzerinden acadrust özel sınıf varlık ve nesneleri, tablo kayıtlarındaki ve bloklardaki xdata'yı, varlık renk kitaplarını ve reactor'ları düşürüyor) | Bir meslektaşınızın DWG'sini açıp kaydetmek hâlâ bazı içeriği siler: bir CAD kullanıcısının affedemeyeceği tek şey | 6–10 | [dosya biçimleri](dosya-bicimi-paritesi.md) |
| 2 | **[alfa engelleyici]** **Gerçek dosya test korpusu ve AutoCAD'de kontrol yok** | 36 dosya testinin tümü sentetik; kimse bir SPCAD DWG/DXF'in AutoCAD'de temiz açıldığını doğrulamadı | 10–15 + insan | [dosya biçimleri](dosya-bicimi-paritesi.md#testler) |
| 3 | **Elips ve spline geometrisi mozaiklenmiş** (kesişimler, ofsetler, en yakın noktalar; poliline ofseti kabartıları yok sayar) | Koordinatlar ince biçimde yanlış; elips ofsetleri poliline olarak çıkıyor | 30–50 | [geometri](geometri-paritesi.md) |
| 4 | **[alfa engelleyici]** **Dinamik blok veya REFEDIT yok; dış referanslar (xref'ler) yerinde düzenlenemiyor veya kırpılamıyor** (dış referanslar, #525, gruplar, #417 ve statik bloklar için Blok Düzenleyici geldi) | Üretim çizimlerinin çoğu bunları kullanır | 55–90 | [özellikler](#özellikler) |
| 5 | **Web derlemesinde otomatik kaydetme yok** (masaüstü uygulamasında #406'dan beri SAVETIME otomatik kaydetme, Çizim Kurtarma, AUDIT ve RECOVER var) | Tarayıcı kullanıcıları sekme veya tarayıcı çöktüğünde kaydedilmemiş işlerini kaybeder | 3–5 | [özellikler](#özellikler) |
| 6 | **Hiçbir CI, PR'larda Rust testlerini çalıştırmıyor** (`cargo fmt` main'de düzeltildi) | açık PR'lar kapı olmadan birleşiyor; gerilemeler görülmüyor | 2–3 | [kararlılık](#kararlılık) |

## Özellikler

| # | Eksik | Kanıt | Kullanıcı etkisi | Tah. |
|---|---|---|---|---|
| F1 | **[alfa engelleyici]** Hâlâ "not available yet" yazan istem seçenekleri: -ATTEDIT tek tek Position, Height, Angle ve Style, -ATTEDIT Color içindeki renk kitapları, malzemelerin, çizim stillerinin ve görünüm penceresi yapılandırmalarının -RENAME'i. SPLINEDIT Join, Edit vertex Add ve Elevate order (#559), PEDIT Edit vertex ve diğer SPLINEDIT seçenekleri (#558) tamam: Edit vertex Tangent directions yalnızca PEDIT oturumu boyunca kalır (LWPOLYLINE hiçbirini saklamaz). MTEXT Dynamic/Static columns tamam (#547). SPLINE, MTEXT ve MLEADER seçenekleri tamam (#408): SPLINE Object polilinenin köşelerine oturur (spline çerçevesi saklanmaz) (STRETCH polygon, CP ve WP ve çit seçimi tamam; TRIM/EXTEND, PLINE arc mode ve Length, XLINE tamam (#397); OFFSET, ROTATE ve FILLET seçenekleri daha önce tamam) | `cmd/blocks/attedit.rs`, `cmd/props/rename.rs` | Düşük: en çok kullanılan komutlar tamam | 1–3 |
| F2 | XREF'in yerinde düzenlenmesi ve kırpılması: XCLIP eksik. REFEDIT, REFSET ve REFCLOSE bir blok başvurusunu yerinde düzenler (çalışma kümesi, bloğun geri kalanı soluk, diğer her şey kilitli, kaydet veya vazgeç; #813); kalanlar: bir xref'i yerinde düzenlemek (kaynak çizimi geri yazılmaz; XOPEN onu açar), iç içe başvuruları seçmek, kilitli çizim için soluk bir görünüm ve Kaydet ve Vazgeç içeren bir başlık şeridi, diğer başvuruların nitelik eşitlemesi (ATTSYNC). XATTACH, External References paleti, -XREF (Bind/Detach/Path/Unload/Reload/Overlay), XBIND, XOPEN, overlay xref'ler, göreli yollar ve açılışta yükleme tamam (#525). Yüklenmemiş durum ve xref katmanı geçersiz kılmaları yalnızca oturum boyunca sürer | `cmd/blocks/xref.rs` | Orta: xref'ler yerinde düzenlenemiyor veya kırpılamıyor | 15–25 |
| F3 | **[alfa engelleyici]** (dinamik blok yazarlığı beta) Dinamik blok yazarlığı yok: Blok Düzenleyici'de parametreler ve eylemler eklenemiyor. Diğer dosyalardan gelen dinamik bloklar okunuyor ve başvuruları düzenleniyor (görünürlük, mesafeler, gerdirme ve taşıma, aramalar, tutma noktaları). BEDIT (yeni blokları da başlatan Edit Block Definition iletişim kutusuyla), BSAVE, BSAVEAS ve BCLOSE statik bir blok tanımını düzenler ve her başvuru güncellenir; REFEDIT bir başvuruyu yerinde düzenler; henüz yazarlık paletleri yok | sorun #185 yardım öneriyor | Yüksek: standart kütüphaneler dinamik | 40–65 |
| F4 | Otomatik kaydetme artıkları: web derlemesinde otomatik kaydetme yok (tarayıcı depolaması); otomatik kaydetme dosyaları DWG değil DXF; dosya okuyucularda ayrı bir RECOVER modu yok (SAVETIME, Çizim Kurtarma, AUDIT ve RECOVER #406 ile geldi; AUDITCTL `.adt` günlüğü geldi) | `engine/src/autosave.rs`, `engine/src/audit.rs` | Düşük-orta (web kullanıcıları) | 2–4 |
| F5 | İlişkisel diziler (ASsociative, Levels, polar ROWs, ARRAYEDIT, ARRAYCLOSE) geldi (#448); kalanlar: dizilerin tutma noktasıyla düzenlenmesi (sayı/aralık tutma noktaları), ARRAYPATH ilişkisel değil, ARRAYEDIT'te öğe silme (CTRL+click) ve Key point taban noktaları yok, seviyeler yalnızca Z'yi kaydırıyor (3B görünüm yok) | `cmd/array.rs`, `cmd/array/edit.rs` | Orta | 4–6 |
| F7 | Ölçülendirme: DIMSTYLE yöneticisi iletişim kutusu yok; akıllı DIM yok; DIMTFILL/DIMTFILLCLR, DIMALTTD/DIMALTTZ, DIMUPT ve DIMTXTDIRECTION modellenmiyor; kırık ölçü yüksekliği ve kırılma boyutu stil ayarı değil; DIMBREAK kırılmaları taşınan nesneleri izlemiyor (stil değişkenleri, DIMJOGGED, DIMJOGLINE, DIMBREAK, DIMEDIT, DIMINSPECT ve QDIM modları tamam, #507) | `doc/src/tables.rs`, `cmd/annotate_dimedit.rs`, `cmd/annotate_qdim.rs` | Orta | 10–15 |
| F8 | TOLERANCE artıkları (özellik kontrol çerçeveleri geldi, #420: model, kendi sembol çizimimiz, DXF, komut satırı istemleri, patlatma, tutma noktaları): Geometrik Tolerans iletişim kutusu yok, LEADER/QLEADER tolerans notlandırması yok, bir çerçevenin içeriğinin yerinde düzenlenmesi yok | `cmd/tolerance.rs` | Düşük-orta (mekanik) | 2–4 |
| F10 | MLEADEREDIT (kılavuz ekle/kaldır), MLEADERALIGN ve MLEADERCOLLECT geldi, ancak bir çoklu kılavuzun tek bir landing'i ve tek bir bloğu var (bir koleksiyon diğer blokları yanında blok başvuruları olarak tutar; MLEADERALIGN'de Options menüsü önizlemesi yok); MLEADERSTYLE'ın 5 alanı var (blok içeriği ve spline/kılavuzsuz türler komut başına, stilde değil; blok içeriğinin nitelikleri yok) | `annotate.rs`, `cmd/mleader_content.rs`, `tables.rs:417` | Orta | 8–12 |
| F9 | MTEXT, biçimlendirme araç çubuğu (yazı tipi, yükseklik, kalın, italik, altı çizili, üstü çizili, renk, hizalama, listeler, sekme ve özel karakterler) ve girintiler ile sekme durakları için bir cetvel içeren tuval üzeri bir düzenleyicide yazılıyor (#532); kalanlar: kutu, yazarken metni girintilere, sekmelere, sütunlara veya yazı tipi ağırlıklarına göre yerleştirmiyor (kalın daha geniş görünüyor, üstü çizili çizilmiyor), düzenleyicide hizalama (ekleme) veya sütun denetimleri yok, mevcut MTEXT düzenlenemiyor (DDEDIT/MTEDIT yok), istiflenmiş kesirler ve diğer kodlar yer tutucu olarak görünüyor; alan yok (FIELD, UPDATEFIELD) ve yazım denetimi yok (SPELL): açık PR #551. Paragraf girintileri, sekmeler, madde işaretleri, numaralandırma ve sütunlar geldi (#547) | `ui-egui/mtext_editor.rs`, `engine/mtext_edit.rs` | Orta | 10–18 |
| F11 | Tablolar (hücre stilleri, formüller, TABLEDIT, TABLEEXPORT, tablo bölme ve CSV veri bağlantıları geldi, #534): formüller AutoCAD FIELD nesneleri olarak okunmuyor veya yazılmıyor (diğer programlar değerleri görür); sayı biçimleri yalnızca formül sonuçlarına uygulanıyor; hücre metni kaydırılmıyor; özel hücre stilleri, kırılmalar ve veri bağlantıları SPCAD xdata'sı olarak taşınıyor (DWG üzerinden değil); veri bağlantıları yalnızca CSV okur (Excel aralıkları yok, geri yazma yok, web'de yok); Tablo Stili iletişim kutusunda hücre stili sekmesi yok | `cmd/table/*`, `io/dxf_table.rs` | Düşük-orta | 6–10 |
| F12 | Not ölçeklendirme tamam (#468): ölçek listesi (SCALELISTEDIT), CANNOSCALE, OBJECTSCALE, ANNOALLVISIBLE/ANNOAUTOSCALE, görünüm penceresi ölçeği, DXF, durum çubuğu listesi. Kalanlar: ölçek başına konumlar (ANNORESET etkisiz), görünüm penceresi ölçeğinden ayrı bir görünüm penceresi not ölçeği, bloklar ve eski LEADER'lar içindeki not ölçekli nesneler, AutoCAD dosyalarının nesne başına ölçek bağlamları (not ölçekli değil olarak okunur) | `cmd/annoscale.rs`, `doc/src/anno.rs` | Düşük-orta | 4–8 |
| F14 | **[alfa engelleyici]** Çizdirme artıkları: ikili `.ctb`/`.stb`/`.pc3` dosyaları okunamıyor; model alanı PDF çizimleri varsayılan kenar boşluklarını koruyor (bir aygıt kâğıdının kenar boşlukları yerleşimlere ve PNG çizimlerine uygulanıyor); sistem yazıcıları yok (H3); sayfa kurulumundaki "Çizim saydamlığı" DXF/DWG'ye kaydedilmiyor (kamuya açık DXF Reference'ta bunun için bir çizim yerleşimi bayrağı yok), bu yüzden yalnızca oturum boyunca ve JSON'da kalıcı. Çizdirme iletişim kutusu (#160), çizdirme önizlemesi ve PUBLISH (#186), uç, birleştirme ve dolgu stilleriyle çizim stilleri (CTB ve STB, [cizim-stilleri.md](cizim-stilleri.md), #410), kâğıt boyutları ve yazdırılabilir alanlarla çizici aygıtları ([yazicilar.md](yazicilar.md), #564) ve çizdirilen saydamlık (sayfa kurulumu ve çizdirme iletişim kutusu seçeneği, PLOTTRANSPARENCYOVERRIDE) geldi | `io/src/plotter.rs`, `cmd/plotters.rs` | Orta | 4–8 |
| F13 | Tarama: 23 desen (AutoCAD ~85; bizimkiler özgün kalmalı); Hatch and Gradient ve Hatch Edit iletişim kutularında Önizleme veya Inherit Properties yok (HPDLGMODE 0, HATCH'i komut satırında tutar); MPolygon yok; HATCHEDIT ve -HATCHEDIT, seçilen noktalardan veya kapalı nesnelerden sınır ekler ve seçilen döngüleri kaldırır, ancak sınırıyla birlikte kopyalanan bir tarama kopyaya bağlanmaz; HPGAPTOL yalnızca eğri uçları arasındaki boşlukları kapatır | `doc/src/library.rs`, `cmd/hatch.rs`, `cmd/hatch/edit.rs` | Orta | 6–10 |
| F15 | Görünüm pencereleri: MVIEW Restore "not available yet" yazıyor (yerleşime adlandırılmış görünüm penceresi yapılandırmaları, bkz. U11); kırpılmış bir görünüm penceresi ile kırpma sınırı ayrı ayrı hareket edip ölçekleniyor. Çokgen, nesne ve kırpılmış görünüm pencereleri, VPCLIP, MVIEW ON/OFF/Lock/2/3/4 ve durum çubuğu ölçek listesi geldi (#453) | `cmd/layout/viewports.rs` | Düşük | 2–4 |
| F16 | Gruplar: bir grup seçildiğinde grup sınır kutusu veya tek tutma noktası yok (GROUPDISPLAYMODE), bütün bir grubu kopyalamak yeni bir grup oluşturmuyor, Object Grouping iletişim kutusu yok (GROUP, GROUPEDIT, UNGROUP, -GROUP, PICKSTYLE grup seçimi ve DXF/DWG gidiş-dönüşü geldi, #417); PICKSTYLE'ın ilişkisel tarama biti saklanıyor ama kullanılmıyor | `cmd/groups.rs` | Düşük | 2–4 |
| F17 | Nitelikler (ATTSYNC, genel -ATTEDIT, BATTMAN düzenleme ve DATAEXTRACTION/EATTEXT geldi, #433): doğrulama, ön ayar veya konum kilitleme modları yok ve çok satırlı nitelikler (2026-10-12'den beri okunuyor, çiziliyor ve kaydediliyor) BATTMAN'de oluşturulamıyor veya değiştirilemiyor; veri çıkarımı yalnızca blok başvurularını ve nitelik değerlerini kapsıyor (CSV veya tablo) — sihirbaz, iç içe bloklar, diğer nesnelerin özellikleri, `.dxe` şablonları veya elektronik tablo çıktısı yok | `cmd/blocks/battman.rs`, `cmd/blocks/extract.rs` | Düşük | 4–6 |
| F19 | MATCHPROP Settings bir anahtar sözcük istemi (iletişim kutusu yok) ve malzeme, gölge veya merkez nesne ayarları yok; TEXTTOFRONT ve HATCHTOBACK yalnızca geçerli alanda etkili; OVERKILL poliline segmentlerini diğer nesnelerle karşılaştırmıyor (seçici ve Settings, DRAWORDER Above/Under, TEXTTOFRONT, HATCHTOBACK, OVERKILL tolerans/yok say/örtüşme/uçtan uca seçenekleri geldi, #439) | `props/matchprop.rs`, `draworder.rs`, `overkill.rs` | Düşük | 2–4 |
| F20 | OPTIONS iletişim kutusu tamam (#23): Görüntü, Açma ve Kaydetme, Çizim, Seçim, Kullanıcı Tercihleri ve Profiller sekmeleri ve kullanıcı profili (nesne yakalama modları, çizim anahtarları, Dinamik Girdi, PICKBOX, APERTURE, GRIPSIZE, CURSORSIZE, PICKFIRST, PICKADD, QPMODE, model ve kâğıt arka planı, yakalama işaretçisi boyutu, sistem imleci) yeniden başlatmalardan sonra da kalıyor. Kalanlar: sağ tık özelleştirmesi, Açma ve Kaydetme'de otomatik kaydetme aralığı ve varsayılan kaydetme sürümü (özelliklerin kendisi geldi, #406, #421), AutoSnap mıknatısı ve ipucu anahtarları, diğer çizim alanı öğelerinin renkleri, içe ve dışa aktarımlı adlandırılmış profiller | `ui-egui/src/options/` | Düşük | 2–4 |
| F21 | MLINE, REGION ve HELIX #510'dan beri nesne, eksiklerle: MLEDIT T bağlantıları kare kesiyor; UNION/SUBTRACT/INTERSECT yalnızca sınırları birbirine değmeyen bölgeleri birleştiriyor; bölgeler kapalı poliline olarak kaydediliyor (ACIS verisi yok); PURGE çok satırlı stilleri atlıyor | `draw2/mline.rs`, `inquiry/regions.rs`, `dxf_region.rs` | Düşük | 6–10 |
| F22 | Sorgu artıkları (alanlar ve iç içe bloklar arasında COUNT, QuickCalc paleti ve COMPARE geldi, #563; bölgelerde MASSPROP, #510): COMPARE gösterilen sekmeyi karşılaştırır, tüm yerleşimleri aynı anda değil, ve COMPAREEXPORT raporu yok; QuickCalc geçmişini ve değişkenlerini yalnızca oturum boyunca tutar | `file/compare.rs`, `palettes/quickcalc.rs` | Düşük | 3–5 |
| F23 | Sheet Set Manager, eTransmit, Share, Traces, Markup Import, Activity Insights yok | kapsanmayan menü öğeleri | Düşük-orta (ekip iş akışları) | 40–70 |
| F24 | 3B yok: UCS, 3B görünümler, yörünge, katılar, yüzeyler, ağlar, kesitler, görsel stiller, malzemeler, ışıklar, işleme (150 menü öğesi) | [kontrol listesi](parite-kontrol-listesi.md) | Orta (3B kullanıcıları) | 300–450 |

## Kullanıcı arayüzü ve deneyimi (UI/UX)

Ayrıntı ve kanıt: [arayuz-paritesi.md](arayuz-paritesi.md).

Ana hatlı dolgu vurguları artık işaretçi yeniden çizimleri ve GPU yeniden oluşturmaları arasında
deterministik kontur başlangıçlarını ve yönlerini koruyor; bu, kesikli önizlemenin titriyor gibi
görünmesini önlüyor (U6).

Options ▸ Selection artık bir seçim penceresi için iki ayrı tıklama gerektirebiliyor (PICKDRAG 0)
veya tıklamalara ve sürüklemeye izin verebiliyor (2, varsayılan), kullanıcı profilinde saklanıyor.
Yalnızca sürükleme modu (1) kullanılamıyor.
| # | Eksik | Kullanıcı etkisi | Tah. |
|---|---|---|---|
| U4 | Sekme yakalama döngüsü her modun en yakın yakalamasını sunar, her adayı değil. Yazılan yakalama geçersiz kılmaları (`END`, `MID` … `NON`), Shift/Ctrl+sağ tık yakalama menüsü ve Sekme döngüsü geldi (#393), menünün Temporary track point'i #441'de | Düşük | 1 |
| U5 | Tutma noktası menüleri yalnızca sağ tıkla açılıyor (üzerine gelme menüsü yok, seçenekler arasında dönmek için Ctrl yok); ölçülerde, MLEADER'da, taramalarda veya metinde çok işlevli tutma noktaları yok; bir yay segmentindeki Add Vertex onu düzleştiriyor. Sıcak tutma noktası seçenekleri (Base point, Copy, Undo, Reference, eXit), birden çok sıcak tutma noktası ve poliline/çizgi/yay tutma noktası menüleri geldi (#425) | Düşük-orta | 2–4 |
| U6 | Seçim: kement; CLass anahtar sözcüğü; çizimle birlikte kaydedilen izolasyon (OBJECTISOLATIONMODE 1) ve durum çubuğu izolasyon düğmesi. Bas-sürükle pencereleri (#63), W, C, F, WP ve CP anahtar sözcükleri (#226), BOX, AUto, SIngle, Multiple, Group, Add, Remove ve Undo, SELECTSIMILAR, ISOLATEOBJECTS/HIDEOBJECTS/UNISOLATEOBJECTS (#428) ve seçim döngüsü (#1143) geldi | Orta | 3–5 |
| U7 | Sağ tık menüleri: yerinde yerleşim sekmesi yeniden adlandırma veya Move or Copy iletişim kutusu yok, Select All Layouts yok, komut satırına Yapıştır yok; palet menüleri yalnızca Katmanlar satırlarında ve Özellikler arka planında. Komut menüsü (basılı sağ tık: Enter, Cancel, Recent Input, seçenekler, Snap Overrides, Pan/Zoom), nesne düzenlemeleri içeren seçim menüsü, komut satırı, çizim sekmesi, yerleşim sekmesi ve palet menüleri geldi (#512) | Düşük | 1–2 |
| U10 | SETVAR yaklaşık 1,000 değişkenin yaklaşık 150'sini kapsıyor (UNDO seçenekleri, MREDO ve UNDOCTL ve CIRCLERAD, OFFSETDIST, POLYSIDES, HPNAME gibi başlık değişkenleri ve SETVAR ? içindeki ölçü değişkenleri geldi, #461; çıplak bir değişken adını komut olarak yazmak SETVAR'ı onun üzerinde çalıştırır) | Düşük-orta | 3–5 |
| U11 | MVIEW Restore (yerleşime kaydedilmiş bir görünüm penceresi yapılandırması); VIEW Orthographic, UCS ve arka plan ayarları; görünüm burulması saklanıyor ama gösterilmiyor (Zoom Realtime ve Dynamic, ZOOMFACTOR, adlandırılmış görünümler (VIEW) ve kaydedilmiş yapılandırmalı döşenmiş model görünüm pencereleri (VPORTS) #463'te geldi) | Düşük | 1–2 |
| U12 | Çalışma alanları, gezinti çubuğu, CUI özelleştirmesi, ViewCube sürükleme (#7) (Komut Takma Adları iletişim kutusu geldi, #1381) | Düşük-orta | 13–23 |
| U13 | macOS'ta Finder'dan `.dxf` açılamıyor (#158) (pencere denetimleri düzeltildi, #161) | Orta | 1–2 |

## Dosya biçimleri

Ayrıntı: [dosya-bicimi-paritesi.md](dosya-bicimi-paritesi.md).

| # | Eksik | Kullanıcı etkisi | Tah. |
|---|---|---|---|
| FF1 | **[alfa engelleyici]** Modellemediğimiz şeyleri gidiş-dönüşte koru. DXF tamam (#528): bilinmeyen varlık ve nesneler (CLASS kayıtlarıyla), yabancı xdata (köprüler dahil), uzantı sözlükleri, kalıcı reactor'lar, DICTIONARYVAR ve XRECORD'lar, katman durumları, renk kitabı renkleri, kaydetmede yeniden eşlenen başvurular; IMAGE, VIEW/UCS, gruplar, blok açıklamaları, gerçek renkler, saydamlık (#201, #204, #208, #209, #213, #223); WIPEOUT ve gradyanlar (#147, #162). Kalanlar: DWG üzerinden acadrust özel sınıf varlık ve nesneleri, tablo kayıtlarındaki ve bloklardaki xdata'yı, varlık renk kitabı adlarını ve reactor'ları düşürüyor; UCS yüksekliği; MTEXT arka plan renkleri; bilinmeyen varlıklar yalnızca katman düzenlemelerini izliyor ve yeniden adlandırılan katmanlar ve bloklar yabancı verilerini kaybediyor | Yüksek | 6–10 |
| FF2 | Çoklu kılavuz artıkları (yerel MULTILEADER okuma/yazma geldi, #459): çoklu kılavuz başına birkaç landing (ek kılavuz kökleri çizgilere katlanır), nesne başına geçersiz kılmalar (kılavuz rengi ve çizgi tipi, ok ucu sembolü, metin çerçevesi), not ölçek bağlamları; MULTILEADER çıktımızın metin yerleşimini AutoCAD'de kontrol edin | Orta | 3–5 |
| FF3 | Kaydetme sürümü artıkları (DXF/DWG 2000–2018, ikili DXF, SAVEAS `version`/`binary` ve varsayılan kaydetme sürümü geldi, #421): R14 DWG veya R12 DXF çıktısı yok; Kaydet iletişim kutusu dosya başına sürüm seçemiyor (dosya seçici seçilen türü bildirmiyor; File > Default Save Format belirler); komut satırı SAVEAS biçim istemi yok | Düşük | 2–4 |
| FF4 | Kod sayfası artıkları (`$DWGCODEPAGE` içindeki 2007 öncesi DXF okuma geldi, #505): Johab (ANSI_1361) ve DOS 855/857/860–865/869, ANSI_1252 olarak okunuyor; kaydetme dosyanın kod sayfası yerine `ANSI_1252` bildiriyor (metin saf ASCII olduğundan yine de doğru); acadrust 2007 öncesi DWG'de DOS kod sayfalarını yaklaşık olarak ele alıyor (DOS437'yi 866, DOS850'yi 1252 olarak) | Düşük | 1–2 |
| FF5 | **[alfa engelleyici]** Gerçek dosya korpusu (`storytold/cadcraft-corpus`), aç → kaydet → yeniden aç testleri, AutoCAD'de kara kutu kontrolü | Yüksek | 10–15 + insan |
| FF6 | **[alfa engelleyici]** DWG verimi: 10 MB'lık bir DWG'nin başarısız olması ~2 dk sürdü (#57) | Gerçek projeler için yüksek | 6–10 |
| FF7 | Diğer programların dosyalarındaki tablo kaydı ve nesne tanıtıcıları ilk kaydetmede yeniden numaralandırılıyor (varlık tanıtıcıları korunuyor; `*D` blokları artık çoğalmıyor ve bir kez yeniden açıldıktan sonra SPCAD'in kendi dosyaları tüm tanıtıcıları kaydetmeler arasında koruyor, #495) | Düşük | 4–6 |
| FF8 | SHX yazı tipleri (şekil ve büyük yazı tipleri) okunmuyor | Orta-yüksek (metin ölçüleri farklı) | 8–12 |
| FF9 | `.pc3` çizici yapılandırma dosyaları; çizgi tipi şekil (SHX) öğeleri atlanıyor (şablonlar ve `.dwt`, kullanıcı `.lin`/`.pat` ve `.pgp` takma ad dosyaları geldi, #552; `.ctb`/`.stb`, #410) | Düşük-orta | 5–10 |
| FF10 | PDF içe aktarma ve altlıklar (PDFATTACH); raster görüntü ekleme (IMAGEATTACH). PDF dışa aktarımında gömülü TrueType alt kümelerinde gerçek metin, PDF katmanları, dosya başına birkaç sayfa ve köprüler var (#567, #571) | Orta | 20–35 |
| FF11 | DWF/DWFx, DGN, WMF, EPS, ACIS/STL ve diğer değişim biçimleri | Düşük-orta | 50–100 |
| FF12 | Web derlemesinde DWG | Orta | 6–10 |
| FF13 | R12 ikili DXF, R13 öncesi DWG | Düşük | 10–20 |
| FF14 | SVG içe aktarma artıkları (şekiller, yollar, gruplar ve Inkscape katmanları geldi, #636): CSS sınıfları (`<style>`), `use` başvuruları, dolgular ve gradyanlar, metin veya gömülü görüntüler yok; yuvarlatılmış dikdörtgen köşeleri (`rx`/`ry`) yok sayılıyor. Böyle bir dosyayı açmak nelerin dışarıda bırakıldığını adlandırır (#627) | Düşük-orta | 2–4 |

## Donanım

Ayrıntı: [donanim-paritesi.md](donanim-paritesi.md).

| # | Eksik | Tah. |
|---|---|---|
| H1 | 3Dconnexion SpaceMouse (aygıtı ve 3B görünümleri gerektirir) | 6–10 + insan |
| H2 | Çoklu monitör: diğer ekranlarda yüzen paletler, monitör başına DPI (Windows 11 DPI için PR #110) | 4–6 |
| H3 | PDF dosyalarının ötesinde çiziciler ve sistem yazıcıları (PC3 aygıtları) | 6–10 |
| H4 | Kalem tabletleri / sayısallaştırıcı (TABLET) | 3–5 + insan |
| H5 | Ekran büyüteci izleme (#39, PR #187) | 1–2 |

## Yerelleştirme

Ayrıntı: [yerellestirme-paritesi.md](yerellestirme-paritesi.md).

| # | Eksik | Tah. |
|---|---|---|
| L1 | Katalog arayüzü kapsıyor (989 satır, #36) ama motor tanılamalarını, CLI/MCP yanıtlarını veya komut geçmişini kapsamıyor | 4–6 |
| L2 | Mac için AutoCAD'in diğer dilleri (zh-Hans, es, fr, ja, ko, it; de 2026-10-11'de geldi) artı standardın listesinden pt, hi, ar, id, vi | 35–55 + yerel inceleme |
| L3 | Çizim metninde Devanagari ve diğer Hint yazıları için karmaşık betik şekillendirme; RTL (aynalanmış) arayüz. Çizim metninde Arapça betik şekillendirme ve çift yönlü metin geldi (#287) | 20–30 |
| L4 | Her platformda IME düzeltmeleri (PR'lar #110–#119); dikey yazı stilleri | 6–10 |

## Kararlılık

| # | Eksik | Kanıt | Tah. |
|---|---|---|---|
| S1 | Hiçbir CI, PR'larda `cargo xtask ci` çalıştırmıyor (yalnızca paketleme lint'i ve sürüm derlemeleri) | `.github/workflows/` | 2–3 |
| S2 | ~~`cargo fmt --check` main'de başarısız~~ main'de düzeltildi (8fceaaf) | — | 0 |
| S3 | 40 açık PR, birçoğu inceleme bekleyen topluluk hata düzeltmeleri | `gh pr list` | 10–20 |
| S4 | Açık doğruluk sorunları: kapsam birleşimi uzak geometri (#166), komut ortasında görünmeyen çizgiler (#44), tarayıcı çizimi ham PDF baytları gösteriyor (#134) (sekme kapatma ve tarayıcıda kaydetme düzeltildi, #76, #133) | sorunlar | 4–6 |
| S5 | Arayüz üzerinden her komutun duman testi yok (PR #42 bir tane öneriyor) | — | 6–10 |

## Platformlar

| # | Eksik | Tah. |
|---|---|---|
| P1 | Web derlemesi: çizim ham PDF baytları gösteriyor (#134), DWG yok (File > Open ve kaydetme indirmeleri düzeltildi, #55, #133) | 3–5 |
| P2 | Windows ve Linux çok az çalışma zamanı testi alıyor; Linux Mint fare gecikmesi (#25); Windows büyüteci (#39) | 6–10 |
| P3 | macOS dosya ilişkilendirmeleri: Finder'dan `.dxf` açma başarısız (#158) | 1–2 |

## Ekosistem ve otomasyon

| # | Eksik | Tah. |
|---|---|---|
| E1 | Çekirdeğin ötesinde AutoLISP (#758 dili, `command`, varlık listeleri, seçim kümeleri, `get…` girdisi, APPLOAD, `(load)`, `c:` komutları ve başlangıçta `acad.lsp`/`acaddoc.lsp` getirdi): Visual LISP ActiveX (`vlax-`, `vla-`), DCL iletişim kutuları, reactor'lar, Startup Suite içeren bir APPLOAD iletişim kutusu, UCS farkındalıklı `trans` | 30–50 |
| E3 | Express Tools eşdeğerleri (SUPERHATCH; NCOPY, BREAKLINE, TXT2MTXT, BURST ve MOCORO var: BREAKLINE'ın Block seçeneği bir `.dwg` dosyası değil çizimin bir bloğunu alır ve TXT2MTXT'nin Settings'i iletişim kutusu yerine komut satırı istemleridir, tek tip satır aralığı seçeneği olmadan) | 24–40 |
| E4 | Eklenti API'si (sahip kararı: ObjectARX/.NET yerine Rust/WASM eklentileri) | 30–50 |
| E5 | CUI özelleştirmesi (takma ad düzenleyici, ALIASEDIT'in Komut Takma Adları iletişim kutusu geldi, #1381; düzenlemeler oturum boyunca kalır) | 13–23 |

## Yapay zekâ özellikleri

| # | Eksik | Tah. |
|---|---|---|
| A1 | Smart Blocks eşdeğerleri (yerleştirme önerileri, değiştirme, algılama ve dönüştürme, arama), yerel açık modeller kullanarak | 25–50 + model lisanslama |
| A2 | Uygulama içi asistan (AutoCAD'de Autodesk Assistant var); SPCAD'in MCP sunucusu zaten herhangi bir ajanın onu yönetmesine izin veriyor, geçerli seçimin salt okunur yapılandırılmış sorguları da dahil (`query_entities selected:true`) | 10–20 |
| A3 | Markup Assist (PDF/görüntülerden işaretlemeleri içe aktarma) | 10–20 |

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-16 | küçük | ALIASEDIT yazılarak veya Tools ▸ Customize'dan Komut Takma Adları iletişim kutusunu açar (kullanıcı takma adları yerleşik olanların yanında; Add, Edit, Remove; OK, Cancel, Apply), #1381: U12 ve E5 daraltıldı |
| 2026-10-16 | küçük | MOCORO (Tools ▸ Express Tools ▸ Move/Copy/Rotate): nesneleri ve bir taban noktasını bir kez seçin, sonra eXit'e kadar Move, Copy, Rotate, Scale, Base ve Undo, tek geri alma adımı; JSON biçimi `{base, ops}`: E3 daraltıldı |
| 2026-10-16 | küçük | MLEDIT Merged Tee (MT): ilk çok satırın dış öğeleri ikincinin yakın tarafında durur, bu taraf açılır ve iç öğeleri ikincinin iç öğesine kadar devam eder: F21 daraltıldı, saatler değişmedi |
| 2026-10-15 | küçük | BURST (Tools ▸ Express Tools ▸ Explode Attributes to Text): blok başvuruları EXPLODE'un yaptığı gibi patlar, görünür nitelik değerleri niteliğin stili, yüksekliği, konumu, katmanı ve rengiyle metne dönüşür, görünmez nitelikler ve tanımlar gider: E3 daraltıldı |
| 2026-10-13 | küçük | Sınır geçiş sırasını kanonikleştirerek dolu nesne vurgu çizgi çapalarını kararlı hale getirdi; işleyici, CPU işaretçi çerçevesi ve GPU yeniden oluşturma testleri düzeltmeyi kapsıyor (U6) |
| 2026-10-14 | küçük | HATCHEDIT, iç noktalardan veya kapalı nesnelerden sınır döngüleri ekler ve seçilen döngüleri kaldırır; iletişim kutusunda, komut satırında ve JSON komutunda: F13 daraltıldı |
| 2026-10-15 | küçük | `entities` / MCP `query_entities` `selected:true` kabul ediyor, mevcut filtreler ve sayfalamayla birleştirilebiliyor; seçim, çizim ve geri al/yinele değişmeden kalıyor. Parite tahmininde değişiklik yok. |
| 2026-10-15 | küçük | BREAKLINE (Tools ▸ Express Tools ▸ Break-line Symbol): iki nokta arasında kırılma sembollü bir poliline, Block/Size/Extension seçenekleri, boyut çarpı DIMSCALE: E3 daraltıldı |
| 2026-10-15 | küçük | TXT2MTXT (Tools ▸ Express Tools ▸ Convert Text to Mtext): seçilen TEXT ve MTEXT, yukarıdan aşağıya veya seçim sırasıyla, isteğe bağlı sözcük kaydırmayla tek bir MTEXT'te birleştirilir veya her TEXT için bir MTEXT; SEttings oturum boyunca hatırlanır: E3 daraltıldı |
| 2026-10-13 | küçük | Seçim penceresi hareket tercihi (PICKDRAG 0/2) Options ▸ Selection'a eklendi ve profilde saklanıyor (U6), Portekizce ve Ukraynaca etiketlerle |
| 2026-10-12 | küçük | Nesne saydamlığı (varlık, ByLayer, ByBlock) çiziliyor: ekranda (TRANSPARENCYDISPLAY, varsayılan açık), PNG/SVG dışa aktarımlarında ve sayfa kurulumunun "Çizim saydamlığı" seçeneği açıkken çizimlerde (PDF alfa, PNG karıştırma, SVG opaklık; PLOTTRANSPARENCYOVERRIDE): F14 daraltıldı |
| 2026-10-12 | küçük | Hatch and Gradient iletişim kutusu (HATCH, GRADIENT; HPDLGMODE) ve Hatch Edit iletişim kutusu (HATCHEDIT, sağ tık, bir taramaya çift tık): F13 Önizleme, Inherit Properties ve düzenleme modunda sınır ekleme/kaldırma ile daraltıldı |
| 2026-10-12 | küçük | Komut satırında yazılan çıplak bir sistem değişkeni adı (ORTHOMODE, GRIDMODE…) o değişkende SETVAR'ı çalıştırır, AutoCAD'de olduğu gibi; U10 daraltıldı |
| 2026-10-13 | küçük | STRETCH, çapraz ve pencere çokgenlerini (CP, WP), BOX ve bir çiti (F) alır, yazılı ve JSON olarak (`polygon`, `fence`): F1 daraltıldı |
| 2026-10-12 | küçük | Geometri sayısal değerleri için özellik tabanlı ve düşmanca girdi testleri geldi (deterministik xorshift üreteci, yeni bağımlılık yok; 1e6–1e7 koordinatlarda bilinen yanıtlar): S6 kaldırıldı |
| 2026-10-12 | önemsiz | F13: HATCHEDIT düzenlemeden önce hatalı bir ölçeği veya açıyı reddeder (#724) |
| 2026-10-12 | küçük | F3: dinamik blok başvuruları tutma noktalarını gösterir (mesafe okları, konum kareleri, görünürlük/arama listeleri, görünürlük durumu başına) ve sürükler (`blocks.reference.grips`, `blocks.reference.grip`) |
| 2026-10-17 | küçük | AUDITCTL sistem değişkeni ve AUDIT/RECOVER'da .adt günlük üretimi geldi: F4 daraltıldı (.adt günlüğü eksikliği kaldırıldı) |
| 2026-10-12 | küçük | Çok satırlı nitelikler okunuyor, çiziliyor ve kaydediliyor (önce metinlerini kaybediyorlardı): F17 oluşturma ile daraltıldı |
| 2026-10-13 | küçük | F3: bir dinamik bloğun kendi çizimini açmak (model alanında yazarlık öğeleri) AutoCAD'in yaptığı gibi `<Current Drawing>` üzerinde Blok Düzenleyici sunar |
| 2026-10-12 | küçük | Edit Block Definition iletişim kutusu (blok listesi, önizleme, açıklama; yeni bir ad yeni bir blok başlatır): F3 metni güncellendi, saatler değişmedi |
| 2026-10-12 | küçük | Statik bloklar için Blok Düzenleyici (BEDIT, BSAVE, BSAVEAS, BCLOSE) geldi: 4. satır (65–100 → 55–90 sa) ve F3 (50–75 → 40–65 sa) REFEDIT ve dinamik bloklarla daraltıldı |
| 2026-10-12 | küçük | Biçimlendirme araç çubuğu ve girinti/sekme cetveliyle yerinde MTEXT düzenleyici geldi (#532): F9 düzenleyici artıkları, alanlar ve yazım denetimiyle daraltıldı |
| 2026-10-12 | küçük | `.scr` dosyalarını okuyan SCRIPT komutu geldi (bir dosyayı CLI/MCP ile aynı betik motorundan geçirir; iç içe betikler reddedilir): E2 kaldırıldı |
| 2026-10-11 | küçük | Birleştirilen özellikler için yenilendi: kod sayfaları (#505), izleme artıkları (#441), dinamik girdi/tutma noktası girdisi (#519, #425) ve SPLINEDIT Join/Add/Elevate (#559) üst-10 satırlarını kapattı (üst 10, bir sonraki inceleme yeniden sıralayana kadar 6 satır); MULTILEADER (#459) ve xref'ler (#525) 1. ve 4. satırları daralttı; F1 kalan taslaklarla daraltıldı (#558, #559, #547); F5, F7, F9, F12, F14, F15, F17, F20, F22, U5, U7, U10, FF2, FF4, FF10 daraltıldı; F18, U2, U8, U9 kaldırıldı (tamam); U13, S3, S4, P1 birleştirilen düzeltmeler için güncellendi |
| 2026-10-11 | küçük | Almanca arayüz kataloğu geldi: L1 satır sayısı 936 → 989, L2 daraltıldı (de tamam; 40–60 → 35–55 sa) |
| 2026-10-11 | küçük | SPLINE Knots/Tangency/toLerance/Object, MTEXT Height/Justify/Line spacing/Rotation/Style/Width/Columns, MLEADER spline ve kılavuzsuz türler ve blok içeriği geldi (#408): üst-10 5. satır, F1 ve F10 daraltıldı |
| 2026-10-11 | küçük | Nesne yakalama izleme, Extension, Parallel ve gerçek Apparent Intersection geldi (#379): üst-10 3. satır ve U1 artıklarla daraltıldı |
| 2026-10-10 | küçük | Main birleştirildi: Ukraynaca katalog, UNITS iletişim kutusu, kısayol gösterimi, dinamik girdi işaretçi kutuları, bas-sürükle pencereleri, SETVAR DIM*, WIPEOUT ve gradyanlar, fmt için kapatılan veya daraltılan satırlar |
| 2026-10-10 | küçük | Alfa engelleyiciler işaretlendi (çekirdek iş akışı kapısından geçemeyen satırlar) |
| 2026-10-10 | büyük | İlk eksik listesi: sıralanmış ilk 10 ve boyuta göre bilinen her eksik, kanıt ve tahminlerle |
