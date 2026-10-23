# Çizim stilleri

Bir çizim stili tablosu nesnelerin nasıl yazdırılacağına karar verir. SPCAD bunu PDF çizimlerine (PLOT,
EXPORTPDF ve bunlar üzerinden çizdiren her şey) uygular. Ayrıca **Çizim stillerini göster** açıkken gösterilen
bir yerleşime de uygular: ekranda ve yerleşimin PNG ve SVG dışa aktarımlarında.

## Tablolar

| Tür | Stili neye göre seçer | Stiller |
|---|---|---|
| Renge bağlı (`.ctb`) | nesnenin dizin rengi (gerçek renk en yakın dizin rengini kullanır) | tam olarak 255, `Color_1` … `Color_255` |
| Adlandırılmış (`.stb`) | nesnenin **PlotStyle** özelliği | önce `Normal`, sonra herhangi adlar |

Bir stil şunları ayarlayabilir:

| Alan | Anlamı | "Use object" değeri |
|---|---|---|
| `color` | çıktı rengi, `"#rrggbb"` | yok / `null` |
| `grayscale` | rengi parlaklığının grisi olarak yazdır (Rec. 709 ağırlıkları) | `false` |
| `screening` | mürekkep yoğunluğu %0–100, beyaz kâğıda doğru karıştırarak | `100` |
| `linetype` | çizgi tipi adı (yüklü veya yerleşik kitaplıktan; `Continuous` düz çizer) | yok / `null` |
| `lineweight` | milimetrenin yüzde birleri, 0–211; çizgi kalınlıkları çizdirildiğinde kullanılır | yok / `null` |
| `end`, `join`, `fill` | çizgi ucu, çizgi birleşimi ve alan dolgu stili | `object` |

Renk 7 siyah yazdırır.

### Çizgi uçları, birleşimler ve dolgu desenleri

- **`end` ve `join`**, kâğıt üzerinde 0.25 mm'den geniş çizgileri (çizgi kalınlıklarından) şekillendirir.
  Çizgi dolu dış hattı olarak çizilir: `butt` uç noktada biter, `square` onun ötesine yarım genişlik ulaşır,
  `round` yarım daire, `diamond` ise bir sivri uç ekler. `miter` birleşimleri bir noktada buluşur (10 yarım
  genişlikten sonra pahlanır), `bevel` köşeyi keser, `round` yuvarlar ve `diamond` yarım genişlik dışarı
  sivrilir. `object` yuvarlaktır, çizgiler zaten öyle çizilir.
- **`fill`**, dolu alanların düz dolgusunu değiştirir: 2B katılar, izler, geniş çoklu çizgiler, dolu taramalar
  ve dolu metin. `solid` ve `object` düz tutar. Desenler SPCAD'in kendi tanımlarıdır; kâğıt üzerinde ölçülür
  ve sayfanın başlangıç noktasına sabitlenir. Her 1 mm'de yinelenirler:

| Dolgu | Desen |
|---|---|
| `checkerboard` | 0.5 mm kareler, bir dolu bir boş |
| `crosshatch` | yatay ve dikey çubuklar, 0.2 mm genişliğinde |
| `diamonds` | 45° döndürülmüş 0.35 mm kareler, her 0.7 mm'de |
| `horizontalBars`, `verticalBars` | yinelenenin üçte biri genişliğinde çubuklar |
| `slantLeft`, `slantRight` | yinelenenin üçte biri genişliğinde `\` ve `/` çubukları |
| `squareDots` | her 0.5 mm'de 0.2 mm kareler |

Silme alanları asla desenlenmez. İki milyondan fazla desen hücresi gerektirecek bir çizim, kalan alanları
düz tutar. Yerleşimler bunları görüntüleme listesi oluşturulurken çizer. Model alanı çizimi bunları kendi
çizim ölçeğinde çizer.

### Yerleşik tablolar

Bunlar SPCAD'in kendi tanımlarıdır, herhangi bir satıcının dosyalarının kopyaları değildir.

| Tablo | Ne yapar |
|---|---|
| `default.ctb` | her nesne özelliğini korur |
| `monochrome.ctb` | her rengi siyah yazdırır |
| `grayscale.ctb` | her rengi parlaklığının grisi olarak yazdırır |
| `default.stb` | `Normal` (nesne özellikleri), `Black`, `Screened 50%`, `Thick` (0.50 mm) |
| `monochrome.stb` | `Normal` siyah yazdırır; `Screened 50%` siyahı %50'de yazdırır |

Çizimde tutulan bir tablo, aynı adlı yerleşik tabloyu gölgeler.

## Adlandırılmış çizim stilleri

Katmanların bir çizim stili adı vardır (varsayılan `Normal`). Nesneler `ByLayer` (varsayılan), `ByBlock`
veya bir stil adı taşır. `ByLayer` katmanın stilini alır. Bir blok içindeki 0 katmanında bu, blok
başvurusunun katmanıdır. `ByBlock` blok başvurusunun stilini alır. Tablonun sahip olmadığı bir ad `Normal`
olarak yazdırılır. Renge bağlı bir tablo bu özelliği yok sayar.

## Komutlar

- `pagesetup {layout, plotStyleTable, displayPlotStyles}` yerleşimin tablosunu ve Çizim stillerini göster
  bayrağını ayarlar. Yerleşim olmadan, yerleşik tabloları `plotStyleTables` içinde bildirir.
- `plot` / `exportpdf {plotStyleTable}` bir çizim için sayfa kurulumunun tablosunu değiştirir. `"None"`
  nesne renklerini çizdirir.
- `plotstyle {name, handles?}` nesnelerin çizim stilini ayarlar. Hiçbir şey seçili değilken yeni nesneler
  için geçerli çizim stilini ayarlar (`CPLOTSTYLE`). `name` olmadan geçerli stili ve adlandırılmış stilleri
  bildirir.
- `layer.set {name, plotStyle}` bir katmanın çizim stilini ayarlar.
- `stylesmanager {action, …}` tabloları yönetir: `list`, `get`, `new` (`kind`, `from`), `set`
  (`style: {name, …fields}`, `removeStyle`, `description`), `delete`, `load` (`path` veya `text`)
  ve `save` (`path` veya `text` döndürür). Yazıldığında veya **File ▸ Plot Styles...** menüsünden
  seçildiğinde, Çizim Stili Yöneticisi iletişim kutusunu açar.

## Tablo dosyaları

Tablolar kendi biçimimizde kaydedilir: biçim işaretçili UTF-8 JSON. "nesnenin özelliğini kullan" olarak
bırakılan alanlar atlanır.

```json
{
  "spcadPlotStyleTable": 1,
  "name": "office.stb",
  "description": "Office pens",
  "kind": "named",
  "styles": [
    { "name": "Normal" },
    { "name": "Pen 5", "color": "#000000", "screening": 40, "lineweight": 35, "linetype": "HIDDEN", "end": "round" }
  ]
}
```

- `kind`, `colorDependent` veya `named`'dir.
- `end`, `object`, `butt`, `square`, `round` veya `diamond`'dan biridir.
- `join`, `object`, `miter`, `bevel`, `round` veya `diamond`'dan biridir.
- `fill`, `object`, `solid`, `checkerboard`, `crosshatch`, `diamonds`, `horizontalBars`,
  `slantLeft`, `slantRight`, `squareDots` veya `verticalBars`'tan biridir.
- `color` ayrıca `[r, g, b]` olarak yazılabilir.

Okuma, düşmanca değerleri düzeltir:

- Ekranlama 100 ile, çizgi kalınlıkları 211 ile sınırlanır.
- Renge bağlı bir tablo her zaman `Color_n` adlı 255 stil alır.
- Adlandırılmış bir tablo benzersiz stil adları alır (en fazla 1024), `Normal` önce gelir.
- 4 MB'tan büyük dosyalar reddedilir.

İkili `.ctb`/`.stb` dosyaları okunmaz. O biçimin kamuya açık bir belirtimini bulamadık.

## DXF dosyalarında

- Sayfa kurulumunun tablosu PLOTSETTINGS grup 7'dir ve Çizim stillerini göster, çizim yerleşimi bayrağı 2'dir
  (grup 70).
- Çizim stili adları DXF Reference'ı izler. Adlandırılmış nesne sözlüğü girdisi
  `ACAD_PLOTSTYLENAME`, girdileri `ACDBPLACEHOLDER` nesnelerini adlandıran bir `ACDBDICTIONARYWDFLT`'tir
  (varsayılan `Normal`). Katmanlar ve nesneler grup 390 ile onlara işaret eder. 390'ı olmayan bir nesne
  `ByLayer`'dır. `ByBlock` yazılmaz, bu yüzden `ByLayer` olarak geri okunur.
- Çizimde tutulan tablolar, adlandırılmış nesne sözlüğünün `SPCAD_PLOTSTYLES` XRECORD'unda (JSON) taşınır.
