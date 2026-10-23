//! Bakes the build stamp (`YYYYMMDDHHMM`, UTC+3) into the engine so every front end shows the same
//! version: `MAJOR.MINOR.PATCH_vYYYYMMDDHHMM`. `SPCAD_BUILD_STAMP` overrides it and is taken as
//! already being in UTC+3.

use std::time::{SystemTime, UNIX_EPOCH};

/// Türkiye saat dilimi: yıl boyunca UTC+3 (2016'dan beri yaz saati uygulanmıyor).
const UTC_OFFSET_SECONDS: u64 = 3 * 60 * 60;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SPCAD_BUILD_STAMP");
    let stamp = match std::env::var("SPCAD_BUILD_STAMP") {
        Ok(value) if is_stamp(&value) => value,
        Ok(value) => {
            println!("cargo:warning=SPCAD_BUILD_STAMP={value:?} is not YYYYMMDDHHMM; using the build time");
            build_time_stamp()
        }
        Err(_) => build_time_stamp(),
    };
    println!("cargo:rustc-env=SPCAD_BUILD_STAMP={stamp}");
}

fn is_stamp(value: &str) -> bool {
    value.len() == 12 && value.bytes().all(|b| b.is_ascii_digit())
}

fn build_time_stamp() -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0).saturating_add(UTC_OFFSET_SECONDS);
    let days = (secs / 86_400) as i64;
    let of_day = secs % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}{month:02}{day:02}{:02}{:02}", of_day / 3_600, (of_day % 3_600) / 60)
}

/// Howard Hinnant's `civil_from_days`: days since 1970-01-01 to a proleptic Gregorian date.
fn civil_from_days(days: i64) -> (i64, u64, u64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe as i64 + era * 400 + i64::from(month <= 2);
    (year, month, day)
}
