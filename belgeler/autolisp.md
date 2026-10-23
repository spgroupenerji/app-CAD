# AutoLISP

SPCAD, AutoLISP rutinlerini (`.lsp` dosyaları) çalıştırır. Nelerin olduğu ve nelerin henüz olmadığı:
[eksikler.md](eksikler.md) (E1).

## Yükleme ve çalıştırma

- **APPLOAD** (Tools > Load Application…, takma ad `AP`) bir `.lsp` dosyası ister ve çalıştırır. Pencereye
  bir `.lsp` dosyası bırakmak da aynısını yapar.
- **`(load "name")`** bir dosyayı yoldan yükler; adın uzantısı yoksa `.lsp` eklenir. APPLOAD'in yüklediği
  bir dosya adıyla yeniden bulunur; dosya sistemi olmayan web'de de.
- **`(defun c:NAME …)`** bir komut tanımlar: komut satırına `NAME` yazın. Yerleşik komutlar adlarını korur.
- **Komut satırında**, `(expression)` değeri değerlendirir ve gösterir, `!variable` bir değişkeni gösterir.
  Bir komutun isteminde (serbest metin isteminde değil) yazıldığında, bir ifadenin değeri onu yanıtlar:
  `CIRCLE 0,0 (* 2 r)`.
- **Programatik olarak** (betikler, denetim kanalı, MCP, CLI): `lisp {code, inputs?}` ve
  `appload {path} | {name, text | data}` sonuna kadar çalışır ve `{value, output, error?}` döndürür.
  `inputs`, `get…` çağrılarını sırayla yanıtlar (sayılar, dizeler, `[x, y]` noktaları); kalanı olmayan
  bir çağrı nil alır.

### Başlangıçta

SPCAD, LISP'i **destek klasöründen**, ayarlar klasöründeki `support` dizininden yükler (Windows'ta
`%APPDATA%\SPCAD\support`, macOS'ta `~/Library/Application Support/SPCAD/support`, Linux'ta
`~/.config/spcad/support`; ilk başlatmada oluşturulur):

- `acad.lsp` oturum başına bir kez, ilk çizime; `ACADLSPASDOC` 1 olarak ayarlanmışsa her çizime;
- `acaddoc.lsp` açılan veya oluşturulan her çizime;
- ardından, tanımladıklarında `S::STARTUP`.

Bir başlangıç dosyasındaki `(autoload "file" '("CMD1" "CMD2"))`, `CMD1` ve `CMD2` komutlarını, ilk
yazıldıklarında `file` dosyasını (o da destek klasöründen) yükleyen komutlar yapar. Başlangıç dosyaları
girdi istemeden sonuna kadar çalışır. Bir çizimin kendi klasörü asla aranmaz: indirilen bir çizimle gelen
`acaddoc.lsp` çalışmaz.

Her çizimin kendi değişkenleri ve fonksiyonları vardır. Bir rutin, hangi komutları çalıştırırsa çalıştırsın
tek bir geri alma adımıdır. Girdi bekleyen bir rutin (`getpoint`, `pause`) bu sırada pencerenin çalışmasına
izin verir; Esc onu "Function cancelled" ile durdurur ve bunu `*error*` alır. Uzun bir döngü her karede bir
dilim çalıştırır; böylece pencere canlı kalır ve Esc onu da durdurur.

## Dil

Tamsayılar 32 bittir ve AutoLISP'inkiler gibi taşar; herhangi bir gerçek sayı aritmetiği gerçek yapar.
Değişkenler dinamik kapsamlıdır: bir fonksiyonun argümanları ve yerelleri ( `/` sonrası) çalışırken
bağlanır. Semboller büyük/küçük harfe duyarsızdır. Gerçek sayılar altı anlamlı basamakla yazdırılır.

Bir rutinin yaptığı hiçbir şey programı çökertemez: derin özyineleme "internal stack limit reached"
hatasıdır, bozuk kaynak bir okuyucu hatasıdır ve her yerleşik fonksiyon argümanlarını denetler.

## Yerleşik fonksiyonlar

- **Biçimler:** `quote` `'` `function` `setq` `set` `if` `cond` `while` `repeat` `foreach` `progn`
  `and` `or` `defun` `defun-q` `lambda` `eval` `apply` `mapcar` `load` `autoload` `exit` `quit`
- **Sayılar:** `+ - * / rem 1+ 1- abs min max sqrt exp log sin cos atan expt fix float gcd
  logand logior lsh ~ = /= < > <= >= zerop minusp numberp`
- **Listeler:** `car cdr` ve dört düzeye kadar `c…r` bileşimleri, `cons list append reverse
  length nth last member assoc subst acad_strlsort`, `vl-position vl-remove vl-sort
  vl-remove-if vl-remove-if-not vl-member-if vl-every vl-some vl-list-length vl-consp`
- **Yüklemler:** `atom listp null not type eq equal boundp vl-symbolp`
- **Dizeler:** `strcat strlen substr strcase itoa atoi atof chr ascii read wcmatch`,
  `vl-string-search vl-string-subst vl-string-trim vl-string-left-trim vl-string-right-trim
  vl-string-translate vl-string->list vl-list->string vl-prin1-to-string vl-princ-to-string
  vl-symbol-name vl-symbol-value vl-filename-base vl-filename-extension vl-filename-directory`
- **Geometri:** `polar distance angle inters trans` (`trans` noktayı döndürür: henüz UCS yok)
- **Çıktı ve dosyalar:** `princ prin1 print prompt terpri alert open close read-line write-line
  findfile`
- **Hatalar:** `*error*`, `vl-catch-all-apply vl-catch-all-error-p vl-catch-all-error-message`
- **Çizim:** `command` (ayrıca `command-s`, `vl-cmdf`) `pause` ile, sonraki çağrı için açık bırakılan
  bir komut ve iptal etmek için `(command)`; `getvar setvar`; `entget entmake entmod entdel entlast
  entnext entupd handent`; `ssget` (X, L, P, I, W, C, F, WP, CP kipleri, bir nokta veya kullanıcının
  seçmesi; joker karakterli ve `-4` işleçli filtreler) `ssadd ssdel sslength ssname ssmemb
  sssetfirst ssgetfirst`; `tblsearch tblnext` (LAYER, LTYPE, STYLE, BLOCK, DIMSTYLE, APPID)
- **Girdi:** `initget getpoint getcorner getdist getangle getorient getreal getint getstring
  getkword entsel nentsel`
- **Birimler:** `rtos angtos distof angtof`
- **Etkisiz işlemler:** `vl-load-com gc redraw`; `ver`

Varlık listeleri SPCAD'in DXF dosyalarının DXF grup kodlarını kullanır: `entget` ne döndürürse bir DXF
dosyasının nesne için tuttuğu odur, artı `-1` (adı) ve `410` (yerleşimi). Açılar (50–58 grupları),
dosyanın derece tuttuğu AutoCAD'de olduğu gibi radyandır; `entmake`, `entmod` ve `ssget` filtreleri de
radyan alır.

## Korpusa karşı test

AutoLISP uyumluluk korpusu (#648) kendi deposunda bulunur. `SPCAD_LISP_CORPUS` değişkenini ona
yönlendirin ve `cargo test -p spcad-engine --test lisp_corpus` çalıştırın; değişken olmadan test
atlandığını söyler.
