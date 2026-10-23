# SPCAD'i özelleştirme ve otomatikleştirme

SPCAD'i otomatikleştirmek veya başka CAD programları için yazılmış eklentileri taşımak için bugün
kullanabilecekleriniz ve henüz bulunmayanlar.

| Elinizde | SPCAD'de | Daha fazla |
|---|---|---|
| LISP rutinleri (`.lsp`) | APPLOAD veya `(load "name")` ile çalıştırın. Çekirdek dil, `command`, varlık listeleri, seçim kümeleri, `get…` girdisi ve `c:` komutları çalışır. | [autolisp.md](autolisp.md) |
| Başlangıç LISP'i (`acad.lsp`, `acaddoc.lsp`) | Başlangıçta ve her çizim için destek klasöründen yüklenir. | [autolisp.md](autolisp.md#başlangıçta) |
| Betikler (`.scr`) | SCRIPT (Araçlar > Betik Çalıştır...), `spcad-cli run --script-file` ve MCP ile denetim kanalı üzerinden betik metni. | aşağıda |
| Komut takma adları (`.pgp`) | ALIASEDIT `{file}` oturum için bir takma ad dosyası yükler; REINIT dosyayı yeniden okur. | aşağıda |
| Derlenmiş eklentiler (C++, .NET) | Desteklenmez. Rust ile yazılıp WebAssembly'ye derlenen eklentiler için bir eklenti API'si planlanıyor. | [eksikler.md](eksikler.md) (E4) |
| CAD programını dışarıdan süren programlar | JSON denetim kanalı, MCP sunucusu ve `spcad-cli`. TCP soketi açabilen veya süreç başlatabilen herhangi bir dil bunları kullanabilir. | aşağıda |

## LISP rutinleri

Yalnızca nesne çizen, düzenleyen ve sorgulayan rutinlerin çoğu olduğu gibi çalışır. Henüz
desteklenmeyenler: Visual LISP ActiveX fonksiyonları (`vlax-…`, `vla-…`), DCL iletişim kutuları ve
reactor'lar ([eksikler.md](eksikler.md) E1). Bunları kullanan bir rutinin bu bölümlerinin yeniden
yazılması gerekir; örneğin iletişim kutusu yerine `command` çağrıları ve `get…` istemleriyle.

Her oturumda bulunmasını istediğiniz rutinleri destek klasörüne koyun (ayar klasöründeki `support`;
yollar [autolisp.md](autolisp.md#başlangıçta) içinde) ve `acad.lsp` ya da `acaddoc.lsp` dosyasından
veya `(autoload "file" '("CMD"))` ile yükleyin; böylece komut ilk kez yazıldığında yüklenirler.

## Betikler

`.scr` dosyası, komut satırına tek satırda yazacağınız şeylerin dosyasıdır. Uygulamada SCRIPT ile
veya başsız olarak çalıştırın:

```sh
spcad-cli run drawing.dxf --script-file setup.scr --save out.dxf
```

## Komut takma adları

ALIASEDIT bir `.pgp` takma ad dosyasını (`ALIAS, *COMMAND` satırları) okur ve takma adlarını
oturumun geri kalanı için ekler. Harici program başlatacak satırlar atlanır. Bir betikten veya
denetim kanalından: `aliasedit {"file": "/path/to/aliases.pgp"}`.

## SPCAD'i başka bir programdan sürme

Her komutun iletişim kutusu açmadan çalışan bir JSON biçimi vardır. `spcad-cli commands [FILTER]`
kataloğu yazdırır: her komutun kimliği ve parametreleri.

- **Başsız toplu işler:** `spcad-cli run FILE --cmd 'id {json}' --save OUT --export OUT.png`.
- **Çalışan uygulama:** `--control PORT` ile başlatın ve `127.0.0.1:PORT` adresine JSON satırları
  gönderin. Bkz. [denetim-protokolu.md](denetim-protokolu.md).

  ```sh
  printf '%s\n' '{"id":1,"method":"cmdline.input","params":{"text":"circle 0,0 5"}}' | nc -w 2 127.0.0.1 7979
  ```

- **Yapay zekâ ajanları:** `spcad-cli mcp`, başsız veya uygulamaya bağlı. Bkz. [mcp.md](mcp.md).
