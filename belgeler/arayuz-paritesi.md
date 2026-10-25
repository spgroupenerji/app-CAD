# Arayüz paritesi

> **Son gözden geçirme:** 2026-10-22 · **Son güncelleme:** 2026-10-22 · **Değişiklik:** orta (Araç Rafı: sürükle-bırak, iki taraf, kalıcı) · **Hedef:** Autodesk AutoCAD 2027 for Mac (26.0)

SPCAD'nin etkileşiminin AutoCAD'inkiyle nasıl karşılaştırıldığı: nokta girişi, nesne kenetlemeleri,
izleme, dinamik giriş, tutamaçlar, seçim, komut satırı, pencere çerçevesi ve gezinme. Bir çizerin
AutoCAD'deki hızı bunlardan gelir, bu yüzden "komut var" yeterli değildir: aynı girdiyi kabul
etmeli, aynı noktalara kenetlenmeli ve aynı tuşlara yanıt vermelidir. İş maddeleri:
[eksikler.md](eksikler.md#kullanıcı-arayüzü-ve-deneyimi-uiux).

**Özet (tahmini): UI/UX sadakati ≈ 48%.** Komut satırı, nokta sözdizimi, yaygın çalışan kenetlemeler,
kutupsal/orto, nesne kenetleme izleme ve paletler gerçektir; Extension, Parallel ve Apparent
Intersection kenetlemeleri de öyle (#379); dinamik giriş salt okunur bir araç ipucudur; FROM, M2P,
TK ve nokta filtreleri yoktur; sıcak tutamaç seçenekleri duyurulur ama yok sayılır; bağlam menüleri
yoktur. Kalan: **≈ 110–170 Opus 5.5 saat**.

## Bu nasıl ölçüldü

- SPCAD: 2026-10-10'da kaynaktan okundu (main, bd2412f): `kitapliklar/motor/src/{prompt,
  snap, select, grips, lib, units}.rs`, `cmd/*`, `kitapliklar/arayuz-egui/src/*`. Bu geçişte çalıştırılmadı.
- AutoCAD: davranış, çalışan AutoCAD 2027 for Mac'te kara kutu olarak gözlemlendi
  (`plan/autocad/01-observed-ui.md`, yalnızca yerel, asla commit edilmeyen referans ekran
  görüntüleriyle), menü ağacı (`xtask/veri/menu-catalog.txt`) ve Autodesk'in belgeleri.
- Ağırlıklar (bir çizerin her alana ne sıklıkla dokunduğu): nokta girişi 15%, nesne kenetlemeleri
  15%, izleme ve kutupsal 10%, dinamik giriş 10%, tutamaçlar 10%, seçim 10%, komut satırı 10%,
  pencere çerçevesi ve paletler 10%, gezinme 5%, klavye ve bağlam menüleri 5%.

| Alan | Ağırlık | SPCAD | Ağırlıklı |
|---|---:|---:|---:|
| Nokta girişi | 15% | 45% | 6.8 |
| Nesne kenetlemeleri | 15% | 45% | 6.8 |
| İzleme, kutupsal, orto, ızgara/kenetleme | 10% | 70% | 7.0 |
| Dinamik giriş | 10% | 45% | 4.5 |
| Tutamaçlar | 10% | 40% | 4.0 |
| Seçim | 10% | 40% | 4.0 |
| Komut satırı | 10% | 60% | 6.0 |
| Pencere çerçevesi ve paletler | 10% | 45% | 4.5 |
| Gezinme | 5% | 55% | 2.8 |
| Klavye kısayolları, bağlam menüleri | 5% | 25% | 1.3 |
| **Toplam** | 100% | | **≈ 48%** |

## Nokta girişi (≈ 45%)

| Girdi | AutoCAD | SPCAD | Kanıt |
|---|---|---|---|
| `x,y[,z]`, `@dx,dy`, `@d<a`, `d<a`, `#x,y`, yalın `@` | evet | evet; UCS olmadığı için `#` mutlak ile aynıdır; Z ayrıştırılır sonra atılır | `prompt.rs:147-180` |
| `*x,y` (WCS geçersiz kılma) | evet | evet (`*x,y`, `*@dx,dy`, `@*dx,dy`); UCS olmadığı için düz biçimlerle aynı | `prompt.rs` |
| Doğrudan mesafe girişi | evet, orto, kutupsal ve izleme yollarına uyar | evet, orto; kutupsal yalnızca UI'nin kenetlenmiş imleci aracılığıyla | `lib.rs:782-793` |
| Açı biçimleri: `45d30'`, radyan, grad, ölçüm `N45dE` | evet | evet | `units.rs:141-197` |
| Mimari/mühendislik mesafeleri `5'6-1/2"` | evet | evet | `units.rs:93-140` |
| `FROM` (temel nokta + öteleme) | evet | evet: "Base point:", "<Offset>:" ve `@` temel noktadan ölçülür; M2P ve filtrelerle iç içe geçer | `pointmod.rs` |
| `M2P` / `MTP` (iki nokta arasındaki orta nokta) | evet | evet | `pointmod.rs` |
| `TK` (izleme) | evet | **hayır** | |
| Nokta filtreleri `.x`, `.y`, `.xy`, `.z` | evet | evet (`.x` … `.yz`, "(need YZ):"); Z sorulur ve atılır (3B yok) | `pointmod.rs` |
| Açı geçersiz kılma `<45` | evet | evet: `<a`, `<<a`, `<<<a`; yazılan mesafe veya bir seçim açıyı izler, yazılan koordinatlar kazanır | `pointmod.rs` |
| Nesne kenetleme geçersiz kılmaları (`END`, `MID`, `INT` … `NON`) | evet | evet, her kip ve `NON`, uzun adlarla ve `_`/`'` önekleriyle, yalnızca sonraki seçim için; yazılan bir nokta açıklık içinde kenetlenir; komutun kendi anahtar sözcüğü kazanır (`_cen` her zaman kenetlenir); kenetleme menüsünden de | `pointmod.rs` |
| Yazılı hesap makinesi `'CAL` | evet | hayır | |

## Nesne kenetlemeleri (≈ 45%)

| Kip | AutoCAD | SPCAD (`snap.rs:135-334`) |
|---|---|---|
| Uç nokta | her eğri, katı/3B köşeler, ölçü uçları | çizgiler, yaylar, çoklu çizgi parçaları, açık elipsler ve spline'lar, ışın tabanı, SOLID/TRACE köşeleri |
| Orta nokta | elips/spline yayları dahil her eğri, çoklu çizgi parçaları | yalnızca parçalar; **çemberlerde, elipslerde, spline'larda yok** |
| Merkez | yaylar, çemberler, elipsler, çoklu yaylar | yaylar, çemberler, elipsler |
| Geometrik merkez | herhangi bir kapalı çoklu çizgi, spline | yalnızca kapalı LWPOLYLINE |
| Düğüm | noktalar, ölçü tanım noktaları, yazı hizalama noktaları | yalnızca POINT |
| Çeyrek | çemberler, yaylar, elipsler | evet |
| Kesişim | her çift, uzatılmış kesişim | en fazla 200 yakın ilkel arasında; elips/spline çiftleri yaklaşık ([geometri-paritesi.md](geometri-paritesi.md)) |
| Uzatma | evet, kesikli uzatma yoluyla | evet: bir çizgi veya yay ucunda dur, sonra çizgi ilerler (bir ışın) ve yay çemberi etrafında döner; noktalı yol ve "Extension: d < a°" (`snap/tracking.rs`); elips, spline veya çoklu çizgi yay uçlarından değil |
| Ekleme | yazı, mtext, bloklar, nitelikler, attdef'ler | TEXT, MTEXT, INSERT; ATTDEF veya MLEADER yok |
| Dik | her eğri, ertelenmiş | çizgiler, yaylar, çemberler, xline'lar; ilk nokta için ertelenmiş (#168) |
| Teğet | çemberler, yaylar, elipsler, spline'lar, ertelenmiş | yaylar ve çemberler; ilk nokta için ertelenmiş; **elips veya spline yok** |
| En yakın | her eğri | evet |
| Görünür kesişim | projeksiyonlu kesişim | 2B'de evet: imlecin yakınındaki nesnelerin uzatılsaydı buluşacağı yer; üzerinde durulan bir nesne tüm uzatmasını gönderir, böylece başka bir nesneyle kesişimi de kenetlenir (`snap.rs` `osnap_with`, `snap/tracking.rs`) |
| Paralel | evet, paralel yolla | evet: bir çizgide dur, sonra ona paralel bir yol temel noktadan geçer (`snap/tracking.rs`) |
| İstemde yazılan geçici geçersiz kılmalar (`END`, `MID`, `INT` …) | evet | **hayır**: metin düz girdi olarak gelir |
| Shift/Ctrl+sağ tık kenetleme menüsü | evet | evet: From, Mid Between 2 Points, Point Filters, kenetleme kipleri, None, Osnap Settings; henüz Temporary track point yok (`context_menu.rs`) |
| Aday kenetlemeler arasında geçmek için Tab | evet | evet, imleç altındaki her çalışan kipin en yakın kenetlemesi arasında (`pointmod::snap_candidates`) |
| AutoSnap işaretçisi, araç ipucu, mıknatıs, açıklık kutusu | evet | kip başına işaretçi ve ad etiketi; açıklık APERTURE'dan; mıknatıs yok, açıklık kutusu yok |
| OSNAPZ, 3B nesne kenetlemeleri | evet | hayır |

## İzleme, kutupsal, orto, ızgara ve kenetleme (≈ 70%)

| Özellik | AutoCAD | SPCAD |
|---|---|---|
| Orto (F8) | evet, basılı tutulduğunda Shift onu geçersiz kılar | evet; Shift basılı tutulduğunda onu açar veya kapatır (`Session::snap_cursor`, `snap/tracking.rs`) |
| Kutupsal izleme artışı | evet | evet, ANGBASE'den ölçülür; noktalı yol ve "Polar: d < a°" araç ipucu (`snap/tracking.rs`, `canvas.rs` `draw_tracking`) |
| Son parçaya göre ek kutupsal açılar | evet | ek açılar evet (POLARMODE 4 ile POLARADDANG); **son parçaya göre hayır** |
| Kutupsal mesafe kenetlemesi (PolarSnap) | evet | **hayır** |
| **Nesne kenetleme izleme** (noktaları edinme, hizalama yolları, yolların kesişimleri) | evet, F11 | evet (#379): bir kenetleme noktası üzerinde durmak onu edinir (`+`), tekrar durmak bırakır, en fazla yedi tane; yatay/düşey yollar veya POLARMODE 2 ile her kutupsal açı; POLARMODE 8 ile edinmek için Shift; iki yolun, bir yol ile bir nesnenin ve bir yol ile bir kutupsal yolun kesişimleri kenetlenir; araç ipucu her yolu adlandırır; yazılan bir mesafe (doğrudan mesafe girişi) imlecin üzerinde olduğu yol boyunca, yolun geldiği noktadan ölçülerek ilerler. Motor: `Session::snap_cursor`, `snap/tracking.rs`; AUTOSNAP bitleri 8 ve 16 |
| Izgara (dikdörtgen, ana çizgiler, uyarlanabilir, sınırlar) | evet | dikdörtgen ve izometrik (30°, 90° ve 150° çizgiler), ana çizgiler, uyarlanabilir görüntüleme |
| Kenetleme (dikdörtgen, izometrik, kutupsal kenetleme) | evet | dikdörtgen ve izometrik (SNAPSTYL 1: izometrik eksenler boyunca kenetleme Y aralığı), SNAPBASE'den SNAPANG ile döndürülmüş |
| İzometrik çizim (ISODRAFT, izodüzlemler F5) | evet | evet (#556): ISODRAFT ve durum çubuğu anahtarı SNAPSTYL ve düzlemi ayarlar; ISOPLANE / F5 / Ctrl+E Left, Top, Right arasında döner (SNAPISOPAIR); orto, doğrudan mesafe girişi ve artı işareti düzlemin eksenlerini izler; ELLIPSE Isocircle (yarıçap veya Diameter, eliptik yaylar için de); Drafting Settings kenetleme stili radyo düğmesi. Motor: `snap/iso.rs`, `cmd/settings/iso.rs` |
| Drafting Settings iletişim kutusu | evet | evet (`dialogs.rs:36-80`) |
| Tarama ve Degrade / Taramayı Düzenle iletişim kutuları | evet | evet (`hatchdlg.rs`): desen örnekleri, degrade renkleri ve biçimleri, orijin, nokta veya nesne seçimi, ilişkisel, ayrı, ada algılama; Taramayı Düzenle noktalardan veya kapalı nesnelerden sınır ekler ve seçili döngüleri kaldırır; HATCH, GRADIENT (HPDLGMODE), HATCHEDIT, sağ tık veya bir taramaya çift tıklama ile açılır; Preview veya Inherit Properties yok |

## Dinamik giriş (≈ 45%)

AutoCAD'in baş üstü girişi (F12) düzenlenebilir işaretçi giriş alanlarına (x, y), aralarında Tab ve
kilit simgeleri bulunan boyutsal giriş alanlarına (uzunluk ve açı), varsayılan olarak göreli ikinci
noktalara, tutamaçlarda boyutsal girişe, tıklanabilir seçenekleri olan bir isteme ve aşağı ok
menüsüne sahiptir. SPCAD'de #60'tan beri işaretçi giriş kutuları vardır
(`kitapliklar/arayuz-egui/src/dinamik_girdi.rs`): bir komut nokta isterken imlecin yanında değer kutuları; ikinci
ve sonraki noktalar varsayılan olarak kutupsal mesafe ve açıya (Kartezyen için DYNPIFORMAT = 1);
`,` ve `<` bir değeri kilitler ve sonraki kutuya geçer; Tab ayırıcıyı yazar; Enter göreli bir giriş
gönderir (mutlak için DYNPICOORDS = 1) ve boş bir kutuyu imleçten doldurur; düz bir sayı doğrudan
mesafe girişi olarak kalır. Eksik: tutamaçlarda boyutsal giriş (esnetirken uzunluk/açı alanları),
kilit simgeleri, aşağı ok seçenek menüsü olan istem (DYNPROMPT) ve DYNDIM.

## Tutamaçlar (≈ 40%)

| Özellik | AutoCAD | SPCAD |
|---|---|---|
| Nesne tipi başına esnetme tutamaçları | evet | çizgiler (orta nokta hareket eder), yaylar (başlangıç/orta/son), elipsler (eksenler), çoklu çizgiler (köşeler; parça orta noktaları kabarıklığı hareket ettirir veya yeniden biçimlendirir), ölçüler (yazı ve tanım noktaları), MLEADER, görünüm penceresi, wipeout, spline (`grips.rs:79-270`) |
| Yazı/MText tutamaçları | konum, genişlik, sütun tutamaçları | tüm nesneyi hareket ettirir |
| Space/Enter ile döngülenen sıcak tutamaç kipleri | Stretch, Move, Rotate, Scale, Mirror | evet (`canvas.rs:58-67`) |
| Sıcak tutamaç seçenekleri: Base point, Copy, Undo, Reference, eXit | evet | **istemde duyurulur ama ayrıştırılmaz**; motorun JSON tutamaç komutları `copy` kabul eder, UI bunu asla geçirmez |
| Aynı anda birkaç sıcak tutamaç (Shift+tık) | evet | **hayır** (tek tutamaç) |
| Çok işlevli tutamaç üzerine gelme menüleri (Add Vertex, Remove Vertex, Convert to Arc, Lengthen …) | evet | **hayır** |
| Tutamaç boyutsal girişi (uzunluk/açı alanları) | evet | hayır |
| Dinamik blok tutamaçları | evet | hayır (dinamik blok yok) |
| Tutamaç görünümü (GRIPSIZE, renkler, üzerine gelme) | evet | GRIPSIZE, GRIPS ve GRIPOBJLIMIT dikkate alınır (Options ▸ Selection); tutamaç renkleri yok |

## Seçim (≈ 40%)

| Özellik | AutoCAD | SPCAD |
|---|---|---|
| Seçim, örtük pencere (soldan→sağa) ve kesişen (sağdan→sola) | evet | evet, renkli ve kesikli dikdörtgenler (`lib.rs:576-589`, `canvas.rs:823-836`), SELECTIONAREA, WINDOWAREACOLOR, CROSSINGAREACOLOR ve SELECTIONAREAOPACITY'nin söylediği gibi doldurulur (Visual Effect Settings) |
| Anahtar sözcükler W, C, WP, CP, F, BOX, AU, SI, M, G (grup), CL (sınıf) | evet | CLass hariç evet: W, C, F, WP, CP (#226); BOX, AUto, SIngle, Multiple, Group, Add, Remove, Undo (#428); ALL, Last, Previous |
| Bas-sürükle seçim pencereleri (PICKDRAG) | evet | tıklamalar veya sürükleme (2, varsayılan) ya da yalnızca iki tıklama (0); Options ▸ Selection ▸ Select using two clicks only, profilde saklanır; yalnızca sürükleme kipi (1) kullanılamaz |
| Kement | evet | **hayır** |
| Seçim döngüsü (SELECTIONCYCLING) | evet | evet (#1143): SELECTIONCYCLING 2 bir tıklamanın altındaki nesneleri listeler (tip, katman; işaretçinin altındaki satır nesnesini vurgular), 1 rozet; Shift+Space vurguyu aralarında adımlar (#1145); Ctrl+W yok |
| Seçim önizlemesi, üzerine gelme vurgusu | evet | seçili ve üzerine gelinen nesneler seçim renginde yeniden çizilir, dolgular yarı saydamdır (SELECTIONEFFECT 1; 0 kesikli), seçim efekti rengi; HIGHLIGHT (kaydedilmez), SELECTIONPREVIEW, PREVIEWEFFECT, PREVIEWFILTER (Visual Effect Settings) |
| PICKADD, PICKFIRST, PICKAUTO | evet | PICKADD (Shift), PICKFIRST |
| Ana hatlı dolgu vurgularının kararlılığı | evet | kapalı konturlar yeniden çizimlerde ve GPU yeniden oluşturmalarında aynı başlangıcı, yönü ve sıralamayı korur; kesikli dolu glifler, tarama/katı bölgeler, ok uçları ve wipeout ana hatları artık işaretçi hareketiyle kaymaz |
| QSELECT, SELECTSIMILAR, FILTER | evet | QSELECT iletişim kutusu; SELECTSIMILARMODE ile SELECTSIMILAR (#428, SEttings bit kodunu sorar; iletişim kutusu yok); FILTER yok |
| Nesne yalıtma/gizleme (ISOLATEOBJECTS, HIDEOBJECTS) | evet | evet, geçici (OBJECTISOLATIONMODE 0; #428): Tools > Isolate, UNISOLATEOBJECTS; durum çubuğu düğmesi yok |
| Gruplar tek olarak seçilebilir (PICKSTYLE) | evet | evet (PICKSTYLE 1, #417), Options ▸ Selection üzerinde Object grouping; ilişkisel tarama seçimi (bit 2) saklanır, kullanılmaz |

## Komut satırı (≈ 60%)

| Özellik | AutoCAD | SPCAD |
|---|---|---|
| AutoComplete (komutlar, sysvar'lar, dize ortası, eşanlamlılar) | evet | Tab ile kabul edilen komut listesi (`cmdline.rs:30,176,317`); sysvar veya dize ortası araması doğrulanmadı |
| Takma adlar | `acad.pgp` içinde yaklaşık 250 | komutlar genelinde 144 takma ad (kaynaktan sayıldı) |
| Geçmiş (F2), Yukarı/Aşağı geri çağırma | evet | evet |
| Saydam komutlar (`'zoom`, `'pan`, `'osnap`) | evet | evet (`lib.rs:496-503`) |
| Enter/Space son komutu yineler | evet | evet |
| İstemde tıklanabilir anahtar sözcükler | evet | evet (`cmdline.rs:253-267`) |
| Mark, Back, BEgin, End, Auto, Control ile UNDO | evet | yalnızca sayım, tüm belge anlık görüntüleri 2,000 ile sınırlı (`lib.rs:477`, `edit.rs:56`) |
| Her sistem değişkeni için SETVAR | ~1,000 sysvar | adla yaklaşık 60 oturum/başlık değişkeni (`sysvars.rs`) |
| Betikler (SCRIPT, `.scr`) | evet | CLI/MCP/kontrol üzerinden betik metni; `.scr` dosyası okuyan komut yok |
| Yüzen/yerleşik komut penceresi, saydamlık, geçmiş satırları | evet | kenetli, tam genişlikte komut satırı: üstünde geçmiş satırları, altında istem; yükseklik üst kenardan sürüklenir, F2 tüm geçmişi açar |

## Pencere çerçevesi ve paletler (≈ 45%)

| Özellik | AutoCAD for Mac | SPCAD |
|---|---|---|
| Menü çubuğu | 11 menü, 491 yaprak öğe | aynı 11 menü; 245/491 öğe kapsanıyor (233'ü `cargo xtask parity` tarafından, yalnızca motor komutlarını sayar, artı 12 yalnızca UI öğesi: bkz. [parite-kontrol-listesi.md](parite-kontrol-listesi.md)); bir menü açıkken başka bir başlığı işaret etmek o menüyü açar (#1567) |
| Şerit (Giriş, Ekle, Açıklama, Parametrik, Görünüm, Yönet, Çıktı), Araç Çubuğu, Dosya sekmeleri, Başlangıç sekmesi, yerleşim sekmeleri | evet | şerit: yedi sekme; etkin sekmenin bütün panelleri her pencere genişliğinde görünür kalır (etiket boyutu tema ölçeğinden seçilir ve gerçek metin düzeniyle ölçülerek en fazla iki satıra kırpılmadan sarılır; sığmayan son büyük düğmeler panel başlığı menüsüne taşınır; 900 px'te paneller yalnız başlık olur) ve satır pencereyi doldurur; Özellikler panelinde dar şeritte geçerli renk (tek satır), geniş şeritte renk/çizgi tipi/ağırlık ızgarası; dosya ve yerleşim sekmeleri ayrı |
| Pencere başlığı ve sürüm | evet (çizim adı, ürün sürümü) | evet: üst çubukta `SP-GROUP CAD BETA_vYYYYMMDDHHMM` (derleme damgası UTC+3, ayırıcısız); About ve Başlangıç aynı beta etiketini gösterir |
| Paletleri kapatma/açma | evet (başlıktaki X, Ctrl+1) | evet: Katmanlar ve Özellikler başlıklarında X; üst çubuktaki palet simgesi, Window ▸ Palettes ve Ctrl+1 ile yeniden açılır |
| Araç Rafı (kullanıcının şeritten/menüden sürükleyip doldurduğu, iki tarafa açılan, kapanan ve genişliği değişen palet) | evet (Tool Palettes) | evet: varsayılan kapalı; `ui.shelf.left/right` ile açılır, başlıktan kapanır, kenetleme ayırıcısıyla genişler, araçlar `ui.shelf.add/remove/clear` ile yönetilir ve profilde saklanır; eski sabit Araç Takımları paleti kaldırıldı (işlevleri şerit ve menülerde) |
| Özellikler Denetçisi, Hızlı Özellikler | evet | evet (her denetim işler veya görünür biçimde kullanılamaz, #182); blok referansları saklanan ekleme noktalarından salt okunur Position X/Y/Z gösterir; koordinat düzenleme hâlâ eksik |
| Katman Özellikleri Yöneticisi (filtreler, durumlar, VP geçersiz kılmaları) | evet | evet |
| Bloklar paleti, Content | evet | Bloklar penceresi (eklemek için tıklayın, #28) |
| Durum çubuğu anahtarları | ~25 | çoğu anahtar; Annotation Visibility ve Workspace hiçbir şey yapmaz |
| ViewCube | etkileşimli 3B | 2B için tıklanabilir (Top/Home → sınırlar); sürükleme hiçbir şey yapmaz (#7) |
| Gezinme çubuğu, SteeringWheels | evet | hayır |
| Çalışma alanları | Drafting & Annotation, 3D Modeling | hayır |
| Options / Preferences iletişim kutusu | birçok sekme | evet: altı sekme (Görüntü, Açma ve Kaydetme, Çizim, Seçim, Kullanıcı Tercihleri, Profiller), referans düzeninde sekme şeridi, çerçeveli grup kutuları ve altta Tamam/Yardım; Plot ve Online sekmeleri yok |
| Özelleştirme (CUI, takma ad düzenleyici, araç paletleri) | evet | Komut Takma Adları iletişim kutusu (ALIASEDIT, #1381); CUI veya araç paletleri yok |
| Temalar | Dark, Light | Dark, Light, System (#179) |
| Erişilebilirlik | VoiceOver temelleri | eframe aracılığıyla AccessKit; ekran okuyucu kapsamı test edilmedi; büyüteç sorunu #39; animasyon efektleri işletim sistemi ayarını izler (View ▸ Accessibility ▸ Animation Effects: System, On, Off) |

## Gezinme (≈ 55%)

| Özellik | AutoCAD | SPCAD |
|---|---|---|
| ZOOM Extents, All, Window, Previous, In/Out, Center, Scale, Object | evet | evet, 50 girdilik geçmiş (`cmd/view.rs:107-149`) |
| ZOOM Realtime (sürükle), Dynamic | evet | **hayır** |
| İmleç çevresinde tekerlek yakınlaştırma, ZOOMFACTOR, ZOOMWHEEL | evet | sabit çarpan (ZOOMFACTOR yok) |
| Orta tuşla sürükleyerek kaydırma, orta tuşa çift tıklayarak sınırlar | evet | evet (`canvas.rs:538-583`) |
| İzleme dörtgeni iki parmak kaydırma, sıkıştırarak yakınlaştırma (macOS) | evet | evet (#58) |
| Adlandırılmış görünümler (VIEW) | evet | **hayır** (VIEW tablosu okunmaz veya yazılmaz) |
| Döşenmiş model alanı görünüm pencereleri (VPORTS) | evet | doğrulanmadı: VPORTS kâğıt alanı MVIEW'ına eşlenir |
| 3B yörünge, ViewCube sürükleme, SteeringWheels | evet | hayır (M11) |

## Klavye ve bağlam menüleri (≈ 25%)

- Kısayol tablosu: 28 girdi (Cmd kısayolları, F1, F2, F3, F5, F7–F12) artı 23 komut `.key()`
  kısayolu. AutoCAD for Mac'in varsayılanları sayıca benzerdir; F4 (3B osnap) ve F6 (dinamik UCS)
  eksik.
- Delete macOS'te seçimi siler (#64); Escape iptal eder.
- Bir komut çalışırken tuvalde sağ tık Enter'dır; basılı tutmak (250 ms, AutoCAD'in zamana duyarlı
  sağ tıkı) komut menüsünü açar: Enter, Cancel, Recent Input, istemin seçenekleri, Snap Overrides,
  Pan/Zoom. Aksi halde bir kısayol menüsü açar (Repeat, Recent Commands, Clipboard ▸, Isolate ▸,
  seçimin düzenleme komutları, Draw Order ▸, Group, seçili nesne tipinin düzenleme komutları,
  Select Similar, Deselect All, Undo/Redo, Pan/Zoom; #325, #512). Komut satırı, çizim sekmeleri,
  yerleşim sekmeleri, Katmanlar paleti satırları ve Özellikler paletinin de menüleri vardır (#512).
  Hâlâ eksik: yerinde yerleşim sekmesi yeniden adlandırma, Move or Copy, Select All Layouts.

## Revizyon geçmişi

| Tarih | Değişiklik | Özet |
|---|---|---|
| 2026-10-25 | orta | Şerit tipografisi kökten düzeltildi: etiket boyutu tema ölçeğinden seçilir (11.5 → 10.5 → 9.5 pt), genişlikler gerçek metin düzeniyle ölçülür ve her etiket en fazla iki satıra kırpılmadan sığar; sığmayan son büyük düğmeler panel başlığı menüsüne taşınır, hiçbir yazı kesilmez ve bitişik etiketler birleşmez; 16 dilde 1280/1415/1674 px kırpma testi ve etiket çakışma testi eklendi |
| 2026-10-25 | orta | Şerit artık sığmayan paneli gizlemiyor: paneller ölçülen genişliklere göre kademeli sıkışır (etiketli büyük düğmeler → ikon kolonları → yalnız başlık), artan genişlik içeriği en çok gizlenen panellere dağıtılır ve satır pencereyi doldurur; 900–1920 px arasında Giriş sekmesinin on grubu da görünür, tüm komutlar panel başlığı menüsünde kalır; 1280 px'te büyük düğme etiketleri korunur; kittest genişlik matrisi ve %125/%150 DPI testleri eklendi |
| 2026-10-13 | küçük | Doldurulmuş bölge vurgu sınırları deterministik koordinat sırasıyla dolaşılır; CPU üzerine gelme/seçim yeniden çizimlerinde ve GPU vurgu yeniden oluşturmalarında tire fazı sıçramaları düzeltildi; CPU tire konumu, piksel ve GPU ağı gerileme testleri |
| 2026-10-14 | küçük | Seçim döngüsü (#1143): SELECTIONCYCLING listesi ve rozeti, çakışan nesneler arasında Shift+Space (#1145) |
| 2026-10-14 | küçük | Taramayı Düzenle iletişim kutusu seçilen noktalardan veya kapalı nesnelerden sınır ekler ve seçili döngüleri kaldırır |
| 2026-10-14 | küçük | Shift basılı tutulduğunda orto'yu geçersiz kılar (ORTHOMODE kapalıyken açar, açıkken kapatır), AutoCAD'teki gibi |
| 2026-10-16 | küçük | ALIASEDIT, Komut Takma Adları iletişim kutusunu açar (#1381) |
| 2026-10-18 | küçük | Pencere içi menü çubuğu: bir menü açıkken başka bir başlığı işaret etmek onu kapatır ve o menüyü açar (#1567) |
| 2026-10-22 | orta | Üst çubukta ürün adı + `BETA_vYYYYMMDDHHMM` (UTC+3, ayırıcısız) gösterildi; Katmanlar/Özellikler paletlerine kapatma X'i ve paletleri topluca aç/kapa simgesi eklendi |
| 2026-10-22 | orta | Sabit sol Araç Takımları paleti kaldırıldı (işlevleri şerit ve menülerde doğrulandı) ve yerine kullanıcı rafı geldi: şerit/menü düğmeleri rafa sürüklenir, raf sol/sağ açılır, başlıktan kapanır, ayırıcıyla genişler; araçlar ve taraf profilde saklanır |
| 2026-10-22 | orta | Şerit eklendi (yedi sekme, büyük/küçük düğmeler, panel menüleri, taşma menüsü, Özellikler paneli); komut satırı pencere altına kenetlendi ve yerleşim sekmeleri ayrı şeride taşındı; Options referans düzeniyle (sekme şeridi, grup kutuları, ortalanmış Tamam/Yardım) yeniden çizildi; koyu/açık paletler referans tonlarına çekildi |
| 2026-10-13 | küçük | Options ▸ Selection, Select using two clicks only ekler; PICKDRAG 0 sürükleme penceresi oluşturmayı/tamamlamayı devre dışı bırakır, 2 tıklama veya sürüklemeyi korur ve tercih yeniden başlatmada kalır; UI işaretçi ve profil gerileme testleri |
| 2026-10-12 | küçük | Seçili ve üzerine gelinen nesneler seçim renginde yeniden çizilir (yazı ve ok uçları dahil); Options ▸ Selection, AutoCAD'inki gibi düzenlendi: Object grouping, Show grips, tutamaç nesne sınırı, seçim önizlemesi, Visual Effect Settings (seçim alanı, önizleme filtresi) |
| 2026-10-12 | küçük | Animasyon efektleri (iletişim kutusu ve menü solmaları, yumuşatılmış tekerlek kaydırma ve ⌘/Ctrl+tekerlek yakınlaştırma) işletim sisteminin ayarını izler; View ▸ Accessibility ▸ Animation Effects, System, On veya Off seçer |
| 2026-10-12 | küçük | Tarama ve Degrade / Taramayı Düzenle iletişim kutuları (HATCH, GRADIENT, HATCHEDIT, bir taramaya çift tıklama; HPDLGMODE) |
| 2026-10-12 | küçük | Blok referansları artık saklanan ekleme X/Y/Z'sini Özellikler Denetçisi'nde gösterir, sıfır olmayan Z dahil; alanlar salt okunur |
| 2026-10-11 | küçük | İzometrik çizim (#556): izometrik kenetleme ve ızgara, ISODRAFT, ISOPLANE (F5), izodüzlem eksenleri boyunca orto, ELLIPSE Isocircle |
| 2026-10-11 | küçük | Nesne kenetleme izleme, Extension, Parallel, Apparent Intersection, ek kutupsal açılar (#379): izleme 30% → 70%, nesne kenetlemeleri 40% → 45%; toplam 43% → 48% |
| 2026-10-10 | küçük | Dinamik giriş 15% → 45% (işaretçi kutuları, #60); bas-sürükle pencereleri (#63); toplam 40% → 43% |
| 2026-10-10 | büyük | İlk UI paritesi denetimi: nokta girişi, kenetlemeler, izleme, dinamik giriş, tutamaçlar, seçim, komut satırı, pencere çerçevesi, gezinme, tuşlar |
