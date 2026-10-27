//! Genel arayüz ikonları: Lucide seti (iconflow) üzerinden tek kaynak.
//!
//! CAD komut ikonları \`simgeler\` modülünde kalır; burası menü, şerit, palet başlığı,
//! durum çubuğu ve diyalogların ortak simgeleridir. Fontlar \`tema::install_fonts\`
//! içinde ilk kareden önce yüklenir; glyph'ler özel kullanım alanındadır.

use std::collections::HashMap;
use std::sync::OnceLock;

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Painter, Pos2};

/// Arayüzde kullanılan Lucide ikonları.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArayuzIkonu {
    Grid,
    Snap,
    Ortho,
    Polar,
    Isodraft,
    Track,
    Osnap,
    Lineweight,
    Transparency,
    DynInput,
    QuickProps,
    Annotation,
    Workspace,
    Settings,
    Search,
    Layers,
    PanelLeft,
    Close,
    Check,
    ChevronDown,
    Plus,
    Minus,
    File,
    FolderOpen,
    Save,
    Print,
    Undo,
    Redo,
    Table,
    Command,
    Ruler,
    Move,
    Copy,
    Rotate,
    ZoomIn,
    Hand,
    Lock,
    Unlock,
    Freeze,
    Trash,
    Help,
    Warning,
    Info,
    Square,
    Pentagon,
    Cloud,
    Type,
    Box,
    SquarePlus,
    SquarePen,
    PaintBucket,
    Dot,
    Spline,
}

impl ArayuzIkonu {
    /// Tüm ikonlar; eşleme testi bunları gezer.
    pub const ALL: [ArayuzIkonu; 53] = [
        Self::Grid,
        Self::Snap,
        Self::Ortho,
        Self::Polar,
        Self::Isodraft,
        Self::Track,
        Self::Osnap,
        Self::Lineweight,
        Self::Transparency,
        Self::DynInput,
        Self::QuickProps,
        Self::Annotation,
        Self::Workspace,
        Self::Settings,
        Self::Search,
        Self::Layers,
        Self::PanelLeft,
        Self::Close,
        Self::Check,
        Self::ChevronDown,
        Self::Plus,
        Self::Minus,
        Self::File,
        Self::FolderOpen,
        Self::Save,
        Self::Print,
        Self::Undo,
        Self::Redo,
        Self::Table,
        Self::Command,
        Self::Ruler,
        Self::Move,
        Self::Copy,
        Self::Rotate,
        Self::ZoomIn,
        Self::Hand,
        Self::Lock,
        Self::Unlock,
        Self::Freeze,
        Self::Trash,
        Self::Help,
        Self::Warning,
        Self::Info,
        Self::Square,
        Self::Pentagon,
        Self::Cloud,
        Self::Type,
        Self::Box,
        Self::SquarePlus,
        Self::SquarePen,
        Self::PaintBucket,
        Self::Dot,
        Self::Spline,
    ];

    /// Lucide adı.
    pub const fn ad(self) -> &'static str {
        match self {
            Self::Grid => "grid-3x3",
            Self::Snap => "magnet",
            Self::Ortho => "square",
            Self::Polar => "compass",
            Self::Isodraft => "box",
            Self::Track => "crosshair",
            Self::Osnap => "circle-dot",
            Self::Lineweight => "weight",
            Self::Transparency => "blend",
            Self::DynInput => "text-cursor-input",
            Self::QuickProps => "sliders-horizontal",
            Self::Annotation => "eye",
            Self::Workspace => "layout-dashboard",
            Self::Settings => "settings",
            Self::Search => "search",
            Self::Layers => "layers",
            Self::PanelLeft => "panel-left",
            Self::Close => "x",
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::Plus => "plus",
            Self::Minus => "minus",
            Self::File => "file",
            Self::FolderOpen => "folder-open",
            Self::Save => "save",
            Self::Print => "printer",
            Self::Undo => "undo-2",
            Self::Redo => "redo-2",
            Self::Table => "table",
            Self::Command => "command",
            Self::Ruler => "ruler",
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Rotate => "rotate-cw",
            Self::ZoomIn => "zoom-in",
            Self::Hand => "hand",
            Self::Lock => "lock",
            Self::Unlock => "lock-open",
            Self::Freeze => "snowflake",
            Self::Trash => "trash-2",
            Self::Help => "circle-question-mark",
            Self::Warning => "triangle-alert",
            Self::Info => "info",
            Self::Square => "rectangle-horizontal",
            Self::Pentagon => "pentagon",
            Self::Cloud => "cloud",
            Self::Type => "type",
            Self::Box => "package",
            Self::SquarePlus => "square-plus",
            Self::SquarePen => "square-pen",
            Self::PaintBucket => "paint-bucket",
            Self::Dot => "dot",
            Self::Spline => "spline",
        }
    }

    /// Glyph; Lucide adı kurulu sette yoksa \`None\`.
    pub fn glyph(self) -> Option<char> {
        harita().get(&self).copied()
    }
}

/// Lucide codepoint haritası; ilk çözümlemede bir kez kurulur.
fn harita() -> &'static HashMap<ArayuzIkonu, char> {
    static HARITA: OnceLock<HashMap<ArayuzIkonu, char>> = OnceLock::new();
    HARITA.get_or_init(|| {
        use iconflow::{Pack, Size, Style, try_icon};
        let mut harita = HashMap::new();
        for ikon in ArayuzIkonu::ALL {
            if let Ok(ikon_ref) = try_icon(Pack::Lucide, ikon.ad(), Style::Regular, Size::Regular)
                && let Some(glyph) = char::from_u32(ikon_ref.codepoint)
            {
                harita.insert(ikon, glyph);
            }
        }
        harita
    })
}

/// iconflow fontlarını egui font tanımlarına ekler; Latin fontlardan sonra geri düşüş olarak.
pub fn fontlari_ekle(fonts: &mut FontDefinitions) {
    for font in iconflow::fonts() {
        let ad = font.family.to_string();
        fonts.font_data.insert(ad.clone(), std::sync::Arc::new(FontData::from_static(font.bytes)));
        // Özel kullanım alanı (PUA) codepoint'leri sistem fontlarında da bulunabildiğinden
        // (Segoe UI Symbol gibi) ikon fontu listede ÖNE alınır; aksi hâlde yanlış glyph çizilir.
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            if let Some(liste) = fonts.families.get_mut(&family) {
                liste.insert(0, ad.clone());
            }
        }
    }
}

/// Glyph'i tek satır metin olarak çizer; \`Painter\` tabanlı yüzeyler için.
pub fn ciz(painter: &Painter, merkez: Pos2, ikon: ArayuzIkonu, boyut: f32, renk: Color32) {
    if let Some(glyph) = ikon.glyph() {
        painter.text(merkez, egui::Align2::CENTER_CENTER, glyph.to_string(), FontId::new(boyut, FontFamily::Proportional), renk);
    }
}

/// Genel amaçlı CAD ikonlarının Lucide karşılığı; CAD semantiği taşıyanlar `None` kalır.
pub fn cad_esleme(ikon: crate::simgeler::Icon) -> Option<ArayuzIkonu> {
    use crate::simgeler::Icon as C;
    Some(match ikon {
        C::New => ArayuzIkonu::File,
        C::Open => ArayuzIkonu::FolderOpen,
        C::Save | C::SaveAs => ArayuzIkonu::Save,
        C::Undo => ArayuzIkonu::Undo,
        C::Redo => ArayuzIkonu::Redo,
        C::Plot | C::Publish => ArayuzIkonu::Print,
        C::PageSetup | C::Import | C::Export | C::Attach => ArayuzIkonu::File,
        C::Preview => ArayuzIkonu::Annotation,
        C::ZoomWindow | C::ZoomExtents => ArayuzIkonu::ZoomIn,
        C::Pan => ArayuzIkonu::Hand,
        C::Layers => ArayuzIkonu::Layers,
        C::Help => ArayuzIkonu::Help,
        C::Table => ArayuzIkonu::Table,
        C::Erase => ArayuzIkonu::Trash,
        C::Lock => ArayuzIkonu::Lock,
        C::Unlock => ArayuzIkonu::Unlock,
        C::Snowflake => ArayuzIkonu::Freeze,
        C::Grid => ArayuzIkonu::Grid,
        C::Snap => ArayuzIkonu::Snap,
        C::Ortho => ArayuzIkonu::Ortho,
        C::Polar => ArayuzIkonu::Polar,
        C::Osnap => ArayuzIkonu::Osnap,
        C::OTrack => ArayuzIkonu::Track,
        C::Lineweight => ArayuzIkonu::Lineweight,
        C::Transparency => ArayuzIkonu::Transparency,
        C::Isodraft => ArayuzIkonu::Isodraft,
        C::DynInput => ArayuzIkonu::DynInput,
        C::QuickProps => ArayuzIkonu::QuickProps,
        C::Gear => ArayuzIkonu::Settings,
        C::Annotation => ArayuzIkonu::Annotation,
        C::Workspace => ArayuzIkonu::Workspace,
        C::Plus => ArayuzIkonu::Plus,
        C::Close => ArayuzIkonu::Close,
        C::Search => ArayuzIkonu::Search,
        C::Measure | C::Distance => ArayuzIkonu::Ruler,
        C::Rectangle => ArayuzIkonu::Square,
        C::Polygon => ArayuzIkonu::Pentagon,
        C::RevCloud => ArayuzIkonu::Cloud,
        C::Text | C::MText => ArayuzIkonu::Type,
        C::BlockCreate => ArayuzIkonu::Box,
        C::Insert => ArayuzIkonu::SquarePlus,
        C::BlockEdit => ArayuzIkonu::SquarePen,
        C::Hatch => ArayuzIkonu::PaintBucket,
        C::Spline => ArayuzIkonu::Spline,
        _ => return None,
    })
}

#[cfg(test)]
mod testler {
    use super::ArayuzIkonu;

    #[test]
    fn her_arayuz_ikonu_lucide_de_bulunuyor() {
        for ikon in ArayuzIkonu::ALL {
            assert!(ikon.glyph().is_some(), "{ikon:?} ({}) Lucide'de yok", ikon.ad());
        }
    }

    #[test]
    fn adlar_tekil() {
        let mut adlar: Vec<&str> = ArayuzIkonu::ALL.iter().map(|i| i.ad()).collect();
        adlar.sort_unstable();
        adlar.dedup();
        assert_eq!(adlar.len(), ArayuzIkonu::ALL.len());
    }

    #[test]
    fn cad_karsiliklari_lucide_de_bulunuyor() {
        let mut sayi = 0;
        for ikon in crate::simgeler::Icon::ALL {
            if let Some(hedef) = super::cad_esleme(*ikon) {
                assert!(hedef.glyph().is_some(), "{hedef:?}");
                sayi += 1;
            }
        }
        assert!(sayi >= 25, "eşlenen ikon sayısı: {sayi}");
    }
}
