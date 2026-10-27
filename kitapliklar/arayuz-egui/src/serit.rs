//! The ribbon: a strip of tabs, each holding command panels with large and stacked small
//! buttons, laid out like the reference CAD ribbon. Commands are the same ones the menus and the
//! command line run; the Layers panel carries the current-layer dropdown and the Properties
//! panel edits the current colour, linetype and lineweight in a 2×2 grid. The strip measures its
//! labels and keeps every panel of the active tab on one row: panels shrink through tiers (labelled
//! large buttons, icon-only small columns, title-only) and the spare width is handed to the panels
//! with the most content still hidden, so no group disappears behind an overflow menu.

#![allow(clippy::disallowed_methods)]

use egui::{Align2, FontId, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::CadApp;
use crate::simgeler::{self, Icon};
use crate::tema::{self, Tokens};

pub struct Dugme {
    pub icon: Icon,
    pub id: &'static str,
    pub label: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tur {
    Komutlar,
    Katmanlar,
    Ozellikler,
}

pub struct Panel {
    pub baslik: &'static str,
    pub buyuk: &'static [Dugme],
    pub kucuk: &'static [Dugme],
    pub tur: Tur,
}

pub struct Sekme {
    pub id: &'static str,
    pub baslik: &'static str,
    pub paneller: &'static [Panel],
}

const fn d(icon: Icon, id: &'static str, label: &'static str) -> Dugme {
    Dugme { icon, id, label }
}

const fn p(baslik: &'static str, buyuk: &'static [Dugme], kucuk: &'static [Dugme]) -> Panel {
    Panel { baslik, buyuk, kucuk, tur: Tur::Komutlar }
}

macro_rules! panel {
    ($ad:literal, [$($b:expr),* $(,)?], [$($s:expr),* $(,)?]) => {
        p($ad, &[$($b),*], &[$($s),*])
    };
}

pub const SEKMELER: &[Sekme] = &[
    Sekme {
        id: "home",
        baslik: "Home",
        paneller: &[
            panel!(
                "Draw",
                [
                    d(Icon::Line, "line", "Line"),
                    d(Icon::Polyline, "pline", "Polyline"),
                    d(Icon::Circle, "circle", "Circle"),
                    d(Icon::Arc, "arc", "Arc"),
                ],
                [
                    d(Icon::Rectangle, "rectang", "Rectangle"),
                    d(Icon::Polygon, "polygon", "Polygon"),
                    d(Icon::Ellipse, "ellipse", "Ellipse"),
                    d(Icon::Spline, "spline", "Spline"),
                    d(Icon::Point, "point.multiple", "Multiple Point"),
                    d(Icon::XLine, "xline", "Construction Line"),
                    d(Icon::Ray, "ray", "Ray"),
                    d(Icon::Donut, "donut", "Donut"),
                    d(Icon::Hatch, "hatch", "Hatch"),
                    d(Icon::Gradient, "gradient", "Gradient"),
                    d(Icon::Boundary, "boundary", "Boundary"),
                    d(Icon::RevCloud, "revcloud", "Revision Cloud"),
                    d(Icon::Region, "region", "Region"),
                    d(Icon::Wipeout, "wipeout", "Wipeout"),
                ]
            ),
            panel!(
                "Modify",
                [d(Icon::Move, "move", "Move"), d(Icon::Copy, "copy", "Copy"), d(Icon::Rotate, "rotate", "Rotate"), d(Icon::Trim, "trim", "Trim")],
                [
                    d(Icon::Mirror, "mirror", "Mirror"),
                    d(Icon::Scale, "scale", "Scale"),
                    d(Icon::Stretch, "stretch", "Stretch"),
                    d(Icon::Offset, "offset", "Offset"),
                    d(Icon::ArrayRect, "arrayrect", "Rectangular Array"),
                    d(Icon::ArrayPolar, "arraypolar", "Polar Array"),
                    d(Icon::Fillet, "fillet", "Fillet"),
                    d(Icon::Chamfer, "chamfer", "Chamfer"),
                    d(Icon::Explode, "explode", "Explode"),
                    d(Icon::Erase, "erase", "Erase"),
                    d(Icon::Break, "break", "Break"),
                    d(Icon::Join, "join", "Join"),
                    d(Icon::Lengthen, "lengthen", "Lengthen"),
                    d(Icon::MatchProp, "matchprop", "Match Properties"),
                ]
            ),
            panel!(
                "Annotation",
                [
                    d(Icon::Text, "text", "Text"),
                    d(Icon::MText, "mtext", "Multiline Text"),
                    d(Icon::DimLinear, "dimlinear", "Linear"),
                    d(Icon::MLeader, "mleader", "Multileader")
                ],
                [
                    d(Icon::DimAligned, "dimaligned", "Aligned"),
                    d(Icon::DimAngular, "dimangular", "Angular"),
                    d(Icon::DimRadius, "dimradius", "Radius"),
                    d(Icon::DimDiameter, "dimdiameter", "Diameter"),
                    d(Icon::DimArc, "dimarc", "Arc Length"),
                    d(Icon::DimOrdinate, "dimordinate", "Ordinate"),
                    d(Icon::DimBaseline, "dimbaseline", "Baseline"),
                    d(Icon::DimContinue, "dimcontinue", "Continue"),
                    d(Icon::QDim, "qdim", "Quick Dimension"),
                    d(Icon::Table, "table", "Table"),
                    d(Icon::Search, "find", "Find and Replace"),
                    d(Icon::Annotation, "annoupdate", "Update Annotation"),
                ]
            ),
            Panel {
                baslik: "Layers",
                buyuk: &[d(Icon::LayerProps, "ui.dialog.layers", "Layer Properties")],
                kucuk: &[
                    d(Icon::LayerIso, "layiso", "Isolate"),
                    d(Icon::LayerFreeze, "layfrz", "Freeze Layer"),
                    d(Icon::LayerOff, "layoff", "Turn Off Layer"),
                    d(Icon::LayerLock, "laylck", "Lock Layer"),
                    d(Icon::Unlock, "layulk", "Unlock Layer"),
                    d(Icon::MakeCurrent, "laymcur", "Make Object's Layer Current"),
                    d(Icon::LayerMatch, "laymch", "Match Layer"),
                    d(Icon::LayerPrev, "layerp", "Previous Layer"),
                    d(Icon::Bulb, "layon", "Turn On All Layers"),
                    d(Icon::Sun, "laythw", "Thaw All Layers"),
                    d(Icon::LayerIso, "layuniso", "Unisolate"),
                    d(Icon::List, "layerstate.list", "Layer States"),
                ],
                tur: Tur::Katmanlar,
            },
            panel!(
                "Block",
                [d(Icon::BlockCreate, "block", "Create Block"), d(Icon::Insert, "insert", "Insert")],
                [
                    d(Icon::BlockEdit, "bedit", "Block Editor"),
                    d(Icon::AttDef, "attdef", "Define Attributes"),
                    d(Icon::AttDef, "attsync", "Synchronize Attributes"),
                    d(Icon::Blocks, "blocks.list", "Block List"),
                    d(Icon::Blocks, "bclose", "Close Block Editor"),
                ]
            ),
            Panel { baslik: "Properties", buyuk: &[], kucuk: &[], tur: Tur::Ozellikler },
            panel!(
                "Groups",
                [d(Icon::Blocks, "group", "Group"), d(Icon::Close, "ungroup", "Ungroup")],
                [
                    d(Icon::List, "groupedit", "Edit Group"),
                    d(Icon::SelectObjects, "selectsimilar", "Select Similar"),
                    d(Icon::QuickSelect, "qselect", "Quick Select"),
                    d(Icon::PickAdd, "selectioncycling", "Selection Cycling"),
                ]
            ),
            panel!(
                "Utilities",
                [d(Icon::Measure, "measure", "Measure"), d(Icon::Area, "area", "Area")],
                [
                    d(Icon::Distance, "dist", "Measure Distance"),
                    d(Icon::List, "list", "List"),
                    d(Icon::QuickProps, "id", "ID Point"),
                    d(Icon::List, "count", "Count"),
                    d(Icon::Measure, "massprop", "Mass Properties"),
                    d(Icon::Search, "audit", "Audit"),
                    d(Icon::Erase, "purge", "Purge"),
                    d(Icon::Gear, "overkill", "Overkill"),
                    d(Icon::Properties, "status", "Drawing Status"),
                ]
            ),
            panel!(
                "Clipboard",
                [d(Icon::Paste, "pasteclip", "Paste")],
                [
                    d(Icon::Cut, "cutclip", "Cut"),
                    d(Icon::Copy, "copyclip", "Copy"),
                    d(Icon::Copy, "copybase", "Copy with Base Point"),
                    d(Icon::Paste, "pasteorig", "Paste to Original Coordinates"),
                    d(Icon::Paste, "pasteblock", "Paste as Block"),
                ]
            ),
            panel!(
                "View",
                [d(Icon::Pan, "pan", "Pan"), d(Icon::ZoomExtents, "zoom.extents", "Zoom Extents")],
                [
                    d(Icon::ZoomWindow, "zoom.window", "Zoom Window"),
                    d(Icon::Circle, "zoom.object", "Zoom Object"),
                    d(Icon::Plus, "zoom.in", "Zoom In"),
                    d(Icon::Close, "zoom.out", "Zoom Out"),
                    d(Icon::Orbit, "3dorbit", "Orbit"),
                    d(Icon::Switcher, "zoom.previous", "Zoom Previous"),
                    d(Icon::Gear, "regen", "Regen"),
                    d(Icon::Gear, "regenall", "Regen All"),
                ]
            ),
        ],
    },
    Sekme {
        id: "insert",
        baslik: "Insert",
        paneller: &[
            panel!(
                "Block",
                [d(Icon::Insert, "insert", "Insert"), d(Icon::BlockCreate, "block", "Create Block")],
                [
                    d(Icon::BlockEdit, "bedit", "Block Editor"),
                    d(Icon::AttDef, "attdef", "Define Attributes"),
                    d(Icon::AttDef, "attsync", "Synchronize Attributes"),
                    d(Icon::Blocks, "blocks.list", "Block List"),
                ]
            ),
            panel!(
                "Reference",
                [d(Icon::Attach, "xattach", "Attach Reference"), d(Icon::Blocks, "xref", "External References")],
                [
                    d(Icon::Insert, "xbind", "Bind"),
                    d(Icon::Open, "xopen", "Open Reference"),
                    d(Icon::BlockEdit, "refedit", "Edit Reference"),
                    d(Icon::Close, "refclose", "Close Reference"),
                ]
            ),
            panel!(
                "Import",
                [d(Icon::Import, "import", "Import")],
                [
                    d(Icon::Insert, "insert", "Insert Block"),
                    d(Icon::Table, "dataextraction", "Data Extraction"),
                    d(Icon::Insert, "datalink", "Data Link")
                ]
            ),
            panel!(
                "Data",
                [d(Icon::Table, "table", "Table")],
                [
                    d(Icon::List, "dataextraction", "Data Extraction"),
                    d(Icon::Insert, "datalink", "Data Link"),
                    d(Icon::Table, "tablestyle", "Table Style"),
                ]
            ),
        ],
    },
    Sekme {
        id: "annotate",
        baslik: "Annotate",
        paneller: &[
            panel!(
                "Text",
                [d(Icon::Text, "text", "Text"), d(Icon::MText, "mtext", "Multiline Text")],
                [
                    d(Icon::Search, "find", "Find and Replace"),
                    d(Icon::Text, "textedit", "Edit Text"),
                    d(Icon::MText, "txt2mtxt", "Convert to Multiline Text"),
                ]
            ),
            panel!(
                "Dimension",
                [d(Icon::DimLinear, "dimlinear", "Linear"), d(Icon::DimAligned, "dimaligned", "Aligned")],
                [
                    d(Icon::DimAngular, "dimangular", "Angular"),
                    d(Icon::DimRadius, "dimradius", "Radius"),
                    d(Icon::DimDiameter, "dimdiameter", "Diameter"),
                    d(Icon::DimArc, "dimarc", "Arc Length"),
                    d(Icon::DimOrdinate, "dimordinate", "Ordinate"),
                    d(Icon::DimBaseline, "dimbaseline", "Baseline"),
                    d(Icon::DimContinue, "dimcontinue", "Continue"),
                    d(Icon::QDim, "qdim", "Quick Dimension"),
                ]
            ),
            panel!(
                "Leader",
                [d(Icon::MLeader, "mleader", "Multileader")],
                [
                    d(Icon::AddLeader, "mleaderedit.add", "Add Leader"),
                    d(Icon::RemoveLeader, "mleaderedit.remove", "Remove Leader"),
                    d(Icon::AlignLeaders, "mleaderalign", "Align"),
                    d(Icon::MLeader, "mleaderedit", "Edit Multileader"),
                    d(Icon::MLeader, "mleaderstyle", "Multileader Style"),
                ]
            ),
            panel!(
                "Table",
                [d(Icon::Table, "table", "Table")],
                [
                    d(Icon::Table, "tabledit", "Edit Table"),
                    d(Icon::Table, "tablestyle", "Table Style"),
                    d(Icon::Export, "tableexport", "Export Table"),
                ]
            ),
            panel!(
                "Markup",
                [d(Icon::RevCloud, "revcloud", "Revision Cloud")],
                [
                    d(Icon::Annotation, "annoupdate", "Update Annotation"),
                    d(Icon::Redo, "annoreset", "Reset Annotation"),
                    d(Icon::Annotation, "annovisible.toggle", "Annotation Visibility"),
                ]
            ),
            panel!(
                "Styles",
                [d(Icon::DimLinear, "dimstyle", "Dimension Style"), d(Icon::Text, "style", "Text Style")],
                [d(Icon::Table, "tablestyle", "Table Style"), d(Icon::MLeader, "mleaderstyle", "Multileader Style"),]
            ),
        ],
    },
    Sekme {
        id: "parametric",
        baslik: "Parametric",
        paneller: &[
            panel!(
                "Geometric",
                [d(Icon::Constraint, "autoconstrain", "Auto Constrain"), d(Icon::DelConstraint, "delconstraint", "Delete Constraints")],
                [
                    d(Icon::Coincident, "gccoincident", "Coincident"),
                    d(Icon::Parallel, "gcparallel", "Parallel"),
                    d(Icon::Perpendicular, "gcperpendicular", "Perpendicular"),
                    d(Icon::Horizontal, "gchorizontal", "Horizontal"),
                    d(Icon::Vertical, "gcvertical", "Vertical"),
                    d(Icon::Tangent, "gctangent", "Tangent"),
                    d(Icon::Concentric, "gcconcentric", "Concentric"),
                    d(Icon::Equal, "gcequal", "Equal"),
                    d(Icon::Collinear, "gccollinear", "Collinear"),
                    d(Icon::Smooth, "gcsmooth", "Smooth"),
                    d(Icon::Symmetric, "gcsymmetric", "Symmetric"),
                    d(Icon::Fix, "gcfix", "Fix"),
                ]
            ),
            panel!(
                "Dimensional",
                [d(Icon::DimLinear, "dclinear", "Linear"), d(Icon::DimAligned, "dcaligned", "Aligned"), d(Icon::DimAngular, "dcangular", "Angular")],
                [
                    d(Icon::DimRadius, "dcradius", "Radius"),
                    d(Icon::DimDiameter, "dcdiameter", "Diameter"),
                    d(Icon::DimLinear, "dimconstraint", "Convert to Constraint"),
                ]
            ),
            panel!(
                "Manage",
                [d(Icon::Parameters, "parameters", "Parameters Manager"), d(Icon::DelConstraint, "delconstraint", "Delete Constraints")],
                [
                    d(Icon::Constraint, "constraintbar", "Show All Constraints"),
                    d(Icon::Gear, "constraintsettings", "Constraint Settings"),
                    d(Icon::Constraint, "constraintinfer", "Infer Constraints"),
                    d(Icon::List, "constraints.dof", "Degrees of Freedom"),
                    d(Icon::Search, "constraints.inspect", "Inspect Constraints"),
                ]
            ),
        ],
    },
    Sekme {
        id: "view",
        baslik: "View",
        paneller: &[
            panel!(
                "Navigate",
                [d(Icon::Pan, "pan", "Pan"), d(Icon::ZoomExtents, "zoom.extents", "Zoom Extents")],
                [
                    d(Icon::ZoomWindow, "zoom.window", "Zoom Window"),
                    d(Icon::Orbit, "3dorbit", "Orbit"),
                    d(Icon::Plus, "zoom.in", "Zoom In"),
                    d(Icon::Close, "zoom.out", "Zoom Out"),
                    d(Icon::Switcher, "zoom.previous", "Zoom Previous"),
                    d(Icon::Gear, "zoom.all", "Zoom All"),
                ]
            ),
            panel!(
                "Palettes",
                [d(Icon::Layers, "ui.dialog.layers", "Layer Properties"), d(Icon::Properties, "ui.panel.properties", "Properties Inspector")],
                [
                    d(Icon::Blocks, "ui.dialog.blocks", "Blocks"),
                    d(Icon::Gear, "ui.toggle.shelf", "Tool Shelf"),
                    d(Icon::ChevronLeft, "ui.shelf.left", "Tool Shelf on the Left"),
                    d(Icon::ChevronRight, "ui.shelf.right", "Tool Shelf on the Right"),
                    d(Icon::Switcher, "ui.toggle.palettes", "Palettes"),
                ]
            ),
            panel!(
                "Interface",
                [d(Icon::Workspace, "ui.toggle.viewcube", "ViewCube"), d(Icon::Switcher, "ui.toggle.filetabs", "File Tab")],
                [
                    d(Icon::Workspace, "ui.toggle.ucsicon", "UCS Icon"),
                    d(Icon::Gear, "ui.toggle.statusbar", "Status Bar"),
                    d(Icon::Menu, "ui.toggle.menubar", "In-window Menu Bar"),
                    d(Icon::Gear, "ui.dialog.dsettings", "Drafting Settings"),
                ]
            ),
            panel!(
                "Display",
                [d(Icon::Grid, "grid", "Grid"), d(Icon::Snap, "snap", "Snap Mode")],
                [
                    d(Icon::Ortho, "ortho", "Ortho Mode"),
                    d(Icon::Polar, "polar", "Polar Tracking"),
                    d(Icon::Osnap, "osnap", "Object Snap"),
                    d(Icon::OTrack, "otrack", "Object Snap Tracking"),
                    d(Icon::DynInput, "dynmode", "Dynamic Input"),
                    d(Icon::Lineweight, "lwdisplay", "Show/Hide Lineweight"),
                    d(Icon::Transparency, "transparencydisplay", "Show/Hide Transparency"),
                    d(Icon::Isodraft, "isodraft", "Isometric Drafting"),
                ]
            ),
        ],
    },
    Sekme {
        id: "manage",
        baslik: "Manage",
        paneller: &[
            panel!(
                "Drawing",
                [d(Icon::Search, "audit", "Audit"), d(Icon::Open, "recover", "Recover")],
                [
                    d(Icon::Open, "drawingrecovery", "Drawing Recovery"),
                    d(Icon::Erase, "purge", "Purge"),
                    d(Icon::Gear, "overkill", "Overkill"),
                    d(Icon::Properties, "status", "Drawing Status"),
                ]
            ),
            panel!(
                "Styles",
                [d(Icon::DimLinear, "dimstyle", "Dimension Style"), d(Icon::Text, "style", "Text Style")],
                [
                    d(Icon::Table, "tablestyle", "Table Style"),
                    d(Icon::MLeader, "mleaderstyle", "Multileader Style"),
                    d(Icon::Gear, "plotstyle", "Plot Style"),
                ]
            ),
            panel!(
                "Applications",
                [d(Icon::Insert, "appload", "Load Application"), d(Icon::Export, "script", "Run Script")],
                [d(Icon::Menu, "aliasedit", "Alias Editor"), d(Icon::Gear, "reinit", "Reinitialize"), d(Icon::List, "sysvars", "System Variables"),]
            ),
            panel!(
                "Drawing Utilities",
                [d(Icon::Gear, "dsettings", "Drafting Settings"), d(Icon::Properties, "units", "Units")],
                [d(Icon::Properties, "limits", "Drawing Limits"), d(Icon::List, "time", "Time")]
            ),
        ],
    },
    Sekme {
        id: "output",
        baslik: "Output",
        paneller: &[
            panel!(
                "Plot",
                [d(Icon::Plot, "plot", "Print"), d(Icon::Preview, "preview", "Plot Preview")],
                [
                    d(Icon::PageSetup, "pagesetup", "Page Setup Manager"),
                    d(Icon::Plot, "plottermanager", "Plotter Manager"),
                    d(Icon::Gear, "plotstyle", "Plot Style"),
                    d(Icon::Plot, "plotwindow", "Plot Window"),
                ]
            ),
            panel!(
                "Export",
                [d(Icon::Export, "export", "Export"), d(Icon::Export, "exportpdf", "Export PDF")],
                [d(Icon::Export, "pdfbook", "PDF Book"), d(Icon::PageSetup, "pagesetup", "Page Setup Manager")]
            ),
            panel!("Publish", [d(Icon::Publish, "publish", "Batch Publish")], []),
        ],
    },
];

/// Window rectangle of the tab strip and body.
pub const SEKME_Y: f32 = 30.0;
pub const GOVDE_Y: f32 = 94.0;
const TUR_H: f32 = 18.0;
const SATIR_H: f32 = 21.0;
const IC_BOSLUK: f32 = 6.0;
/// A large button that shows its label under the icon; the width follows the longest word of
/// the label, so no label is cut with an ellipsis.
const BUYUK_ETIKET_EN_KUCUK: f32 = 40.0;
const BUYUK_ETIKET_EN_BUYUK: f32 = 120.0;
/// Horizontal breathing room inside a labelled large button: the text never fills the whole
/// button, so two adjacent labels cannot visually run into each other.
const BUYUK_METIN_BOSLUK: f32 = 12.0;
/// A large button reduced to its icon (the label becomes the tooltip).
const BUYUK_IKON_G: f32 = 28.0;
/// One column of three icon-only small buttons.
const KUCUK_IKON_G: f32 = 20.0;
const OZELLIK_G: f32 = 300.0;
const OZELLIK_DAR_G: f32 = 132.0;
const KATMAN_LISTE_G: f32 = 150.0;
const KATMAN_LISTE_EN_AZ: f32 = 100.0;
const KOLON_EN_KUCUK: f32 = 56.0;
const KOLON_EN_BUYUK: f32 = 132.0;
const PANEL_SAG_BOSLUK: f32 = 2.0;

fn var(id: &str) -> bool {
    spcad_engine::find_command(id).is_some() || crate::menuler::is_ui_command(id)
}

fn kes(p: &egui::Painter, text: &str, font: FontId, color: egui::Color32, w: f32) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), font, color);
    job.wrap = egui::text::TextWrapping { max_width: w.max(0.0), max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    p.layout_job(job)
}

/// The measured width of a single-line label.
fn metin_g(ui: &egui::Ui, yazi: &str, boyut: f32) -> f32 {
    ui.painter().layout_no_wrap(yazi.to_owned(), tema::serit_etiket(boyut), egui::Color32::WHITE).size().x
}

fn baslik_genisligi(ui: &egui::Ui, panel: &Panel) -> f32 {
    metin_g(ui, crate::i18n::t(panel.baslik), tema::body().size) + 26.0
}

fn etiketli_kolon(ui: &egui::Ui, label: &str, boyut: f32) -> f32 {
    (metin_g(ui, crate::i18n::t(label), boyut) + 23.0).clamp(KOLON_EN_KUCUK, KOLON_EN_BUYUK)
}

fn etiketli_kolonlar(ui: &egui::Ui, panel: &Panel, boyut: f32) -> Vec<f32> {
    panel.kucuk.chunks(3).map(|c| c.iter().map(|b| etiketli_kolon(ui, b.label, boyut)).fold(0.0, f32::max)).collect()
}

/// The width a large button needs for its label to wrap onto at most two lines. The wrap only
/// breaks between words, so the minimum is the best two-line split; the renderer lays the same
/// text out at this width, which is what keeps long translations from being cut mid-word.
fn buyuk_etiket_gerekli(ui: &egui::Ui, b: &Dugme, boyut: f32) -> f32 {
    let yazi = crate::i18n::t(b.label);
    let kelimeler: Vec<f32> = yazi.split_whitespace().map(|k| metin_g(ui, k, boyut)).collect();
    let en = match kelimeler.as_slice() {
        [] => 0.0,
        [tek] => *tek,
        _ => {
            let bosluk = metin_g(ui, " ", boyut);
            let toplam: f32 = kelimeler.iter().sum::<f32>() + bosluk * (kelimeler.len() - 1) as f32;
            let mut ilk = 0.0;
            let mut en_iyi = toplam;
            for (i, kelime) in kelimeler.iter().enumerate() {
                ilk += kelime + if i == 0 { 0.0 } else { bosluk };
                if i + 1 < kelimeler.len() {
                    let ikinci = toplam - ilk - bosluk;
                    en_iyi = en_iyi.min(ilk.max(ikinci));
                }
            }
            en_iyi
        }
    };
    en + 6.0
}

fn buyuk_etiket_genisligi(ui: &egui::Ui, b: &Dugme, boyut: f32) -> f32 {
    (buyuk_etiket_gerekli(ui, b, boyut) + BUYUK_METIN_BOSLUK).clamp(BUYUK_ETIKET_EN_KUCUK, BUYUK_ETIKET_EN_BUYUK)
}

fn buyuk_kolon_genisligi(ui: &egui::Ui, panel: &Panel, etiketli: bool, boyut: f32, adet: usize) -> f32 {
    if etiketli { panel.buyuk.iter().take(adet).map(|b| buyuk_etiket_genisligi(ui, b, boyut)).sum() } else { adet as f32 * BUYUK_IKON_G }
}

/// The width below which even the icons of the drawn large buttons no longer fit.
fn asgari_genislik(ui: &egui::Ui, panel: &Panel, adet: usize) -> f32 {
    let bas = baslik_genisligi(ui, panel);
    match panel.tur {
        Tur::Ozellikler => bas.max(OZELLIK_DAR_G),
        Tur::Komutlar | Tur::Katmanlar => bas.max(IC_BOSLUK * 2.0 + buyuk_kolon_genisligi(ui, panel, false, 0.0, adet)),
    }
}

/// The width at which the drawn large buttons carry their labels at the given size.
fn etiketli_genislik(ui: &egui::Ui, panel: &Panel, boyut: f32, adet: usize) -> f32 {
    let bas = baslik_genisligi(ui, panel);
    match panel.tur {
        Tur::Ozellikler => bas.max(OZELLIK_DAR_G),
        Tur::Komutlar | Tur::Katmanlar => bas.max(IC_BOSLUK * 2.0 + buyuk_kolon_genisligi(ui, panel, true, boyut, adet)),
    }
}

/// The width that shows every command of the panel at the given label size.
fn tam_genislik(ui: &egui::Ui, panel: &Panel, boyut: f32, adet: usize) -> f32 {
    let bas = baslik_genisligi(ui, panel);
    if panel.tur == Tur::Ozellikler {
        return bas.max(OZELLIK_G);
    }
    let liste = if panel.tur == Tur::Katmanlar { KATMAN_LISTE_G + IC_BOSLUK } else { 0.0 };
    let kolonlar: f32 = etiketli_kolonlar(ui, panel, boyut).iter().sum();
    bas.max(IC_BOSLUK * 2.0 + liste + buyuk_kolon_genisligi(ui, panel, true, boyut, adet) + kolonlar)
}

fn toplam(v: &[f32]) -> f32 {
    v.iter().sum()
}

/// One panel on the strip: its width, the label size to draw, and how many of its large buttons
/// are visible (the rest stay in its title menu).
#[derive(Clone, Copy, Debug)]
struct PanelPlani {
    genislik: f32,
    boyut: f32,
    buyuk: usize,
}

fn adetler(sekme: &Sekme) -> Vec<usize> {
    sekme.paneller.iter().map(|p| p.buyuk.len()).collect()
}

/// The labelled width of the tab at the given size, or infinity when even one visible label would
/// need more than two lines (a word longer than a button can hold is never drawn cut; the plan
/// trims that button into the panel menu instead).
fn etiketli_toplam(ui: &egui::Ui, sekme: &Sekme, boyut: f32, adet: &[usize]) -> f32 {
    let mut toplam = 0.0;
    for (panel, n) in sekme.paneller.iter().zip(adet) {
        for b in panel.buyuk.iter().take(*n) {
            if buyuk_etiket_gerekli(ui, b, boyut) + BUYUK_METIN_BOSLUK > BUYUK_ETIKET_EN_BUYUK {
                return f32::INFINITY;
            }
        }
        toplam += etiketli_genislik(ui, panel, boyut, *n);
    }
    toplam
}

fn plan_genislikleri(ui: &egui::Ui, sekme: &Sekme, boyut: f32, adet: &[usize], taban: &[f32], kullanilabilir: f32) -> Vec<f32> {
    let alt: Vec<f32> = sekme.paneller.iter().zip(adet).map(|(p, n)| etiketli_genislik(ui, p, boyut, *n)).collect();
    let ust: Vec<f32> = sekme.paneller.iter().zip(adet).map(|(p, n)| tam_genislik(ui, p, boyut, *n)).collect();
    dagit(taban, &alt, &ust, kullanilabilir)
}

/// Splits the strip width between the panels. The richest tier that fits everywhere sets the
/// base; the remaining width goes to the panels with the most content still to reveal, so the row
/// always fills the strip and no dead gap is left at the right edge.
fn dagit(taban: &[f32], alt: &[f32], ust: &[f32], kullanilabilir: f32) -> Vec<f32> {
    let alt_toplam = toplam(alt);
    if alt_toplam <= kullanilabilir {
        let pay = kullanilabilir - alt_toplam;
        let fark: Vec<f32> = alt.iter().zip(ust).map(|(a, u)| (u - a).max(0.0)).collect();
        let fark_toplam = toplam(&fark);
        if fark_toplam <= 0.0 {
            let esit = pay / alt.len().max(1) as f32;
            return alt.iter().map(|w| w + esit).collect();
        }
        return alt.iter().enumerate().map(|(i, w)| w + pay * fark[i] / fark_toplam).collect();
    }
    let taban_toplam = toplam(taban);
    if taban_toplam <= kullanilabilir {
        let pay = kullanilabilir - taban_toplam;
        let fark: Vec<f32> = taban.iter().zip(alt).map(|(a, u)| (u - a).max(0.0)).collect();
        let fark_toplam = toplam(&fark);
        if fark_toplam <= 0.0 {
            return taban.to_vec();
        }
        return taban.iter().enumerate().map(|(i, w)| w + pay * fark[i] / fark_toplam).collect();
    }
    let k = kullanilabilir / taban_toplam.max(1.0);
    taban.iter().map(|w| w * k).collect()
}

/// How many large buttons a label size may move into the panel menu before the next smaller size
/// is preferred: a couple of overflow tools are acceptable, a ribbon of tiny labels is not.
const EN_FAZLA_KESILEN: usize = 4;

/// Moves the trailing large buttons of the fullest panels into their menus until the labelled tier
/// fits the strip. Returns the visible counts and how many buttons were moved.
fn keserek_sigdir(ui: &egui::Ui, sekme: &Sekme, boyut: f32, kullanilabilir: f32) -> Option<(Vec<usize>, usize)> {
    let mut adet = adetler(sekme);
    let mut kesilen = 0;
    loop {
        if etiketli_toplam(ui, sekme, boyut, &adet) <= kullanilabilir {
            return Some((adet, kesilen));
        }
        let i = (0..adet.len()).filter(|i| adet[*i] > 0).max_by_key(|i| (adet[*i], *i))?;
        adet[i] -= 1;
        kesilen += 1;
    }
}

fn plan_yap(ui: &egui::Ui, sekme: &Sekme, kullanilabilir: f32, boyut: f32, adet: &[usize]) -> Vec<PanelPlani> {
    let taban: Vec<f32> = sekme.paneller.iter().map(|p| baslik_genisligi(ui, p)).collect();
    let genislikler = plan_genislikleri(ui, sekme, boyut, adet, &taban, kullanilabilir);
    sekme.paneller.iter().zip(genislikler).zip(adet).map(|((_, w), n)| PanelPlani { genislik: w, boyut, buyuk: *n }).collect()
}

/// The ribbon plan: every panel of the active tab stays on the row. The largest readable label
/// size is preferred; a few trailing large buttons may move into their panel menu to make room,
/// but when that would hide too many the next size down is tried. Labels are never drawn cut.
fn plan_uret(ui: &egui::Ui, sekme: &Sekme, kullanilabilir: f32) -> Vec<PanelPlani> {
    let mut yedek: Option<(usize, f32, Vec<usize>)> = None;
    for boyut in tema::SERIT_ETIKET_BOYUTLARI {
        let Some((adet, kesilen)) = keserek_sigdir(ui, sekme, boyut, kullanilabilir) else { continue };
        if kesilen <= EN_FAZLA_KESILEN {
            return plan_yap(ui, sekme, kullanilabilir, boyut, &adet);
        }
        if yedek.as_ref().is_none_or(|(k, _, _)| kesilen < *k) {
            yedek = Some((kesilen, boyut, adet));
        }
    }
    if let Some((_, boyut, adet)) = yedek {
        return plan_yap(ui, sekme, kullanilabilir, boyut, &adet);
    }
    // Even a single labelled row does not fit: draw every large button as an icon.
    let taban: Vec<f32> = sekme.paneller.iter().map(|p| baslik_genisligi(ui, p)).collect();
    let adet = adetler(sekme);
    let boyut = *tema::SERIT_ETIKET_BOYUTLARI.last().unwrap_or(&9.5);
    let alt: Vec<f32> = sekme.paneller.iter().zip(&adet).map(|(p, n)| asgari_genislik(ui, p, *n)).collect();
    let ust: Vec<f32> = sekme.paneller.iter().zip(&adet).map(|(p, n)| tam_genislik(ui, p, boyut, *n)).collect();
    let genislikler = dagit(&taban, &alt, &ust, kullanilabilir);
    sekme.paneller.iter().zip(genislikler).zip(&adet).map(|((_, w), n)| PanelPlani { genislik: w, boyut: 0.0, buyuk: *n }).collect()
}

/// The ribbon strip. Buttons run the same commands the menus do.
pub fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let Some(sekme) = SEKMELER.iter().find(|s| s.id == app.ui.ribbon_tab).or_else(|| SEKMELER.first()) else { return };
    let active = sekme.id;
    let mut actions: Vec<&'static str> = Vec::new();
    let mut picked_tab: Option<&'static str> = None;
    egui::Panel::top("cc_ribbon").exact_size(SEKME_Y + GOVDE_Y).frame(egui::Frame::NONE.fill(t.chrome)).show(ui, |ui| {
        let r = ui.max_rect();
        let p = ui.painter().clone();
        let tab_font = FontId::proportional(13.0);
        let mut x = r.left() + 6.0;
        for s in SEKMELER {
            let g = p.layout_no_wrap(crate::i18n::t(s.baslik).to_string(), tab_font.clone(), t.text);
            let w = g.size().x + 26.0;
            let tr = Rect::from_min_size(pos2(x, r.top()), vec2(w, SEKME_Y));
            let resp = ui.interact(tr, ui.id().with(("serit_sekme", s.id)), Sense::click());
            let on = s.id == active;
            if on {
                p.rect_filled(tr, 0.0, t.panel);
            } else if resp.hovered() {
                p.rect_filled(tr, 0.0, t.control_hover.gamma_multiply(0.55));
            }
            let color = if on { t.text } else { t.text_dim };
            p.galley(pos2(tr.center().x - g.size().x / 2.0, tr.center().y - g.size().y / 2.0), g, color);
            simgeler::describe_control(ui, &resp, s.baslik, Some(on), true);
            if resp.clicked() {
                picked_tab = Some(s.id);
            }
            x += w + 2.0;
        }
        // Quick access icons at the right of the tab strip.
        let hizli: [(Icon, &str, &str, Option<&str>); 6] = [
            (Icon::New, "new", "New Drawing", Some("Cmd+N")),
            (Icon::Open, "open", "Open", Some("Cmd+O")),
            (Icon::Save, "qsave", "Save", Some("Cmd+S")),
            (Icon::Undo, "u", "Undo", Some("Cmd+Z")),
            (Icon::Redo, "redo", "Redo", Some("Cmd+Shift+Z")),
            (Icon::Plot, "plot", "Print", None),
        ];
        let size = 20.0;
        let mac = ui.ctx().os().is_mac();
        let mut qx = r.right() - 8.0 - hizli.len() as f32 * (size + 4.0);
        if qx > x + 8.0 {
            for (icon, id, label, shortcut) in hizli {
                let br = Rect::from_min_size(pos2(qx, r.top() + (SEKME_Y - size) / 2.0), vec2(size, size));
                let resp = ui.interact(br, ui.id().with(("serit_hizli", id)), Sense::click());
                if resp.hovered() {
                    p.rect_filled(br, 3.0, t.control_hover);
                }
                simgeler::paint(&p, br.shrink(2.0), icon, !var(id));
                let tip = match shortcut {
                    Some(sc) => format!("{} ({})", crate::i18n::t(label), crate::menuler::shortcut_label(sc, mac)),
                    None => crate::i18n::t(label).to_string(),
                };
                simgeler::describe_control(ui, &resp, &tip, None, false);
                let resp = resp.on_hover_text(tip);
                if resp.clicked() {
                    actions.push(id);
                }
                qx += size + 4.0;
            }
        }
        // Panel body. Every panel of the active tab stays visible: the strip picks the largest
        // readable label size that fits all of them, trims trailing large buttons into their panel
        // menu when needed and hands the spare width to the panels with the most content hidden,
        // so the row always fills the window without a dead gap.
        let body = Rect::from_min_max(pos2(r.left(), r.top() + SEKME_Y), r.max);
        p.rect_filled(body, 0.0, t.panel);
        let plan = plan_uret(ui, sekme, body.width());
        let mut px = body.left();
        for (panel, plan) in sekme.paneller.iter().zip(&plan) {
            let pr = Rect::from_min_size(pos2(px, body.top()), vec2(plan.genislik, body.height()));
            ciz_panel(app, ui, panel, pr, plan, &mut actions);
            p.vline(pr.right() - 0.5, egui::Rangef::new(pr.top() + 2.0, pr.bottom() - 2.0), Stroke::new(1.0, t.border));
            px += plan.genislik;
        }
    });
    if let Some(tab) = picked_tab {
        app.ui.ribbon_tab = tab.to_string();
    }
    for id in actions {
        app.start(id);
    }
}

fn ciz_panel(app: &mut CadApp, ui: &mut egui::Ui, panel: &Panel, rect: Rect, plan: &PanelPlani, actions: &mut Vec<&'static str>) {
    let t = Tokens::get();
    let p = ui.painter().clone();
    let icerik = Rect::from_min_max(rect.min, pos2(rect.right(), rect.bottom() - TUR_H));
    let tur = Rect::from_min_max(pos2(rect.left(), rect.bottom() - TUR_H), rect.max);
    let seed = rect.left() as i32;
    let yazi = if plan.boyut > 0.0 { plan.boyut } else { *tema::SERIT_ETIKET_BOYUTLARI.last().unwrap_or(&9.5) };
    if panel.tur == Tur::Ozellikler {
        ciz_ozellikler(app, ui, rect, actions);
    } else {
        let buyuk_en = if plan.boyut > 0.0 {
            buyuk_kolon_genisligi(ui, panel, true, plan.boyut, plan.buyuk)
        } else {
            buyuk_kolon_genisligi(ui, panel, false, yazi, plan.buyuk)
        };
        if plan.buyuk > 0 && rect.width() >= IC_BOSLUK * 2.0 + buyuk_en {
            let sag = rect.right() - PANEL_SAG_BOSLUK;
            let mut x = rect.left() + IC_BOSLUK;
            for b in panel.buyuk.iter().take(plan.buyuk) {
                let bg = if plan.boyut > 0.0 { buyuk_etiket_genisligi(ui, b, plan.boyut) } else { BUYUK_IKON_G };
                let br = Rect::from_min_size(pos2(x, icerik.top() + 1.0), vec2(bg, icerik.height() - 2.0));
                if plan.boyut > 0.0 {
                    ciz_buyuk_etiketli(ui, &p, &t, b, br, plan.boyut, seed, actions);
                } else {
                    ciz_buyuk_ikon(ui, &p, &t, b, br, seed, actions);
                }
                x += bg;
            }
            if panel.tur == Tur::Katmanlar {
                let kalan = sag - x;
                if kalan >= KATMAN_LISTE_EN_AZ + IC_BOSLUK {
                    let w = KATMAN_LISTE_G.min(kalan - IC_BOSLUK);
                    katman_listesi(app, ui, Rect::from_min_size(pos2(x + 2.0, icerik.top() + 4.0), vec2(w - 8.0, icerik.height() - 8.0)));
                    x += w + IC_BOSLUK / 2.0;
                }
            }
            ciz_kucukler(ui, &p, &t, panel, icerik, sag, x, yazi, seed, actions);
        }
    }
    // Panel title with its options menu.
    let g = p.layout_no_wrap(crate::i18n::t(panel.baslik).to_string(), tema::body(), t.text_dim);
    let tw = (g.size().x + 18.0).min(rect.width());
    let tr = Rect::from_center_size(pos2(rect.center().x, tur.center().y), vec2(tw, TUR_H - 2.0));
    let resp = ui.interact(tr, ui.id().with(("serit_panel", panel.baslik, seed)), Sense::click());
    if resp.hovered() {
        p.rect_filled(tr, 3.0, t.control_hover.gamma_multiply(0.5));
    }
    p.galley(pos2(tr.left() + 4.0, tr.center().y - g.size().y / 2.0), g, if resp.hovered() { t.text } else { t.text_dim });
    simgeler::paint(&p, Rect::from_center_size(pos2(tr.right() - 7.0, tr.center().y), vec2(9.0, 9.0)), Icon::ChevronDown, false);
    simgeler::describe_control(ui, &resp, panel.baslik, None, false);
    egui::Popup::menu(&resp).show(|ui| {
        if panel.tur == Tur::Ozellikler {
            for (etiket, kimlik) in [("Properties", "properties"), ("Match Properties", "matchprop"), ("Quick Select", "qselect")] {
                if ui.button(crate::i18n::t(etiket)).clicked() {
                    actions.push(kimlik);
                    ui.close();
                }
            }
        } else {
            for b in panel.buyuk.iter().chain(panel.kucuk.iter()) {
                if ui.button(crate::i18n::t(b.label)).clicked() {
                    actions.push(b.id);
                    ui.close();
                }
            }
        }
    });
}

fn ciz_buyuk_etiketli(ui: &mut egui::Ui, p: &egui::Painter, t: &Tokens, b: &Dugme, br: Rect, boyut: f32, seed: i32, actions: &mut Vec<&'static str>) {
    let resp = ui.interact(br, ui.id().with(("serit_b", b.id, seed)), Sense::click_and_drag());
    if resp.hovered() {
        p.rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.75));
    }
    let en = var(b.id);
    simgeler::paint(p, Rect::from_center_size(pos2(br.center().x, br.top() + 21.0), vec2(28.0, 28.0)), b.icon, !en);
    let mut job = egui::text::LayoutJob::simple(
        crate::i18n::t(b.label).to_string(),
        tema::serit_etiket(boyut),
        if en { t.text_dim } else { t.text_faint },
        br.width() - BUYUK_METIN_BOSLUK,
    );
    job.wrap.max_rows = 2;
    job.halign = egui::Align::Center;
    let g = p.layout_job(job);
    p.galley(pos2(br.center().x - g.size().x / 2.0, br.bottom() - 4.0 - g.size().y), g, t.text_dim);
    simgeler::describe_control(ui, &resp, b.label, None, false);
    let resp = resp.on_hover_text(crate::i18n::t(b.label));
    resp.dnd_set_drag_payload(crate::paletler::raf::RafAraci(b.id.to_string()));
    if resp.clicked() && en {
        actions.push(b.id);
    }
}

fn ciz_buyuk_ikon(ui: &mut egui::Ui, p: &egui::Painter, t: &Tokens, b: &Dugme, br: Rect, seed: i32, actions: &mut Vec<&'static str>) {
    let resp = ui.interact(br, ui.id().with(("serit_b", b.id, seed)), Sense::click_and_drag());
    if resp.hovered() {
        p.rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.75));
    }
    let en = var(b.id);
    simgeler::paint(p, Rect::from_center_size(br.center(), vec2(22.0, 22.0)), b.icon, !en);
    simgeler::describe_control(ui, &resp, b.label, None, false);
    let resp = resp.on_hover_text(crate::i18n::t(b.label));
    resp.dnd_set_drag_payload(crate::paletler::raf::RafAraci(b.id.to_string()));
    if resp.clicked() && en {
        actions.push(b.id);
    }
}

/// Draws the small buttons that still fit beside the large ones: labelled rows when two or more
/// columns fit, otherwise compact icon-only columns whose labels become tooltips. Everything else
/// stays reachable from the panel title menu.
fn ciz_kucukler(
    ui: &mut egui::Ui,
    p: &egui::Painter,
    t: &Tokens,
    panel: &Panel,
    icerik: Rect,
    sag: f32,
    x: f32,
    yazi: f32,
    seed: i32,
    actions: &mut Vec<&'static str>,
) {
    let x = x + 2.0;
    let kalan = sag - x;
    if kalan <= 0.0 || panel.kucuk.is_empty() {
        return;
    }
    let kolonlar = etiketli_kolonlar(ui, panel, yazi);
    let mut akis = 0.0;
    let mut etiketli = 0usize;
    for w in &kolonlar {
        if akis + w <= kalan + 0.01 {
            akis += w;
            etiketli += 1;
        } else {
            break;
        }
    }
    if etiketli >= 2 {
        let mut cx = x;
        for (i, b) in panel.kucuk.iter().enumerate() {
            let kol = i / 3;
            let satir = i % 3;
            if kol >= etiketli {
                break;
            }
            let cw = kolonlar[kol];
            let br = Rect::from_min_size(pos2(cx, icerik.top() + 2.0 + satir as f32 * SATIR_H), vec2(cw - 2.0, SATIR_H - 1.0));
            let en = var(b.id);
            let resp = ui.interact(br, ui.id().with(("serit_s", b.id, i, seed)), Sense::click_and_drag());
            if resp.hovered() {
                p.rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.75));
            }
            simgeler::paint(p, Rect::from_min_size(pos2(br.left() + 2.0, br.center().y - 8.0), vec2(16.0, 16.0)), b.icon, !en);
            let label = crate::i18n::t(b.label);
            let g = kes(p, label, FontId::proportional(yazi), if en { t.text_dim } else { t.text_faint }, br.width() - 21.0);
            p.with_clip_rect(br).galley(pos2(br.left() + 21.0, br.center().y - g.size().y / 2.0), g, t.text_dim);
            simgeler::describe_control(ui, &resp, b.label, None, false);
            resp.dnd_set_drag_payload(crate::paletler::raf::RafAraci(b.id.to_string()));
            if resp.clicked() && en {
                actions.push(b.id);
            }
            if satir == 2 {
                cx += cw;
            }
        }
        return;
    }
    let ikon_adet = (((kalan + 0.01) / KUCUK_IKON_G).floor() as usize).min(panel.kucuk.len().div_ceil(3));
    for (i, b) in panel.kucuk.iter().enumerate() {
        let kol = i / 3;
        let satir = i % 3;
        if kol >= ikon_adet {
            break;
        }
        let br = Rect::from_min_size(
            pos2(x + kol as f32 * KUCUK_IKON_G, icerik.top() + 2.0 + satir as f32 * SATIR_H),
            vec2(KUCUK_IKON_G - 2.0, SATIR_H - 1.0),
        );
        let en = var(b.id);
        let resp = ui.interact(br, ui.id().with(("serit_s", b.id, i, seed)), Sense::click_and_drag());
        if resp.hovered() {
            p.rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.75));
        }
        simgeler::paint(p, Rect::from_center_size(br.center(), vec2(16.0, 16.0)), b.icon, !en);
        simgeler::describe_control(ui, &resp, b.label, None, false);
        let resp = resp.on_hover_text(crate::i18n::t(b.label));
        resp.dnd_set_drag_payload(crate::paletler::raf::RafAraci(b.id.to_string()));
        if resp.clicked() && en {
            actions.push(b.id);
        }
    }
}

/// The current-layer dropdown of the Layers panel, as on the reference ribbon.
fn katman_listesi(app: &mut CadApp, ui: &mut egui::Ui, rect: Rect) {
    let Ok(d) = app.session.doc() else { return };
    let clay = d.header.str("CLAYER", "0").to_string();
    let layers: Vec<String> = d.layers.iter().map(|l| l.name.clone()).collect();
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::top_down(egui::Align::Min)));
    let mut yeni = None;
    egui::ComboBox::from_id_salt("serit_katman_listesi").width(rect.width() - 4.0).selected_text(clay.clone()).show_ui(&mut child, |ui| {
        for name in &layers {
            if ui.selectable_label(name.eq_ignore_ascii_case(&clay), name).clicked() {
                yeni = Some(name.clone());
            }
        }
    });
    if let Some(name) = yeni {
        let _ = app.run("layer.current", json!({ "name": name }));
    }
}

/// The lineweight choices the Properties dropdown offers: the logical values then the standard
/// millimetre steps.
fn agirlik_secenekleri() -> Vec<(i64, String)> {
    let mut kodlar: Vec<i64> = vec![-1, -2, -3];
    kodlar.extend(spcad_doc::Lineweight::STANDARD.iter().map(|v| i64::from(*v)));
    kodlar.into_iter().map(|k| (k, spcad_doc::Lineweight::from_dxf(k as i16).name())).collect()
}

/// The Properties panel: the current colour, linetype and lineweight in a 2×2 grid with the
/// property tools, as on the reference ribbon.
fn ciz_ozellikler(app: &mut CadApp, ui: &mut egui::Ui, rect: Rect, actions: &mut Vec<&'static str>) {
    let t = Tokens::get();
    let Ok(d) = app.session.doc() else {
        let p = ui.painter();
        p.text(rect.center(), Align2::CENTER_CENTER, crate::tl!("No drawing"), tema::small(), t.text_faint);
        return;
    };
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink2(vec2(6.0, 4.0))).layout(egui::Layout::top_down(egui::Align::Min)));
    child.spacing_mut().item_spacing = vec2(6.0, 3.0);

    let cecolor = d.header.i64("CECOLOR", 256) as i16;
    let cname = cname_of(cecolor);
    let celtype = d.header.str("CELTYPE", "ByLayer").to_string();
    let celweight = d.header.i64("CELWEIGHT", -1);
    let colors: Vec<(i16, String)> = [256i16, 0, 1, 2, 3, 4, 5, 6, 7].into_iter().map(|n| (n, cname_of(n))).collect();
    let linetypes: Vec<String> = d.linetypes.iter().map(|l| l.name.clone()).collect();

    let mut yeni_renk = cecolor;
    let mut yeni_tip = None;
    let mut yeni_agirlik: Option<i64> = None;
    // A narrow strip keeps the current-colour control only; the 2×2 grid needs the full width.
    if rect.width() < OZELLIK_G - 20.0 {
        child.horizontal(|ui| {
            let c = if (0..=255).contains(&cecolor) {
                spcad_color::aci_rgb(cecolor as u8)
            } else {
                spcad_color::Rgb(t.text_faint.r(), t.text_faint.g(), t.text_faint.b())
            };
            let (sw, _) = ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
            ui.painter().rect_filled(sw, 2.0, egui::Color32::from_rgb(c.0, c.1, c.2));
            let w = (ui.available_width() - 8.0).clamp(76.0, 112.0);
            egui::ComboBox::from_id_salt("serit_renk").width(w).selected_text(cname.clone()).show_ui(ui, |ui| {
                for (n, name) in &colors {
                    if ui.selectable_label(cecolor == *n, name).clicked() {
                        yeni_renk = *n;
                    }
                }
            });
        });
    } else {
        child.horizontal(|ui| {
            let c = if (0..=255).contains(&cecolor) {
                spcad_color::aci_rgb(cecolor as u8)
            } else {
                spcad_color::Rgb(t.text_faint.r(), t.text_faint.g(), t.text_faint.b())
            };
            let (sw, _) = ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
            ui.painter().rect_filled(sw, 2.0, egui::Color32::from_rgb(c.0, c.1, c.2));
            egui::ComboBox::from_id_salt("serit_renk").width(112.0).selected_text(cname.clone()).show_ui(ui, |ui| {
                for (n, name) in &colors {
                    if ui.selectable_label(cecolor == *n, name).clicked() {
                        yeni_renk = *n;
                    }
                }
            });
            ui.label(crate::tl!("Linetype"));
            egui::ComboBox::from_id_salt("serit_cizgi_tipi").width(96.0).selected_text(celtype.clone()).show_ui(ui, |ui| {
                for name in &linetypes {
                    if ui.selectable_label(name.eq_ignore_ascii_case(&celtype), name).clicked() {
                        yeni_tip = Some(name.clone());
                    }
                }
            });
        });
        child.horizontal(|ui| {
            ui.label(crate::tl!("Lineweight"));
            let secili = agirlik_secenekleri().into_iter().find(|(k, _)| *k == celweight).map(|(_, ad)| ad).unwrap_or_else(|| "ByLayer".to_string());
            egui::ComboBox::from_id_salt("serit_agirlik").width(96.0).selected_text(secili).show_ui(ui, |ui| {
                for (k, ad) in agirlik_secenekleri() {
                    if ui.selectable_label(k == celweight, ad).clicked() {
                        yeni_agirlik = Some(k);
                    }
                }
            });
            if simgeler::button(ui, Icon::Properties, 22.0, "Properties", false).clicked() {
                actions.push("properties");
            }
            if simgeler::button(ui, Icon::MatchProp, 22.0, "Match Properties", false).clicked() {
                actions.push("matchprop");
            }
            if simgeler::button(ui, Icon::QuickSelect, 22.0, "Quick Select", false).clicked() {
                actions.push("qselect");
            }
        });
    }
    if yeni_renk != cecolor {
        let _ = app.run("color", json!({ "color": yeni_renk }));
    }
    if let Some(name) = yeni_tip {
        let _ = app.run("linetype", json!({ "current": name }));
    }
    if let Some(k) = yeni_agirlik {
        let v = if k >= 0 {
            json!(k as f64 / 100.0)
        } else {
            json!(match k {
                -2 => "byblock",
                -3 => "default",
                _ => "bylayer",
            })
        };
        let _ = app.run("lweight", json!({ "lineweight": v }));
    }
}

fn cname_of(n: i16) -> String {
    match n {
        256 => "ByLayer".to_string(),
        0 => "ByBlock".to_string(),
        n => spcad_color::Color::from_aci(n).name().to_string(),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn every_ribbon_command_exists() {
        for sekme in SEKMELER {
            for panel in sekme.paneller {
                for b in panel.buyuk.iter().chain(panel.kucuk.iter()) {
                    assert!(var(b.id), "{}: {} ({}) is not a command", sekme.id, b.id, b.label);
                }
            }
        }
    }

    #[test]
    fn tab_ids_are_unique_and_home_is_first() {
        let mut ids: Vec<&str> = SEKMELER.iter().map(|s| s.id).collect();
        assert_eq!(ids.first().copied(), Some("home"));
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SEKMELER.len());
    }

    #[test]
    fn panels_have_a_bounded_height() {
        for sekme in SEKMELER {
            for panel in sekme.paneller {
                assert!(panel.kucuk.len() <= 18, "{} {} has too many small buttons", sekme.baslik, panel.baslik);
                assert!(panel.buyuk.len() <= 6, "{} {} has too many large buttons", sekme.baslik, panel.baslik);
            }
        }
    }

    #[test]
    fn layers_panel_carries_the_dropdown_like_the_reference() {
        let home = &SEKMELER[0];
        let katmanlar = home.paneller.iter().find(|p| p.tur == Tur::Katmanlar);
        assert!(katmanlar.is_some(), "the home tab has no Layers dropdown panel");
        assert!(home.paneller.iter().any(|p| p.tur == Tur::Ozellikler), "the home tab has no Properties panel");
        if let Some(p) = katmanlar {
            assert_eq!(p.buyuk.len(), 1, "the Layers panel keeps a single large button");
        }
    }

    fn galley_satirlari(shape: &egui::Shape, out: &mut Vec<(String, usize, bool)>) {
        match shape {
            egui::Shape::Text(t) => out.push((t.galley.text().to_string(), t.galley.rows.len(), t.galley.elided)),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| galley_satirlari(shape, out)),
            _ => {}
        }
    }

    fn metinler(shape: &egui::Shape, clip: Rect, out: &mut Vec<(String, Rect, Rect)>) {
        match shape {
            egui::Shape::Text(t) => out.push((t.galley.text().to_string(), t.visual_bounding_rect(), clip)),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| metinler(shape, clip, out)),
            _ => {}
        }
    }

    #[test]
    fn katmanlar_ve_ozellikler_referans_duzeniyle_cizilir() {
        let app = CadApp::new(spcad_engine::Session::default(), crate::Services::default());
        let home = &SEKMELER[0];
        let genislik = 6000.0;
        let shot_dir = std::env::var_os("CRAFT_UI_SUITE_SHOTS").map(std::path::PathBuf::from);
        let mut builder = egui_kittest::Harness::builder().with_size(vec2(genislik, SEKME_Y + GOVDE_Y));
        if shot_dir.is_some() {
            builder = builder.wgpu();
        }
        let mut h = builder.build_ui_state(
            |ui, app: &mut CadApp| {
                app.logic(&ui.ctx().clone());
                show(app, ui);
            },
            app,
        );
        h.run_steps(4);

        let mut painted = Vec::new();
        for shape in &h.output().shapes {
            metinler(&shape.shape, shape.clip_rect, &mut painted);
        }
        let gorunur = |aranan: &str| painted.iter().any(|(t, _, _)| t == aranan);
        for panel in home.paneller {
            let baslik = crate::i18n::t(panel.baslik).to_string();
            assert!(
                gorunur(&baslik),
                "{} paneli tam genişlikte görünür değil; boyananlar: {:?}",
                panel.baslik,
                painted.iter().map(|(t, _, _)| t.clone()).take(30).collect::<Vec<_>>()
            );
        }
        for etiket in ["Linetype", "Lineweight"] {
            let ceviri = crate::i18n::t(etiket).to_string();
            assert!(gorunur(&ceviri), "{etiket} etiketi Özellikler ızgarasında görünür değil");
        }
        assert!(gorunur("0"), "katman açılır listesi Katmanlar panelinde görünür değil");
        if let Some(dizin) = shot_dir {
            std::fs::create_dir_all(&dizin).unwrap();
            h.render().unwrap().save(dizin.join("spcad-serit-katmanlar-ozellikler.png")).unwrap();
        }
    }

    /// Paints the ribbon at the given width with the given tab active and returns its texts.
    fn serit_boya(genislik: f32, sekme: &'static str) -> Vec<(String, Rect, Rect)> {
        serit_boya_dil(genislik, sekme, 1.0, "en")
    }

    fn serit_boya_olcek(genislik: f32, sekme: &'static str, ppp: f32) -> Vec<(String, Rect, Rect)> {
        serit_boya_dil(genislik, sekme, ppp, "en")
    }

    fn serit_boya_dil(genislik: f32, sekme: &'static str, ppp: f32, dil: &'static str) -> Vec<(String, Rect, Rect)> {
        let (painted, _) = serit_harness(genislik, sekme, ppp, dil);
        painted
    }

    /// The painted texts, the row count and the elision flag of every painted galley. A label that
    /// needs more than two rows or gets elided is clipped mid-text and looks broken.
    fn serit_harness(genislik: f32, sekme: &'static str, ppp: f32, dil: &'static str) -> (Vec<(String, Rect, Rect)>, Vec<(String, usize, bool)>) {
        // kittest wraps the closure in a CentralPanel with an 8 px outer margin; the extra 16 px
        // keep the usable strip exactly genislik wide and tall, matching the real window.
        let app = CadApp::new(spcad_engine::Session::default(), crate::Services::default());
        let mut builder = egui_kittest::Harness::builder().with_pixels_per_point(ppp).with_size(vec2(genislik + 16.0, SEKME_Y + GOVDE_Y + 16.0));
        let shot_dir = std::env::var_os("CRAFT_UI_SUITE_SHOTS").map(std::path::PathBuf::from);
        if shot_dir.is_some() {
            builder = builder.wgpu();
        }
        let mut h = builder.build_ui_state(
            move |ui, app: &mut CadApp| {
                crate::i18n::with_language(dil, || {
                    app.logic(&ui.ctx().clone());
                    app.ui.ribbon_tab = sekme.to_string();
                    show(app, ui);
                });
            },
            app,
        );
        h.run_steps(3);
        let mut painted = Vec::new();
        let mut satirlar = Vec::new();
        for shape in &h.output().shapes {
            metinler(&shape.shape, shape.clip_rect, &mut painted);
            galley_satirlari(&shape.shape, &mut satirlar);
        }
        if let Some(dizin) = shot_dir {
            std::fs::create_dir_all(&dizin).unwrap();
            let ad = format!(
                "spcad-serit-{}-{genislik:.0}{}{}.png",
                sekme,
                if (ppp - 1.0).abs() < f32::EPSILON { String::new() } else { format!("-ppp{ppp}") },
                if dil == "en" { String::new() } else { format!("-{dil}") }
            );
            if let Ok(goruntu) = h.render() {
                let _ = goruntu.save(dizin.join(ad));
            }
        }
        (painted, satirlar)
    }

    fn basliklar(painted: &[(String, Rect, Rect)]) -> Vec<String> {
        painted.iter().map(|(t, _, _)| t.clone()).collect()
    }

    #[test]
    fn her_genislikte_home_panelleri_gorunur() {
        for genislik in [900.0, 1280.0, 1366.0, 1415.0, 1440.0, 1536.0, 1674.0, 1920.0] {
            let painted = serit_boya(genislik, "home");
            for panel in SEKMELER[0].paneller {
                let baslik = crate::i18n::t(panel.baslik).to_string();
                assert!(basliklar(&painted).contains(&baslik), "{genislik} px: {} paneli görünmüyor", panel.baslik);
            }
        }
    }

    #[test]
    fn her_sekme_1280_pikselde_panellerini_gosterir() {
        for sekme in SEKMELER {
            let painted = serit_boya(1280.0, sekme.id);
            for panel in sekme.paneller {
                let baslik = crate::i18n::t(panel.baslik).to_string();
                assert!(basliklar(&painted).contains(&baslik), "{} @1280: {} paneli görünmüyor", sekme.id, panel.baslik);
            }
        }
    }

    #[test]
    fn yuksek_dpi_de_ayni_yerlesim() {
        for ppp in [1.25, 1.5] {
            let painted = serit_boya_olcek(1280.0, "home", ppp);
            for panel in SEKMELER[0].paneller {
                let baslik = crate::i18n::t(panel.baslik).to_string();
                assert!(basliklar(&painted).contains(&baslik), "ppp {ppp}: {} paneli görünmüyor", panel.baslik);
            }
        }
    }

    #[test]
    fn dillerde_serit_etiketleri_kirpilmaz() {
        for dil in crate::i18n::LANGUAGES.iter().map(|l| l.code) {
            let buyuk: Vec<String> =
                SEKMELER[0].paneller.iter().flat_map(|p| p.buyuk.iter()).map(|b| crate::i18n::tr(dil, b.label).to_string()).collect();
            for genislik in [1280.0, 1415.0, 1674.0] {
                let (painted, satirlar) = serit_harness(genislik, "home", 1.0, dil);
                assert!(!painted.is_empty(), "{dil} @{genislik}: hiç metin boyanmadı");
                for (metin, satir, kirpik) in &satirlar {
                    if !buyuk.iter().any(|b| b == metin) {
                        continue;
                    }
                    assert!(*satir <= 2, "{dil} @{genislik}: {metin:?} {satir} satıra sarıldı");
                    assert!(!*kirpik, "{dil} @{genislik}: {metin:?} etiketinin bir kısmı kırpıldı");
                }
            }
        }
    }

    #[test]
    fn dar_seritte_buyuk_dugme_etiketleri_gorunur() {
        for dil in ["en", "tr"] {
            let painted = serit_boya_dil(1280.0, "home", 1.0, dil);
            let toplam: usize = SEKMELER[0].paneller.iter().map(|p| p.buyuk.len()).sum();
            let (gorunen, etiketler): (Vec<Rect>, Vec<(String, Rect)>) = {
                let metinler = basliklar(&painted);
                let mut gorunen = Vec::new();
                let mut etiketler = Vec::new();
                for panel in SEKMELER[0].paneller {
                    for b in panel.buyuk {
                        let etiket = crate::i18n::tr(dil, b.label).to_string();
                        if metinler.contains(&etiket)
                            && let Some((_, rect, _)) = painted.iter().find(|(t, _, _)| *t == etiket)
                        {
                            gorunen.push(*rect);
                            etiketler.push((etiket, *rect));
                        }
                    }
                }
                (gorunen, etiketler)
            };
            assert!(
                gorunen.len() + EN_FAZLA_KESILEN >= toplam,
                "1280 px ({dil}): büyük düğme etiketlerinin yalnızca {}/{} tanesi görünüyor",
                gorunen.len(),
                toplam
            );
            for (i, (ad, a)) in etiketler.iter().enumerate() {
                for (badi, b) in etiketler.iter().skip(i + 1) {
                    assert!(!a.shrink(1.0).intersects(b.shrink(1.0)), "1280 px ({dil}): {ad:?} ile {badi:?} etiketleri birbirine değiyor");
                }
            }
        }
    }

    #[test]
    fn serit_sag_kenari_doldurur_ve_asmaz() {
        for genislik in [900.0, 1280.0, 1440.0, 1674.0, 1920.0] {
            let painted = serit_boya(genislik, "home");
            let en_sag = painted.iter().map(|(_, r, _)| r.max.x).fold(f32::MIN, f32::max);
            // kittest shifts the content by its 8 px central-panel margin.
            assert!(en_sag <= genislik + 10.0, "{genislik} px: boyanan metin sağ sınırı aşıyor ({en_sag:.0})");
            assert!(en_sag >= genislik - 140.0, "{genislik} px: sağda ölü boşluk var (son metin {en_sag:.0})");
        }
    }
}
