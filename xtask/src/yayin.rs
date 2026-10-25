//! `cargo xtask release`: build the release binary into the workspace-root `release/` directory
//! under a timestamped name and keep only that latest file there.
//!
//! The build stamp is produced once here and the same value is baked into the binary through
//! `SPCAD_BUILD_STAMP`, so the artifact name and the About window never disagree.

use std::path::{Path, PathBuf};

/// Dosya adındaki ürün kısaltması.
const APP_AD: &str = "sp-group-cad";
/// Ürün kısaltmasıyla tek damgayı ayıran ek.
const BETA_EKI: &str = "_beta-";
/// Tek dosya derlemesinin uzantısı.
const AD_SONEK: &str = ".exe";
/// `cargo build --release -p spcad` çıktısının adı.
const IKI_ADI: &str = "spcad.exe";
/// `kitapliklar/motor/build.rs` tarafından okunan derleme damgası değişkeni.
const DAMGA_DEGISKENI: &str = "SPCAD_BUILD_STAMP";

/// Türkiye yerel saatiyle (UTC+3, yaz saati yok) tek damga: `vYYYYMMDDHHMM`.
fn damga(simdi: std::time::SystemTime) -> String {
    let sn = simdi.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) + 3 * 3600;
    let (y, a, g) = medeni_tarih((sn / 86_400) as i64);
    let kalan = sn % 86_400;
    format!("v{y:04}{a:02}{g:02}{:02}{:02}", kalan / 3600, (kalan % 3600) / 60)
}

/// Gün sayısından Gregoryen tarihe (Howard Hinnant, civil_from_days).
fn medeni_tarih(gun: i64) -> (i64, u32, u32) {
    let z = gun + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let g = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let a = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if a <= 2 { y + 1 } else { y }, a, g)
}

/// `release/` altındaki indirilebilir dosya adı: `sp-group-cad_beta-vYYYYMMDDHHMM.exe`.
fn artefakt_adi(damga: &str) -> String {
    format!("{APP_AD}{BETA_EKI}{damga}{AD_SONEK}")
}

/// `release/` içindeki bu derlemeden önceki tek dosya derlemeleri (eski `_beta_v…` adları dâhil).
fn eskiler(dizin: &Path, son: &str) -> Result<Vec<PathBuf>, String> {
    let mut eski = Vec::new();
    for kayit in std::fs::read_dir(dizin).map_err(|e| format!("release/ okunamadı: {e}"))? {
        let yol = kayit.map_err(|e| format!("release/ okunamadı: {e}"))?.path();
        let ad = yol.file_name().and_then(|a| a.to_str()).unwrap_or_default();
        if ad != son && ad.starts_with(APP_AD) && ad.ends_with(AD_SONEK) {
            eski.push(yol);
        }
    }
    Ok(eski)
}

pub fn run(root: &Path) -> Result<(), String> {
    // Damga bir kez üretilir: hem artefakt adına hem derlemeye gömülen sürüme aynı değer gider.
    let damga = damga(std::time::SystemTime::now());
    let mut c = crate::cargo();
    c.args(["build", "--release", "-p", "spcad"]).env(DAMGA_DEGISKENI, &damga);
    if let Err(e) = crate::run(c, "cargo build --release -p spcad") {
        return Err(format!("{e}\n  ipucu: çalışan spcad.exe ikiliyi kilitler; önce `taskkill /IM spcad.exe /F`"));
    }
    let kaynak = root.join("target").join("release").join(IKI_ADI);
    if !kaynak.is_file() {
        return Err(format!("{} bulunamadı; derleme gerçekten üretti mi?", kaynak.display()));
    }
    let dizin = root.join("release");
    std::fs::create_dir_all(&dizin).map_err(|e| format!("release/ oluşturulamadı: {e}"))?;
    let ad = artefakt_adi(&damga);
    let hedef = dizin.join(&ad);
    std::fs::copy(&kaynak, &hedef).map_err(|e| format!("{} kopyalanamadı: {e}", hedef.display()))?;
    for yol in eskiler(&dizin, &ad)? {
        std::fs::remove_file(&yol).map_err(|e| format!("eski derleme {} silinemedi: {e}", yol.display()))?;
    }
    println!("release: {}", hedef.display());
    println!("Hakkında: BETA_{damga}");
    Ok(())
}

#[cfg(test)]
mod testler {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    fn damga_sn(sn: u64) -> String {
        damga(UNIX_EPOCH + Duration::from_secs(sn))
    }

    #[test]
    fn damga_utc3_ve_bicim_doğru() {
        assert_eq!(damga_sn(0), "v197001010300");
        assert_eq!(damga_sn(1_709_164_800), "v202402290300");
        assert_eq!(damga_sn(1_792_714_736), "v202610230318");
        assert_eq!(damga(std::time::SystemTime::now()).len(), 13);
    }

    #[test]
    fn medeni_tarih_yilbasi_ve_artik_yil() {
        assert_eq!(medeni_tarih(0), (1970, 1, 1));
        assert_eq!(medeni_tarih(19_723), (2024, 1, 1));
        assert_eq!(medeni_tarih(20_653), (2026, 7, 19));
    }

    #[test]
    fn eskiler_yalniz_onceki_derlemeleri_secer() {
        let dizin = std::env::temp_dir().join(format!("spcad-yayin-{}", std::process::id()));
        std::fs::create_dir_all(&dizin).unwrap();
        for ad in ["sp-group-cad_beta-v202601010000.exe", "sp-group-cad_beta_v202501010000.exe", "baska.exe", "sp-group-cad_beta-v202602020000.txt"] {
            std::fs::write(dizin.join(ad), b"x").unwrap();
        }
        let mut adlar: Vec<String> = eskiler(&dizin, "sp-group-cad_beta-v202602020000.exe")
            .unwrap()
            .iter()
            .filter_map(|p| p.file_name().and_then(|a| a.to_str()).map(str::to_owned))
            .collect();
        adlar.sort();
        assert_eq!(adlar, ["sp-group-cad_beta-v202601010000.exe", "sp-group-cad_beta_v202501010000.exe"]);
        std::fs::remove_dir_all(&dizin).unwrap();
    }

    #[test]
    fn artefakt_adi_tek_damgayi_tasir() {
        let damga = "v202610252115";
        let ad = artefakt_adi(damga);
        assert_eq!(ad, "sp-group-cad_beta-v202610252115.exe");
        assert!(ad.starts_with(APP_AD));
        assert!(ad.ends_with(AD_SONEK));
        assert_eq!(ad.strip_prefix(APP_AD).and_then(|k| k.strip_prefix(BETA_EKI)).and_then(|k| k.strip_suffix(AD_SONEK)), Some(damga));
    }
}
