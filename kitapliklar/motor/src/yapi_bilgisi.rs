//! The version every front end shows: the Cargo version plus the UTC+3 build stamp baked in by
//! `build.rs`, e.g. `0.10.0_v202610221805`.

/// User-visible application version.
pub const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "_v", env!("SPCAD_BUILD_STAMP"));

/// The beta label shown after the product name: `BETA_vYYYYMMDDHHMM` (UTC+3, no separators).
pub const BETA_ETIKETI: &str = concat!("BETA_v", env!("SPCAD_BUILD_STAMP"));

#[cfg(test)]
mod testler {
    use super::{BETA_ETIKETI, VERSION};

    #[test]
    fn version_is_the_cargo_core_plus_a_twelve_digit_stamp() {
        let mut parts = VERSION.split("_v");
        let core = parts.next().unwrap_or_default();
        let stamp = parts.next().unwrap_or_default();
        assert_eq!(parts.next(), None);
        assert_eq!(core, env!("CARGO_PKG_VERSION"));
        assert_eq!(stamp.len(), 12);
        assert!(stamp.bytes().all(|b| b.is_ascii_digit()));
    }

    #[test]
    fn beta_label_is_the_stamp_without_separators() {
        let stamp = BETA_ETIKETI.strip_prefix("BETA_v").unwrap_or_default();
        assert_eq!(stamp.len(), 12, "{BETA_ETIKETI}");
        assert!(stamp.bytes().all(|byte| byte.is_ascii_digit()), "{BETA_ETIKETI}");
        assert!(VERSION.ends_with(stamp), "{VERSION} / {BETA_ETIKETI}");
    }
}
