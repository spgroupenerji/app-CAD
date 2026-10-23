# Hakkında penceresindeki katkıcılar

**Hakkında ▸ Katkıcılar** SPCAD'e katkıda bulunan herkesi, **Hakkında ▸ Modeller** ise
`Co-Authored-By` izlerinde adı geçen yapay zekâ modellerini gösterir. Bu, paylaşılan craftrules
standardı
[`standards/katkida-bulananlar.md`](https://github.com/storytold/craftrules/blob/main/standards/katkida-bulananlar.md)
ile uyumludur; bu sayfa kararın yerel kopyasıdır.

## Karar

- İki dosya da arayüz sandığının `build.rs` dosyası tarafından **ikiliye derlenir** (çalışma anında
  diskten veya ağdan hiçbir şey okunmaz; web derlemesinde de bulunurlar):
  - [`katkida-bulananlar/contributors.json`](../katkida-bulananlar/contributors.json): GitHub kullanıcı adı
    başına commit istatistikleri. `craftrules/scripts/contributors.py` tarafından **üretilir**;
    asla elle düzenlenmez.
  - [`katkida-bulananlar/people.toml`](../katkida-bulananlar/people.toml): kişilerin kendileri için seçtiği
    adlar. **Her katkıcı yalnızca kendi girdisini**, kendi PR'ında ekler.
- Her katkıcı tek satır alır: GitHub kullanıcı adı (her zaman), görünen ad (isteğe bağlı veya
  eşitlenmiş GitHub profil adı), gerçek ad (yalnızca bize söylediyse), birleştirilmiş PR'lar,
  commit'ler, eklenen satırlar, silinen satırlar, eklenen ikili varlıklar, kaldırılan ikili
  varlıklar, ilk ve son commit tarihi.
- Liste, aynı sırayla bir **serbest akış** (metin olarak akan adlar) veya bir **tablo** olarak
  gösterilir. Ada, PR'lara, commit'lere, eklenen satırlara, silinen satırlara, satır farkına,
  eklenen/kaldırılan ikili varlıklara, ilk veya son commit tarihine göre sıralanır. Ad düğmesi
  **Kullanıcı adı → Görünen ad → Gerçek ad** arasında döner; eksik bir ad `@kullanıcıadı`
  değerine geri döner. Alfabetik sıralama büyük/küçük harfe duyarsızdır ve `@` işaretini yok sayar.

## Kendinizi kredilendirin

Girdinizi, kendi commit'lediğiniz bir PR'da `katkida-bulananlar/people.toml` dosyasına ekleyin (her alan
isteğe bağlıdır):

```toml
[people.your-github-username]
real_name = "Your Name"
display_name = "Nickname"
sync_github_name = true   # use the name on your public GitHub profile as display name
```

veya craftrules `craft-uygulamalar/` yanında checkout edilmişse:

```sh
python3 ../../craftrules/scripts/contributors.py --add-me . --real-name "Your Name" --sync-github-name
```

Adlar bir sonraki yenilemeden sonra görünür: betik, her girdinin kendi kullanıcısı tarafından (veya
`consent_source` bulunan bir sürdürücü tarafından) commit'lendiğini denetler. Adlarınızı kaldırmak
için girdinizi silin.

## Yenileme (sürdürücüler)

```sh
python3 ../../craftrules/scripts/contributors.py .           # yeniden üret + doğrula; git, Python 3.11+ ve gh gerekir
python3 ../../craftrules/scripts/contributors.py --check .   # yalnızca people.toml dosyasını doğrula
```

Güncellenen `katkida-bulananlar/contributors.json` dosyasını commit'leyin. Git yazar adları ve e-posta
adresleri hiçbir zaman kaydedilmez.
