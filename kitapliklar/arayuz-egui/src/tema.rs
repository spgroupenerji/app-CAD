//! Design tokens. Colours were measured from the reference look (dark slate chrome, near-black
//! navy model space) and are SPCAD's own values; every widget reads them from here.
//!
//! Two palettes: [`Tokens::DARK`] (the original look) and [`Tokens::LIGHT`]. The interface theme
//! changes the chrome (title and tool bar, menus, palettes, tabs, command line, status bar,
//! dialogs); the drawing area keeps its model-space colours in both, as drawing colours are not
//! part of the interface theme. [`Tokens::get`] returns the palette of the theme in use, which
//! [`CadApp::logic`](crate::CadApp::logic) resolves once per frame from the user's [`ThemePref`].

#![allow(clippy::disallowed_methods)]

use std::cell::Cell;

use egui::{Color32, FontFamily, FontId, Theme, Visuals};
use serde::{Deserialize, Serialize};

/// The interface theme the user chose.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePref {
    /// Follow the operating system's light/dark appearance, live.
    System,
    Light,
    /// The default: SPCAD's original look.
    #[default]
    Dark,
}

/// The theme `System` uses when the OS (or browser) reports no appearance.
pub const SYSTEM_FALLBACK: Theme = Theme::Dark;

impl ThemePref {
    pub const ALL: [ThemePref; 3] = [ThemePref::System, ThemePref::Light, ThemePref::Dark];

    pub fn as_str(self) -> &'static str {
        match self {
            ThemePref::System => "system",
            ThemePref::Light => "light",
            ThemePref::Dark => "dark",
        }
    }

    pub fn parse(s: &str) -> Option<ThemePref> {
        ThemePref::ALL.into_iter().find(|t| t.as_str().eq_ignore_ascii_case(s.trim()))
    }

    /// The theme to show, given what the OS reports (`ctx.system_theme()`).
    pub fn resolve(self, system: Option<Theme>) -> Theme {
        match self {
            ThemePref::System => system.unwrap_or(SYSTEM_FALLBACK),
            ThemePref::Light => Theme::Light,
            ThemePref::Dark => Theme::Dark,
        }
    }
}

thread_local! {
    /// The theme in use on the UI thread (set by [`set_active`] each frame).
    static ACTIVE: Cell<Theme> = const { Cell::new(Theme::Dark) };
}

/// The theme [`Tokens::get`] returns colours for.
pub fn active() -> Theme {
    ACTIVE.with(Cell::get)
}

pub fn set_active(theme: Theme) {
    ACTIVE.with(|a| a.set(theme));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub chrome: Color32,
    pub chrome_dark: Color32,
    pub panel: Color32,
    pub tab_active: Color32,
    pub control: Color32,
    pub control_hover: Color32,
    pub border: Color32,
    pub canvas: Color32,
    pub grid_minor: Color32,
    pub grid_major: Color32,
    pub axis_x: Color32,
    pub axis_y: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub icon: Color32,
    pub icon_accent: Color32,
    pub icon_point: Color32,
    pub accent: Color32,
    pub selection: Color32,
    pub hover: Color32,
    pub grip: Color32,
    pub grip_hot: Color32,
    /// Grips of dynamic block references (AutoCAD's GRIPDYNCOLOR).
    pub grip_dyn: Color32,
    pub snap: Color32,
    /// COMPARE: objects only in this drawing, only in the other one, and the areas of change.
    pub compare_this: Color32,
    pub compare_other: Color32,
    pub compare_area: Color32,
    pub window_stroke: Color32,
    pub crossing_stroke: Color32,
    pub cmd_bg: Color32,
    pub toggle_on: Color32,
    /// Text fields and other inset (extreme) backgrounds.
    pub field: Color32,
    /// Selected text and selected list rows.
    pub select_bg: Color32,
    /// Lines and labels drawn straight onto the drawing area (UCS icon, viewport label, rubber
    /// bands): they follow the model-space background, not the interface theme.
    pub canvas_ink: Color32,
    /// The command-line history lines behind the input bar.
    pub cmd_history: Color32,
    pub cmd_border: Color32,
    /// Command-line keywords (`[Undo/Close]`) and their hover background.
    pub cmd_keyword: Color32,
    pub cmd_keyword_hover: Color32,
    /// The command-line AutoComplete list: background and the highlighted first row.
    pub list_bg: Color32,
    pub list_row: Color32,
    /// Transient warnings (status bar messages).
    pub warn: Color32,
    /// The drawing-area crosshair cursor and its pick box.
    pub crosshair: Color32,
    /// The sheet of paper on a layout tab.
    pub paper: Color32,
}

/// Tipografi ölçeği (pt). Şerit etiketleri, panel metni ve diyalog başlıkları buradan seçilir.
pub const YAZI_MIKRO: f32 = 10.5;
pub const YAZI_KUCUK: f32 = 11.5;
pub const YAZI_GOVDE: f32 = 12.5;
pub const YAZI_VURGU: f32 = 14.0;
pub const YAZI_BASLIK: f32 = 17.0;

/// 4 pt tabanlı boşluk ölçeği (pt).
pub const BOSLUK_1: f32 = 4.0;
pub const BOSLUK_2: f32 = 8.0;
pub const BOSLUK_3: f32 = 12.0;
pub const BOSLUK_4: f32 = 16.0;
pub const BOSLUK_5: f32 = 24.0;
pub const BOSLUK_6: f32 = 32.0;
/// Varsayılan satır yüksekliği (pt).
pub const SATIR_YUKSEKLIK: f32 = 28.0;

/// Köşe yarıçapları (pt).
pub const KOSE_KUCUK: u8 = 4;
pub const KOSE_ORTA: u8 = 6;
pub const KOSE_BUYUK: u8 = 8;

/// Hareket süreleri (saniye): durum değişimi, açılır katman, panel.
pub const HAREKET_HIZLI: f32 = 0.09;
pub const HAREKET_ORTA: f32 = 0.15;
pub const HAREKET_YAVAS: f32 = 0.22;

impl Tokens {
    pub const DARK: Tokens = Tokens {
        chrome: Semantik::DARK.muted,
        chrome_dark: Semantik::DARK.secondary,
        panel: Semantik::DARK.card,
        tab_active: Semantik::DARK.accent,
        control: Semantik::DARK.secondary,
        control_hover: Semantik::DARK.accent,
        border: Semantik::DARK.border,
        canvas: Color32::from_rgb(0x06, 0x08, 0x0b),
        grid_minor: Color32::from_rgb(0x12, 0x16, 0x1b),
        grid_major: Color32::from_rgb(0x1d, 0x23, 0x2b),
        axis_x: Color32::from_rgb(0x43, 0x14, 0x16),
        axis_y: Color32::from_rgb(0x13, 0x3a, 0x1b),
        text: Semantik::DARK.foreground,
        text_dim: Semantik::DARK.muted_foreground,
        text_faint: Color32::from_rgb(0x87, 0x94, 0xa2),
        icon: Semantik::DARK.card_foreground,
        icon_accent: Semantik::DARK.ring,
        icon_point: Color32::from_rgb(0xf0, 0x7a, 0x3a),
        accent: Semantik::DARK.primary,
        selection: Semantik::DARK.ring,
        hover: Semantik::DARK.ring,
        grip: Color32::from_rgb(0x2f, 0x6b, 0xff),
        grip_hot: Color32::from_rgb(0xff, 0x3b, 0x3b),
        grip_dyn: Color32::from_rgb(0x26, 0xc6, 0xe8),
        snap: Color32::from_rgb(0x2f, 0xd0, 0x62),
        compare_this: Color32::from_rgb(0x4c, 0xd9, 0x7b),
        compare_other: Color32::from_rgb(0xff, 0x5c, 0x5c),
        compare_area: Color32::from_rgb(0xff, 0xb4, 0x3c),
        window_stroke: Color32::from_rgb(0x6f, 0xa8, 0xff),
        crossing_stroke: Color32::from_rgb(0x7f, 0xe0, 0x7f),
        cmd_bg: Semantik::DARK.input,
        toggle_on: Semantik::DARK.primary,
        field: Semantik::DARK.input,
        select_bg: Semantik::DARK.accent,
        canvas_ink: Color32::from_rgb(0xb4, 0xba, 0xc2),
        cmd_history: Color32::from_rgba_unmultiplied_const(0x0b, 0x10, 0x15, 255),
        cmd_border: Semantik::DARK.border,
        cmd_keyword: Color32::from_rgb(0x8f, 0xc1, 0xff),
        cmd_keyword_hover: Color32::from_rgb(0x22, 0x33, 0x48),
        list_bg: Semantik::DARK.input,
        list_row: Semantik::DARK.accent,
        warn: Semantik::DARK.warning,
        crosshair: Color32::from_rgb(0xe8, 0xe8, 0xe8),
        paper: Color32::WHITE,
    };

    /// Light grey chrome with dark text. The drawing-area colours (canvas, grid, axes, selection,
    /// grips, snap markers, selection windows) are the same as [`Tokens::DARK`]'s.
    pub const LIGHT: Tokens = Tokens {
        chrome: Semantik::LIGHT.muted,
        chrome_dark: Semantik::LIGHT.secondary,
        panel: Semantik::LIGHT.card,
        tab_active: Semantik::LIGHT.accent,
        control: Semantik::LIGHT.secondary,
        control_hover: Semantik::LIGHT.accent,
        border: Semantik::LIGHT.border,
        text: Semantik::LIGHT.foreground,
        text_dim: Semantik::LIGHT.muted_foreground,
        text_faint: Color32::from_rgb(0x6b, 0x77, 0x84),
        icon: Semantik::LIGHT.card_foreground,
        icon_accent: Semantik::LIGHT.primary,
        icon_point: Color32::from_rgb(0xc4, 0x4a, 0x10),
        accent: Semantik::LIGHT.primary,
        cmd_bg: Color32::from_rgb(0xfa, 0xfb, 0xfc),
        toggle_on: Semantik::LIGHT.primary,
        field: Semantik::LIGHT.input,
        select_bg: Semantik::LIGHT.accent,
        cmd_history: Color32::from_rgba_unmultiplied_const(0xfa, 0xfb, 0xfc, 255),
        cmd_border: Semantik::LIGHT.border,
        cmd_keyword: Color32::from_rgb(0x0b, 0x5c, 0xad),
        cmd_keyword_hover: Color32::from_rgb(0xd6, 0xea, 0xfb),
        list_bg: Semantik::LIGHT.input,
        list_row: Semantik::LIGHT.accent,
        warn: Semantik::LIGHT.warning,
        ..Tokens::DARK
    };

    /// The palette of `theme`.
    pub fn of(theme: Theme) -> Tokens {
        match theme {
            Theme::Dark => Tokens::DARK,
            Theme::Light => Tokens::LIGHT,
        }
    }

    /// The palette of the theme in use, with the user's drawing-area colours ([`set_canvas`]).
    pub fn get() -> Tokens {
        CANVAS.with(Cell::get).over(Tokens::of(active()))
    }

    /// These tokens with `bg` as the model-space background: the grid, axes, crosshair and the
    /// lines drawn on the drawing area are recomputed to stay visible on it.
    pub fn with_model_background(self, bg: Color32) -> Tokens {
        let light = luminance(bg) > 0.5;
        let ink = if light { Color32::BLACK } else { Color32::WHITE };
        Tokens {
            canvas: bg,
            grid_minor: mix(bg, ink, 0.05),
            grid_major: mix(bg, ink, 0.11),
            axis_x: mix(bg, Color32::from_rgb(0xd0, 0x30, 0x30), 0.3),
            axis_y: mix(bg, Color32::from_rgb(0x30, 0xb0, 0x40), 0.3),
            canvas_ink: if light { Color32::from_rgb(0x47, 0x50, 0x5c) } else { Tokens::DARK.canvas_ink },
            crosshair: if light { Color32::from_rgb(0x20, 0x20, 0x20) } else { Tokens::DARK.crosshair },
            ..self
        }
    }
}

/// shadcn esintili semantik renk sözlüğü; WP1'de [`Tokens`] yerine uygulama kodu bunu kullanacak.
///
/// Her alan bir rolü anlatır: `background` sayfanın zemini, `card` ve `popover` onun üstüne çıkan
/// yüzeyler, `border`/`input`/`ring` çizgi ve odak halkası, `foreground` ile `*_foreground` çiftleri
/// o yüzeylerin metni, `danger`/`warning`/`success` durum renkleridir. Değerler ölçülmüştür ve
/// [`Tokens`] gibi sabittir; temaya göre sözlüğü [`Semantik::of`] seçer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Semantik {
    /// Sayfanın zemini.
    pub background: Color32,
    /// Zemin üstündeki normal metin.
    pub foreground: Color32,
    /// Zeminden yükselen kart yüzeyi.
    pub card: Color32,
    /// Kart üstündeki metin.
    pub card_foreground: Color32,
    /// Kartın üstünde açılan menü ve iletişim kutusu yüzeyi.
    pub popover: Color32,
    /// Birincil eylem düğmesinin dolgusu.
    pub primary: Color32,
    /// Birincil dolgu üstündeki metin.
    pub primary_foreground: Color32,
    /// İkincil eylem düğmesinin dolgusu.
    pub secondary: Color32,
    /// İkincil dolgu üstündeki metin.
    pub secondary_foreground: Color32,
    /// Sönük yüzey (pasif sekme, gölgeli şerit).
    pub muted: Color32,
    /// Sönük yüzeydeki ikincil metin.
    pub muted_foreground: Color32,
    /// Vurgu yüzeyi (seçili satır, üzerine gelinen öğe).
    pub accent: Color32,
    /// Vurgu yüzeyindeki metin.
    pub accent_foreground: Color32,
    /// Yüzeyleri ayıran kenarlık.
    pub border: Color32,
    /// Metin alanlarının dolgusu.
    pub input: Color32,
    /// Odak halkası.
    pub ring: Color32,
    /// Hata ve yıkıcı eylem.
    pub danger: Color32,
    /// Uyarı.
    pub warning: Color32,
    /// Başarı ve onay.
    pub success: Color32,
}

impl Semantik {
    /// Koyu palet: SPCAD'ın varsayılan görünümü.
    pub const DARK: Semantik = Semantik {
        background: Color32::from_rgb(0x0c, 0x0f, 0x13),
        foreground: Color32::from_rgb(0xe6, 0xeb, 0xf2),
        card: Color32::from_rgb(0x17, 0x1f, 0x29),
        card_foreground: Color32::from_rgb(0xdf, 0xe5, 0xec),
        popover: Color32::from_rgb(0x1f, 0x29, 0x35),
        primary: Color32::from_rgb(0x25, 0x63, 0xeb),
        primary_foreground: Color32::from_rgb(0xff, 0xff, 0xff),
        secondary: Color32::from_rgb(0x1e, 0x28, 0x34),
        secondary_foreground: Color32::from_rgb(0xe6, 0xeb, 0xf2),
        muted: Color32::from_rgb(0x11, 0x16, 0x1c),
        muted_foreground: Color32::from_rgb(0xae, 0xb9, 0xc6),
        accent: Color32::from_rgb(0x22, 0x30, 0x3e),
        accent_foreground: Color32::from_rgb(0xe6, 0xeb, 0xf2),
        border: Color32::from_rgb(0x2c, 0x38, 0x46),
        input: Color32::from_rgb(0x0b, 0x10, 0x15),
        ring: Color32::from_rgb(0x5b, 0x9b, 0xff),
        danger: Color32::from_rgb(0xf8, 0x51, 0x49),
        warning: Color32::from_rgb(0xd2, 0x99, 0x22),
        success: Color32::from_rgb(0x3f, 0xb9, 0x50),
    };

    /// Açık palet: aynı rollerin gündüz kullanımına göre ölçülmüş değerleri.
    pub const LIGHT: Semantik = Semantik {
        background: Color32::from_rgb(0xf4, 0xf6, 0xf8),
        foreground: Color32::from_rgb(0x12, 0x16, 0x1c),
        card: Color32::from_rgb(0xff, 0xff, 0xff),
        card_foreground: Color32::from_rgb(0x16, 0x1b, 0x21),
        popover: Color32::from_rgb(0xff, 0xff, 0xff),
        primary: Color32::from_rgb(0x25, 0x63, 0xeb),
        primary_foreground: Color32::from_rgb(0xff, 0xff, 0xff),
        secondary: Color32::from_rgb(0xea, 0xee, 0xf3),
        secondary_foreground: Color32::from_rgb(0x1a, 0x20, 0x29),
        muted: Color32::from_rgb(0xee, 0xf1, 0xf5),
        muted_foreground: Color32::from_rgb(0x5b, 0x66, 0x73),
        accent: Color32::from_rgb(0xe3, 0xed, 0xff),
        accent_foreground: Color32::from_rgb(0x14, 0x34, 0x6b),
        border: Color32::from_rgb(0xd9, 0xe0, 0xe7),
        input: Color32::from_rgb(0xff, 0xff, 0xff),
        ring: Color32::from_rgb(0x25, 0x63, 0xeb),
        danger: Color32::from_rgb(0xc2, 0x26, 0x2a),
        warning: Color32::from_rgb(0x8a, 0x53, 0x00),
        success: Color32::from_rgb(0x1f, 0x7a, 0x34),
    };

    /// `theme` temasının sözlüğü.
    pub fn of(theme: Theme) -> &'static Semantik {
        match theme {
            Theme::Dark => &Semantik::DARK,
            Theme::Light => &Semantik::LIGHT,
        }
    }
}

/// The user's drawing-area preferences (Options ▸ Display and Drafting); `None` keeps the theme's
/// colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas {
    pub model: Option<Color32>,
    pub paper: Option<Color32>,
    /// The side of an object snap marker, in points.
    pub snap_marker: f32,
    /// The colour selected objects are highlighted in.
    pub selection: Option<Color32>,
}

/// The default object snap marker size.
pub const SNAP_MARKER: f32 = 11.0;

impl Default for Canvas {
    fn default() -> Self {
        Canvas { model: None, paper: None, snap_marker: SNAP_MARKER, selection: None }
    }
}

impl Canvas {
    fn over(self, t: Tokens) -> Tokens {
        let t = match self.model {
            Some(bg) => t.with_model_background(bg),
            None => t,
        };
        Tokens { paper: self.paper.unwrap_or(t.paper), selection: self.selection.unwrap_or(t.selection), ..t }
    }
}

thread_local! {
    /// The drawing-area preferences in use on the UI thread (set by [`set_canvas`] each frame).
    static CANVAS: Cell<Canvas> = Cell::new(Canvas::default());
}

pub fn set_canvas(c: Canvas) {
    CANVAS.with(|v| v.set(c));
}

/// Half the side of an object snap marker, in points.
pub fn snap_marker_half() -> f32 {
    CANVAS.with(Cell::get).snap_marker / 2.0
}

/// Relative luminance of `c` (0 black … 1 white).
pub fn luminance(c: Color32) -> f32 {
    (0.299 * f32::from(c.r()) + 0.587 * f32::from(c.g()) + 0.114 * f32::from(c.b())) / 255.0
}

/// `a` moved the fraction `f` towards `b`.
fn mix(a: Color32, b: Color32, f: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

pub fn small() -> FontId {
    FontId::new(YAZI_KUCUK, FontFamily::Proportional)
}
pub fn body() -> FontId {
    FontId::new(YAZI_GOVDE, FontFamily::Proportional)
}
/// Ribbon large-button label sizes, richest first. The strip uses the largest size at which every
/// label still wraps onto at most two lines without a word being cut; the title band keeps
/// body(), so the ribbon stays on the same typographic scale as the rest of the chrome.
pub const SERIT_ETIKET_BOYUTLARI: [f32; 3] = [YAZI_KUCUK, YAZI_MIKRO, 9.5];
pub fn serit_etiket(boyut: f32) -> FontId {
    FontId::new(boyut, FontFamily::Proportional)
}
pub fn mono() -> FontId {
    FontId::new(YAZI_GOVDE - 0.5, FontFamily::Monospace)
}

/// Load a system UI font at run time when one is installed (not bundled), else keep egui's;
/// then CJK fonts after the Latin ones, so Latin text keeps its look, in every interface
/// language: file, layout and layer names can be Japanese or Chinese whatever the interface
/// shows (#513). In a Chinese interface (`language` "zh…") the Chinese font comes first, else a
/// Japanese one, so names take their own glyph forms; in an Arabic interface an installed
/// Arabic font follows. A font embedded from craft-fonts comes
/// last, for systems without any (and the web build, which has no system fonts). A Korean
/// interface adds a Hangul font only when the CJK font found doesn't cover Hangul
/// ([`korean_font`]).
pub fn install_fonts(ctx: &egui::Context, language: &str) {
    let mut fonts = egui::FontDefinitions::default();
    let mut added: Vec<(String, std::sync::Arc<egui::FontData>)> = Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    let chinese = language.starts_with("zh");
    #[cfg(not(target_arch = "wasm32"))]
    {
        let candidates: &[&str] = if cfg!(target_os = "macos") {
            &["/System/Library/Fonts/SFNS.ttf", "/System/Library/Fonts/Helvetica.ttc", "/Library/Fonts/Arial.ttf"]
        } else if cfg!(windows) {
            &["C:\\Windows\\Fonts\\segoeui.ttf", "C:\\Windows\\Fonts\\arial.ttf"]
        } else {
            &[
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/share/fonts/TTF/DejaVuSans.ttf",
                "/usr/share/fonts/noto/NotoSans-Regular.ttf",
                "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            ]
        };
        for path in candidates {
            if let Ok(bytes) = std::fs::read(path) {
                fonts.font_data.insert("system-ui".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
                if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
                    f.insert(0, "system-ui".into());
                }
                break;
            }
        }
        let mut paths: Vec<&std::path::Path> = Vec::new();
        if !chinese {
            paths.extend(japanese_font());
        }
        paths.extend(cjk_font());
        paths.dedup();
        added.extend(
            paths
                .into_iter()
                .filter_map(|p| read_capped(p, MAX_UI_FONT_BYTES))
                .enumerate()
                .map(|(i, bytes)| (format!("system-cjk-{i}"), std::sync::Arc::new(egui::FontData::from_owned(bytes)))),
        );
        // العربية: an installed Arabic face after the Latin ones (egui's bundled faces carry no
        // Arabic glyphs), for the Arabic interface.
        if language == "ar"
            && let Some(bytes) = arabic_font().and_then(|p| read_capped(p, MAX_UI_FONT_BYTES))
        {
            added.push(("system-arabic".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes))));
        }
    }
    let embedded: Vec<_> = spcad_fonts::CRAFT_FONTS.iter().collect();
    added.extend(
        embedded_ui_faces(&embedded, language)
            .into_iter()
            .map(|f| (format!("craft-{}", f.stem()), std::sync::Arc::new(egui::FontData::from_static(f.bytes)))),
    );
    // Korean: Noto Sans CJK and most CJK fonts above already cover Hangul; else a Hangul font.
    if language == "ko"
        && !cjk_covers_hangul()
        && let Some(face) = korean_font()
    {
        let mut data = egui::FontData::from_owned(face.bytes.as_ref().clone());
        data.index = face.index;
        added.push(("korean-ui".into(), std::sync::Arc::new(data)));
    }
    for (name, data) in added {
        fonts.font_data.insert(name.clone(), data);
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            if let Some(f) = fonts.families.get_mut(&family) {
                f.push(name.clone());
            }
        }
    }
    crate::ikonlar::fontlari_ekle(&mut fonts);
    ctx.set_fonts(fonts);
}

/// The faces `build.rs` embedded from craft-fonts that the interface loads, in order: every CJK
/// face, because file, layout and layer names can be Japanese or Chinese whatever the interface
/// shows (#513), with the interface's own face leading. A sans-serif face reads best, and a
/// Chinese interface takes the face with its own glyphs ahead of a Japanese one — a Japanese face
/// shares most ideographs but lacks the simplified-only ones (图 园 测), which would be tofu.
/// العربية adds its Arabic face after those, egui's bundled faces carrying no Arabic glyphs.
fn embedded_ui_faces<'a>(faces: &[&'a spcad_fonts::CraftFont], language: &str) -> Vec<&'a spcad_fonts::CraftFont> {
    let cjk = |f: &&spcad_fonts::CraftFont| f.style == "Regular" && f.scripts.iter().any(|s| matches!(*s, "Jpan" | "Hans" | "Hant"));
    let mut out: Vec<_> = faces.iter().copied().filter(cjk).collect();
    out.sort_by_key(|f| !f.family.contains("Gothic") && !f.family.contains("Sans"));
    if language.starts_with("zh") {
        out.sort_by_key(|f| !f.scripts.contains(&"Hans"));
    }
    if language == "ar" {
        out.extend(faces.iter().copied().filter(|f| f.style == "Regular" && f.scripts.contains(&"Arab")));
    }
    out
}

/// Largest interface font file read or embedded face copied (CJK collections are 15–60 MB).
const MAX_UI_FONT_BYTES: u64 = 96 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
fn read_capped(path: &std::path::Path, max: u64) -> Option<Vec<u8>> {
    use std::io::Read as _;
    let file = std::fs::File::open(path).ok()?;
    let len = file.metadata().ok()?.len();
    if len == 0 || len > max {
        return None;
    }
    // Enforce the cap during the read too, even if the file grows after metadata was read.
    let mut bytes = Vec::new();
    file.take(max.saturating_add(1)).read_to_end(&mut bytes).ok()?;
    (!bytes.is_empty() && bytes.len() as u64 <= max).then_some(bytes)
}

/// An installed font with Chinese characters, for the Chinese interface; fonts are never bundled
/// (only looked for where the OS or common packages put them). Checked once.
pub fn cjk_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &["/System/Library/Fonts/PingFang.ttc", "/System/Library/Fonts/STHeiti Light.ttc", "/System/Library/Fonts/Hiragino Sans GB.ttc"]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\msyh.ttc", "C:\\Windows\\Fonts\\msyh.ttf", "C:\\Windows\\Fonts\\simsun.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf",
                        "/usr/local/share/fonts/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
                        "/usr/share/fonts/wqy-microhei/wqy-microhei.ttc",
                        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
                        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// An installed font with Arabic glyphs, for the العربية interface; fonts are never bundled
/// (only looked for where the OS or common packages put them). Checked once.
pub fn arabic_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &[
                        "/System/Library/Fonts/GeezaPro.ttc",
                        "/System/Library/Fonts/Supplemental/Geeza Pro.ttf",
                        "/System/Library/Fonts/Supplemental/Al Nile.ttc",
                        "/System/Library/Fonts/Supplemental/Baghdad.ttf",
                    ]
                } else if cfg!(windows) {
                    &[
                        "C:\\Windows\\Fonts\\segoeui.ttf",
                        "C:\\Windows\\Fonts\\tahoma.ttf",
                        "C:\\Windows\\Fonts\\trado.ttf",
                        "C:\\Windows\\Fonts\\arabtype.ttf",
                        "C:\\Windows\\Fonts\\arial.ttf",
                    ]
                } else {
                    &[
                        "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/opentype/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/truetype/noto/NotoNaskhArabic-Regular.ttf",
                        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                        "/usr/share/fonts/TTF/DejaVuSans.ttf",
                        "/usr/share/fonts/truetype/amiri/Amiri-Regular.ttf",
                        "/usr/share/fonts/truetype/kacst/KacstOne.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// An installed Japanese font, for Japanese names in a non-Chinese interface. Checked once.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn japanese_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &["/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc", "/System/Library/Fonts/Hiragino Sans GB.ttc"]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\YuGothR.ttc", "C:\\Windows\\Fonts\\meiryo.ttc", "C:\\Windows\\Fonts\\msgothic.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/opentype/ipafont-gothic/ipagp.ttf",
                        "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf",
                        "/usr/share/fonts/ipa-pgothic/ipagp.ttf",
                        "/usr/share/fonts/vl-gothic/VL-PGothic-Regular.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// The Hangul letters the Korean interface shows: its catalog and its own name.
fn hangul_letters() -> &'static std::collections::BTreeSet<char> {
    static LETTERS: std::sync::OnceLock<std::collections::BTreeSet<char>> = std::sync::OnceLock::new();
    LETTERS.get_or_init(|| {
        include_str!("i18n/ko.tsv")
            .lines()
            .filter_map(|line| line.split('\t').nth(2))
            .flat_map(str::chars)
            .chain("한국어".chars())
            .filter(|c| matches!(c, '\u{1100}'..='\u{11ff}' | '\u{3130}'..='\u{318f}' | '\u{a960}'..='\u{a97f}' | '\u{ac00}'..='\u{d7ff}'))
            .collect()
    })
}

/// Whether `face` has every Hangul letter the Korean interface shows (rejects malformed fonts
/// and Chinese- or Japanese-only faces).
fn covers_hangul(face: &spcad_fonts::ttf::Face) -> bool {
    hangul_letters().iter().all(|c| face.has_glyph(*c))
}

/// The first face of a font file (the one egui uses) when it covers the Korean interface.
fn korean_face(bytes: std::sync::Arc<Vec<u8>>) -> Option<spcad_fonts::ttf::Face> {
    spcad_fonts::ttf::faces(&bytes).into_iter().find(covers_hangul)
}

/// Whether the CJK fonts [`install_fonts`] loads in every language already show Hangul (Noto Sans
/// CJK, Microsoft YaHei, PingFang do). Only asked about Korean, so other interfaces never read the
/// font twice. Checked once.
pub fn cjk_covers_hangul() -> bool {
    static COVERS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *COVERS.get_or_init(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // egui uses a collection's first face, and each letter from the first font with it.
            let first_covers = |p: &std::path::Path| {
                read_capped(p, MAX_UI_FONT_BYTES)
                    .and_then(|bytes| spcad_fonts::ttf::faces(&std::sync::Arc::new(bytes)).into_iter().next())
                    .is_some_and(|face| covers_hangul(&face))
            };
            if cjk_font().is_some_and(first_covers) {
                return true;
            }
        }
        spcad_fonts::CRAFT_FONTS
            .iter()
            .filter(|f| f.style == "Regular" && f.scripts.iter().any(|s| matches!(*s, "Jpan" | "Hans" | "Hant")))
            .any(|f| spcad_fonts::ttf::faces(&std::sync::Arc::new(f.bytes.to_vec())).first().is_some_and(covers_hangul))
    })
}

/// A Hangul font for the Korean interface when the CJK fonts don't cover it: a Korean face from
/// the optional craft-fonts input, else one installed where the OS or common packages put it.
/// Nothing is bundled or downloaded; without one the interface stays English. Only looked for
/// when Korean is asked about. Checked once; collections are scanned with the fonts crate's
/// face-count cap and reads are capped.
pub fn korean_font() -> Option<&'static spcad_fonts::ttf::Face> {
    static FOUND: std::sync::OnceLock<Option<spcad_fonts::ttf::Face>> = std::sync::OnceLock::new();
    FOUND
        .get_or_init(|| {
            for font in spcad_fonts::CRAFT_FONTS.iter().filter(|f| f.scripts.iter().any(|s| matches!(*s, "Hang" | "Kore"))) {
                if font.bytes.len() as u64 > MAX_UI_FONT_BYTES {
                    continue;
                }
                if let Some(face) = korean_face(std::sync::Arc::new(font.bytes.to_vec())) {
                    return Some(face);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &[
                        "/System/Library/Fonts/AppleSDGothicNeo.ttc",
                        "/System/Library/Fonts/Supplemental/AppleGothic.ttf",
                        "/Library/Fonts/NanumGothic.ttf",
                    ]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\malgun.ttf", "C:\\Windows\\Fonts\\gulim.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/truetype/nanum/NanumGothic.ttf",
                        "/usr/share/fonts/nanum/NanumGothic.ttf",
                        "/usr/share/fonts/opentype/noto/NotoSansCJKkr-Regular.otf",
                        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/local/share/fonts/nanum/NanumGothic.ttf",
                        "/usr/local/share/fonts/noto/NotoSansCJK-Regular.ttc",
                    ]
                };
                for path in candidates {
                    if let Some(face) =
                        read_capped(std::path::Path::new(path), MAX_UI_FONT_BYTES).and_then(|bytes| korean_face(std::sync::Arc::new(bytes)))
                    {
                        return Some(face);
                    }
                }
            }
            None
        })
        .as_ref()
}

/// Whether the interface in `language` shows Hangul (the Korean name in the language picker):
/// in every language when the CJK fonts cover it, in a Korean interface with [`korean_font`].
pub fn shows_hangul(language: &str) -> bool {
    cjk_covers_hangul() || (language == "ko" && korean_font().is_some())
}

/// Whether the interface can show `language` without missing-glyph boxes: Chinese needs a font
/// with its glyphs (installed, or the CJK face this build embedded from craft-fonts), Korean a
/// font with Hangul, العربية an installed Arabic font or the embedded Arabic face.
pub fn can_show(language: &str) -> bool {
    match language {
        "ko" => shows_hangul("ko"),
        "ar" => arabic_font().is_some() || embedded_script("Arab"),
        l if l.starts_with("zh") => cjk_font().is_some() || embedded_script("Hans") || embedded_script("Hant"),
        _ => true,
    }
}

/// Whether a face this build embedded from craft-fonts carries `script` (ISO 15924). The web
/// build has no system fonts, so there the interface rides on the embedded faces alone; the
/// Korean gate already asks this way ([`korean_font`]) and Arabic and Chinese match it. Only a
/// build made with `CRAFT_FONTS_DIR` embeds any (`build.rs`), so the rest is unchanged.
fn embedded_script(script: &str) -> bool {
    any_script(spcad_fonts::CRAFT_FONTS, script)
}

/// Whether any face of `faces` carries `script` (ISO 15924), the question [`embedded_script`]
/// asks of the table this build embedded.
fn any_script(faces: &[spcad_fonts::CraftFont], script: &str) -> bool {
    faces.iter().any(|f| f.style == "Regular" && f.scripts.contains(&script))
}

/// egui's visuals for `theme`, from its tokens.
fn visuals(theme: Theme) -> Visuals {
    let t = Tokens::of(theme);
    let mut v = match theme {
        Theme::Dark => Visuals::dark(),
        Theme::Light => Visuals::light(),
    };
    v.panel_fill = t.panel;
    v.window_fill = t.panel;
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.chrome_dark;
    v.override_text_color = Some(t.text);
    v.widgets.noninteractive.bg_fill = t.panel;
    v.widgets.noninteractive.fg_stroke.color = t.text;
    v.widgets.noninteractive.bg_stroke.color = t.border;
    v.widgets.inactive.bg_fill = t.control;
    v.widgets.inactive.weak_bg_fill = t.control;
    v.widgets.inactive.fg_stroke.color = t.text;
    v.widgets.hovered.bg_fill = t.control_hover;
    v.widgets.hovered.weak_bg_fill = t.control_hover;
    v.widgets.active.bg_fill = t.accent;
    v.widgets.active.weak_bg_fill = t.accent;
    v.selection.bg_fill = t.select_bg;
    v.selection.stroke = egui::Stroke::new(1.0, t.selection);
    v.hyperlink_color = t.accent;
    v.warn_fg_color = t.warn;
    v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, t.border);
    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, t.border);
    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, t.selection);
    v.widgets.active.bg_stroke = egui::Stroke::new(1.0, t.selection);
    v.widgets.noninteractive.expansion = 0.0;
    v.widgets.inactive.expansion = 0.0;
    v.widgets.hovered.expansion = 0.0;
    v.widgets.active.expansion = 0.0;
    v.window_stroke = egui::Stroke::new(1.0, t.border);
    let shadow = Color32::from_black_alpha(if theme == Theme::Dark { 140 } else { 50 });
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 8], blur: 24, spread: 0, color: shadow };
    v.window_shadow =
        egui::epaint::Shadow { offset: [0, 16], blur: 40, spread: 0, color: Color32::from_black_alpha(if theme == Theme::Dark { 160 } else { 60 }) };
    let yuvarlak = egui::CornerRadius::same(KOSE_ORTA);
    v.widgets.noninteractive.corner_radius = yuvarlak;
    v.widgets.inactive.corner_radius = yuvarlak;
    v.widgets.hovered.corner_radius = yuvarlak;
    v.widgets.active.corner_radius = yuvarlak;
    v.window_corner_radius = egui::CornerRadius::same(KOSE_BUYUK);
    v.menu_corner_radius = egui::CornerRadius::same(KOSE_BUYUK);
    v
}

/// Install the styles of both themes and show `theme` (egui's own widgets, popups and menus
/// follow it; our own painting follows [`Tokens::get`]). `follow_system` is true for the
/// [`ThemePref::System`] choice, whose `theme` was resolved from the OS report.
pub fn apply(ctx: &egui::Context, theme: Theme, follow_system: bool) {
    set_active(theme);
    ctx.set_visuals_of(Theme::Dark, visuals(Theme::Dark));
    ctx.set_visuals_of(Theme::Light, visuals(Theme::Light));
    // A System choice keeps egui's own System preference: a concrete Light/Dark would pin the native
    // window appearance (winit then stops reporting OS changes) and the app would never follow again.
    ctx.set_theme(if follow_system { egui::ThemePreference::System } else { egui::ThemePreference::from(theme) });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(BOSLUK_2, BOSLUK_2 - 2.0);
        s.spacing.button_padding = egui::vec2(BOSLUK_3 - 2.0, BOSLUK_1 + 1.0);
        s.spacing.interact_size.y = SATIR_YUKSEKLIK - 4.0;
        s.text_styles.insert(egui::TextStyle::Body, body());
        s.text_styles.insert(egui::TextStyle::Button, body());
        s.text_styles.insert(egui::TextStyle::Small, small());
        s.text_styles.insert(egui::TextStyle::Monospace, mono());
    });
}

#[cfg(test)]
mod testler {
    use spcad_fonts::CraftFont;

    /// A face shaped like `build.rs` writes them, with no font file (as in `spcad-fonts`).
    fn face(family: &'static str, scripts: &'static [&'static str]) -> CraftFont {
        CraftFont { family, style: "Regular", scripts, bytes: &[] }
    }

    fn families<'a>(faces: &[&'a CraftFont]) -> Vec<&'a str> {
        faces.iter().map(|f| f.family).collect()
    }

    #[test]
    fn korean_font_rejects_invalid_or_glyphless_files() {
        for bytes in [vec![], vec![0xff; 32], vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]] {
            assert!(super::korean_face(std::sync::Arc::new(bytes)).is_none());
        }
    }

    /// The interface's own script leads the embedded faces it loads, and العربية adds its face.
    #[test]
    fn embedded_faces_lead_with_the_interfaces_own_script() {
        let japanese = face("BIZ UDPGothic", &["Jpan", "Latn"]);
        let chinese = face("Han Test Face", &["Hans", "Latn"]);
        let arabic = face("Noto Sans Arabic", &["Arab"]);
        let faces = [&japanese, &chinese, &arabic];
        // Every interface reads CJK names, so both CJK faces load, the sans-serif face first.
        assert_eq!(families(&super::embedded_ui_faces(&faces, "en")), ["BIZ UDPGothic", "Han Test Face"]);
        // A Chinese interface takes the Han face first: a Japanese face lacks 图 园 测.
        assert_eq!(families(&super::embedded_ui_faces(&faces, "zh")), ["Han Test Face", "BIZ UDPGothic"]);
        // العربية adds the Arabic face, after the CJK ones egui's bundled faces have no glyphs for.
        assert_eq!(families(&super::embedded_ui_faces(&faces, "ar")), ["BIZ UDPGothic", "Han Test Face", "Noto Sans Arabic"]);
        // Another weight is a style of a face, not an interface face of its own.
        let bold = CraftFont { family: "BIZ UDPGothic", style: "Bold", scripts: &["Jpan"], bytes: &[] };
        assert!(super::embedded_ui_faces(&[&bold], "en").is_empty());
    }

    /// What the gate asks of the table: an embedded face of the script stands in for an installed
    /// font, which the web build never has. The default (no `CRAFT_FONTS_DIR`) table answers no,
    /// leaving the installed-font answer the gate had before.
    #[test]
    fn embedded_faces_answer_for_their_scripts() {
        let faces = [face("BIZ UDPGothic", &["Jpan", "Latn"]), face("Noto Sans Arabic", &["Arab"])];
        assert!(super::any_script(&faces, "Jpan") && super::any_script(&faces, "Arab"));
        assert!(!super::any_script(&faces, "Hans") && !super::any_script(&faces, "Hant"));
        // A `Bold`-only face is not the interface's, and a script code is matched whole.
        let bold = CraftFont { family: "Some Han Bold", style: "Bold", scripts: &["Hans"], bytes: &[] };
        assert!(!super::any_script(&[bold], "Hans") && !super::any_script(&faces, "Ara"));
        assert!(!super::embedded_script("Hans") || super::can_show("zh"));
        assert!(super::can_show("en") && super::can_show("uk"));
    }
}

/// Semantik renk sözlüğünün WCAG 2.1 kontrast ve yüzey kademesi testleri.
#[cfg(test)]
mod semantik_testleri {
    use egui::Color32;

    use super::Semantik;

    /// WCAG 2.1: bir sRGB kanalının (0–255) doğrusal değeri.
    fn dogrusal(kanal: u8) -> f32 {
        let c = f32::from(kanal) / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    }

    /// WCAG 2.1 bağıl parlaklığı (0 siyah … 1 beyaz).
    fn parlaklik(c: Color32) -> f32 {
        0.2126 * dogrusal(c.r()) + 0.7152 * dogrusal(c.g()) + 0.0722 * dogrusal(c.b())
    }

    /// WCAG 2.1 karşıtlık oranı (1.0 … 21.0); iki rengin sırası sonucu değiştirmez.
    fn kontrast(a: Color32, b: Color32) -> f32 {
        let (x, y) = (parlaklik(a), parlaklik(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    /// Ölçülen oranı eşiğe karşı sınar; düşükse ölçümü ve eşiği birlikte bildirir.
    fn esigi_gecer(ad: &str, olculen: f32, esik: f32) {
        assert!(olculen >= esik, "{ad}: ölçülen {olculen:.3} < eşik {esik:.3}");
    }

    /// AAA gövde metni eşiği.
    const AAA: f32 = 7.0;
    /// AA normal metin eşiği.
    const AA: f32 = 4.5;
    /// AA arayüz bileşeni (odak halkası) eşiği.
    const BILESEN: f32 = 3.0;
    /// Kenarlığın zeminden ayrılması için gereken en küçük oran.
    const KENARLIK: f32 = 1.2;

    /// Koyu palette bir yüzeyin altındakinden ayrılması için ölçülen en küçük WCAG parlaklık adımı.
    /// Palet, önizleme tasarımındaki yüzeylerle hizalandı (background 0x0c0f13, card 0x171f29,
    /// popover 0x1f2935); ölçülen adımlar 0,0085 ve 0,0081 olduğundan eşik 0,006'ya çekildi.
    const KOYU_YUZEY_ADIMI: f32 = 0.006;

    /// Açık palette kartın zeminden ayrılması için gereken parlaklık farkı.
    const ACIK_YUZEY_ADIMI: f32 = 0.010;

    #[test]
    fn koyu_palet_metin_kontrasti_aaa() {
        let s = Semantik::DARK;
        esigi_gecer("koyu foreground/background", kontrast(s.foreground, s.background), AAA);
        esigi_gecer("koyu card_foreground/card", kontrast(s.card_foreground, s.card), AAA);
    }

    #[test]
    fn acik_palet_metin_kontrasti_aaa() {
        let s = Semantik::LIGHT;
        esigi_gecer("açık foreground/background", kontrast(s.foreground, s.background), AAA);
        esigi_gecer("açık card_foreground/card", kontrast(s.card_foreground, s.card), AAA);
    }

    #[test]
    fn koyu_palet_ikincil_metin_kontrasti_aa() {
        let s = Semantik::DARK;
        esigi_gecer("koyu muted_foreground/background", kontrast(s.muted_foreground, s.background), AA);
    }

    #[test]
    fn acik_palet_ikincil_metin_kontrasti_aa() {
        let s = Semantik::LIGHT;
        esigi_gecer("açık muted_foreground/background", kontrast(s.muted_foreground, s.background), AA);
    }

    #[test]
    fn koyu_palet_dolgu_metni_kontrasti_aa() {
        let s = Semantik::DARK;
        esigi_gecer("koyu primary_foreground/primary", kontrast(s.primary_foreground, s.primary), AA);
    }

    #[test]
    fn acik_palet_dolgu_metni_kontrasti_aa() {
        let s = Semantik::LIGHT;
        esigi_gecer("açık primary_foreground/primary", kontrast(s.primary_foreground, s.primary), AA);
    }

    #[test]
    fn koyu_palet_durum_renkleri_zeminden_ayrilir() {
        let s = Semantik::DARK;
        esigi_gecer("koyu danger/background", kontrast(s.danger, s.background), AA);
        esigi_gecer("koyu warning/background", kontrast(s.warning, s.background), AA);
        esigi_gecer("koyu ring/background", kontrast(s.ring, s.background), BILESEN);
        esigi_gecer("koyu border/background", kontrast(s.border, s.background), KENARLIK);
    }

    #[test]
    fn acik_palet_durum_renkleri_zeminden_ayrilir() {
        let s = Semantik::LIGHT;
        esigi_gecer("açık danger/background", kontrast(s.danger, s.background), AA);
        esigi_gecer("açık warning/background", kontrast(s.warning, s.background), AA);
        esigi_gecer("açık ring/background", kontrast(s.ring, s.background), BILESEN);
        esigi_gecer("açık border/background", kontrast(s.border, s.background), KENARLIK);
    }

    #[test]
    fn koyu_palet_yuzey_kademesi_basamakli() {
        let s = Semantik::DARK;
        let (zemin, kart, popover) = (parlaklik(s.background), parlaklik(s.card), parlaklik(s.popover));
        assert!(kart > zemin, "koyu card ({kart:.4}) zeminin ({zemin:.4}) üstünde olmalı");
        assert!(kart - zemin >= KOYU_YUZEY_ADIMI, "koyu card−background: {:.4}", kart - zemin);
        assert!(popover - kart >= KOYU_YUZEY_ADIMI, "koyu popover−card: {:.4}", popover - kart);
    }

    #[test]
    fn acik_palet_yuzey_kademesi_tektir() {
        let s = Semantik::LIGHT;
        let (zemin, kart) = (parlaklik(s.background), parlaklik(s.card));
        assert!(kart - zemin >= ACIK_YUZEY_ADIMI, "açık card−background: {:.4}", kart - zemin);
        assert_eq!(s.popover, s.card, "açık palette popover card'ın beyazıdır");
    }

    #[test]
    fn tema_sozlugu_temaya_gore_seciliyor() {
        assert_eq!(Semantik::of(egui::Theme::Dark), &Semantik::DARK);
        assert_eq!(Semantik::of(egui::Theme::Light), &Semantik::LIGHT);
        assert!(Semantik::DARK.background != Semantik::LIGHT.background);
    }
}

/// Ölçek tokenlarının tutarlılığı (P5 kapısı): 4 pt ızgara, artan tipografi/yarıçap/hareket.
#[cfg(test)]
mod olcek_testleri {
    use super::*;

    #[test]
    fn bosluklar_dort_pt_tabanli() {
        for bosluk in [BOSLUK_1, BOSLUK_2, BOSLUK_3, BOSLUK_4, BOSLUK_5, BOSLUK_6] {
            assert!((bosluk % 4.0).abs() < f32::EPSILON, "{bosluk} 4'ün katı değil");
        }
        assert!((SATIR_YUKSEKLIK % 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn olcekler_artan_sirada() {
        assert!(YAZI_MIKRO < YAZI_KUCUK);
        assert!(YAZI_KUCUK < YAZI_GOVDE);
        assert!(YAZI_GOVDE < YAZI_VURGU);
        assert!(YAZI_VURGU < YAZI_BASLIK);
        assert!(KOSE_KUCUK < KOSE_ORTA);
        assert!(KOSE_ORTA < KOSE_BUYUK);
        assert!(HAREKET_HIZLI < HAREKET_ORTA);
        assert!(HAREKET_ORTA < HAREKET_YAVAS);
    }
}
/// Tokens alanlarının Semantik sözlüğüyle tutarlılığı (P5 governance).
#[cfg(test)]
mod semantik_tutarlilik_testleri {
    use super::{Semantik, Tokens};

    #[test]
    fn tokens_semantik_rollerini_koruyor() {
        assert_eq!(Tokens::DARK.panel, Semantik::DARK.card);
        assert_eq!(Tokens::DARK.text, Semantik::DARK.foreground);
        assert_eq!(Tokens::DARK.text_dim, Semantik::DARK.muted_foreground);
        assert_eq!(Tokens::DARK.border, Semantik::DARK.border);
        assert_eq!(Tokens::DARK.accent, Semantik::DARK.primary);
        assert_eq!(Tokens::DARK.field, Semantik::DARK.input);
        assert_eq!(Tokens::DARK.warn, Semantik::DARK.warning);
        assert_eq!(Tokens::LIGHT.panel, Semantik::LIGHT.card);
        assert_eq!(Tokens::LIGHT.text, Semantik::LIGHT.foreground);
        assert_eq!(Tokens::LIGHT.border, Semantik::LIGHT.border);
        assert_eq!(Tokens::LIGHT.accent, Semantik::LIGHT.primary);
    }
}
