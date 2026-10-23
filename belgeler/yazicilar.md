# Çiziciler

Bir çizici (çıktı aygıtı), bir çizimin neyi ve hangi kâğıt boyutlarında yazacağına karar verir. Sayfa
kurulumları ve çizimler bir aygıt ve onun kâğıtlarından birini seçer. Her kâğıt, yazdırılabilir alanını
sınırlayan kenar boşlukları taşır. Yerleşimler yazdırılabilir alanı sayfada kesikli bir dış hat olarak
gösterir.

## Aygıtlar

Yerleşik aygıtlar önce gelir:

| Aygıt | Çıktı |
|---|---|
| `SPCAD PDF` | vektör PDF |
| `SPCAD PNG` | aygıt çözünürlüğünde (300 dpi) beyaz kâğıt üzerine raster PNG |
| `SPCAD SVG` | vektör SVG |

Standart boyutları sunarlar:

- ISO 216 A0–A6 ve B0–B5
- ANSI A–E ve Legal
- ARCH A–E ve E1

ISO sayfalarının 5 mm kenar boşlukları vardır. İnç sayfalarının çeyrek inç (6.35 mm) kenar boşlukları
vardır. Aygıtı `None` olan bir sayfa kurulumu, eskisi gibi sayfa kurulumunun kâğıdına PDF olarak çizdirir.

Kullanıcı aygıtları, SPCAD ayarlar dizininin `plotters` klasöründe tutulan, kendi biçimimizdeki dosyalardır:

| Sistem | Klasör |
|---|---|
| Linux ve FreeBSD | `$XDG_CONFIG_HOME/spcad/plotters` veya `~/.config/spcad/plotters` |
| macOS | `~/Library/Application Support/SPCAD/plotters` |
| Windows | `%APPDATA%\SPCAD\plotters` |

`SPCAD_CONFIG_DIR` ayarlar dizininin yerine geçer. Web'de, ana bilgisayar depolama sağlamadıkça
(`cmd::plotters::set_plotter_store`) aygıtlar oturum boyunca kalır. İkili `.pc3` dosyaları okunmaz; o
biçimin kamuya açık bir belirtimini bulamadık.

## Dosyalar

`<name>.plotter.json`, biçim işaretçili UTF-8 JSON:

```json
{
  "spcadPlotter": 1,
  "name": "Office A3",
  "description": "Raster proofs",
  "output": "png",
  "resolution": 150,
  "orientation": "landscape",
  "defaultPaper": "A3 borderless",
  "papers": [
    { "name": "A3 borderless", "width": 297, "height": 420, "margins": [0, 0, 0, 0] },
    { "name": "ISO A4 (210.00 x 297.00 MM)", "width": 210, "height": 297, "margins": [5, 5, 5, 5] }
  ]
}
```

- `output`, `pdf`, `png` veya `svg`'dir.
- `resolution` inç başına nokta cinsindendir (50–2400) ve PNG'ye uygulanır.
- `orientation` (`portrait` veya `landscape`), bir sayfa kurulumu aygıta geçtiğinde kullanılır.
- Kâğıt boyutları dikey sayfada milimetre cinsindendir. `margins` soldan, alttan, sağdan ve üstten, mm
  cinsindendir.

Okuma, düşmanca değerleri düzeltir:

- Boyutlar 1–5080 mm aralığında tutulur ve genişlik ile yükseklik dikey olacak şekilde yer değiştirir.
- Kenar boşlukları 0–1000 mm aralığında tutulur. Yazdırılabilir alan bırakmayan kenar boşlukları atılır.
- Kâğıt adları benzersizdir ve bir aygıt en fazla 512 kâğıt tutar.
- Eksik veya bilinmeyen bir `defaultPaper`, ilk kâğıt olur.
- 1 MB'tan büyük dosyalar reddedilir.

Dosya adında harfler, rakamlar, boşluklar ve `-_()+,` dışındaki karakterler `_` olur.

## Komutlar

- `plottermanager {action, …}` (ayrıca **File ▸ Plotter Manager...**) aygıtları yönetir:
  - `list` (varsayılan) önce yerleşik aygıtları gösterir.
  - `get {name}` bir aygıt döndürür.
  - `new {name, from?, output?, …}` `from`u veya `output` için standart kâğıtlarla yerleşik aygıtı kopyalar.
  - `set {name, description?, output?, resolution?, orientation?, defaultPaper?, paper?: {name,
    width, height, margins?}, removePaper?}` bir aygıtı düzenler. `paper` bir kâğıt ekler veya o addaki
    kâğıdı değiştirir.
  - `delete {name}` bir aygıtı kaldırır.
  - `load {text, name?}` bir aygıt dosyasının içeriğini saklar.
  - `save {name, path?}` dosyayı `path`e yazar veya `text` olarak döndürür.

  Yerleşik aygıtlar düzenlenemez veya silinemez.
- Yazılan `PLOTTERMANAGER` veya `-PLOTTERMANAGER` bir seçenek ister:
  - `?` aygıtları listeler.
  - `New` bir ad ve bir çıktı ister (`Pdf`, `PNg`, `Svg`).
  - `Delete` bir aygıt ister.
  - `Paper` bir aygıt, bir kâğıt adı ve mm cinsinden `width,height` ister ve o kâğıdı ekler.
- `pagesetup {layout, device, paper?}` yerleşimin aygıtını ayarlar. `paper`, aygıtın kâğıtlarından birini
  veya kısa adıyla standart bir boyutu (`A3`, `Letter`) adlandırır. Kâğıdın kenar boşlukları yerleşimin
  yazdırılabilir alanı olur. `paper` olmadan, bir aygıt değişikliği geçerli kâğıdı aygıtta varsa korur,
  yoksa aygıtın varsayılan kâğıdını alır. Bir aygıt değişikliği ayrıca `landscape` verilmedikçe aygıtın
  yönünü alır. `device: "None"` onu temizler.
- `plot {device?, paper?, …}` `device` ile veya yerleşimin sayfa kurulumu aygıtıyla çizdirir.
  - Aygıt çıktıyı ve çözünürlüğü belirler.
  - `paper` (aygıtın kâğıtlarından biri) bu çizim için boyutu ve kenar boşluklarını ayarlar.
  - `paper` olmadan bir model alanı çizimi, aygıtın varsayılan kâğıdını ve yönünü kullanır.
  - Model çizimlerinde `center` varsayılan olarak `true`'dur ve mevcut ortalanmış çıktıyı korur. Çizilen
    sınırların sol alt köşesini yazdırılabilir alanın sol alt başlangıç noktasından konumlandırmak için
    `offset: [x, y]` ile `center: false` ayarlayın; ofsetler milimetredir, varsayılanı `[0, 0]`'dır ve
    ±100,000 mm ile sınırlıdır. PDF ve PNG bu denetimleri destekler. SVG çizimleri kendi sınır görünümlerini
    korur; etkin bir ofset yok sayılmak yerine reddedilir. Yerleşim çizimleri model ofsetlerini kullanmaz.
  - `exportpdf` aynı seçenekleri alır ve her zaman PDF yazar.

Model alanı PDF çizimleri kâğıdın kenar boşluklarını henüz uygulamaz. Çizimi varsayılan kenar boşluklarının
içine sığdırırlar. PNG çizimleri bunları uygular. SVG çizimleri bir yerleşimin tüm sayfasını veya model alanı
sınırlarını gösterir.

## Çizim saydamlığı

Nesneler saydamlıklarıyla (kendilerininki, ByLayer olduğunda katmanlarınınki, ByBlock olduğunda blok
başvurusununki) yalnızca istendiğinde, AutoCAD'de olduğu gibi çizdirilir:

- `pagesetup {layout, plotTransparency: bool}` yerleşimin "Çizim saydamlığı" ayarını saklar (varsayılan
  olarak kapalı). Sayfa Kurulumu Yöneticisi ve Plot iletişim kutusunda onay kutusu vardır.
- `plot {plotTransparency?: bool}` (ve `exportpdf`, `preview`) bunu bir çizim için geçersiz kılar. Model
  alanı çizimlerinde istenmedikçe kapalıdır.
- `PLOTTRANSPARENCYOVERRIDE` önce karar verir: 0 saydamlığı asla çizdirmez, 1 (varsayılan) yukarıdaki
  ayarı kullanır, 2 her zaman çizdirir.

PDF çizimleri bunu alfa olarak yazar (`/ca` ve `/CA` ile `/ExtGState`), PNG çizimleri karıştırır, SVG
çizimleri `opacity` kullanır. Saydamlık olmadan bir çizim eskisi gibi tam olarak yazılır. DXF çizim
yerleşimi bayraklarında bu seçenek için bit yoktur, bu yüzden DXF veya DWG dosyalarında kaydedilmez.

Ekranda `TRANSPARENCYDISPLAY` (varsayılan olarak açık; durum çubuğundaki Transparency düğmesi) saydamlığı
gösterir; bir çizimin PNG ve SVG dışa aktarımları bunu her zaman çizer.
