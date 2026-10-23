//! The user's tool shelf: a docking panel the user fills by dragging command buttons from the
//! ribbon or a menu. It opens on either side, closes and reopens, and its width is the docking
//! splitter's. The tools and the side are saved with the interface state.

use egui::{Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::CadApp;
use crate::simgeler::{self, Icon};
use crate::tema::Tokens;

/// A command dragged from the ribbon or a menu. The shelf accepts it on release.
#[derive(Clone, Debug)]
pub struct RafAraci(pub String);

/// Whether the shelf can hold `id`: an engine command or a UI command.
pub fn gecerli(id: &str) -> bool {
    spcad_engine::find_command(id).is_some() || crate::menuler::is_ui_command(id)
}

/// The icon and the translated label shown for `id`.
pub fn gorunum(id: &str) -> (Icon, String) {
    for sekme in crate::serit::SEKMELER {
        for panel in sekme.paneller {
            for b in panel.buyuk.iter().chain(panel.kucuk.iter()) {
                if b.id == id {
                    return (b.icon, crate::i18n::t(b.label).to_string());
                }
            }
        }
    }
    if let Some(c) = spcad_engine::find_command(id) {
        return (Icon::Gear, crate::i18n::t(c.label).to_string());
    }
    if let Some((_, label, _, _)) = crate::menuler::UI_COMMANDS.iter().find(|(command, ..)| *command == id) {
        return (Icon::Gear, crate::i18n::t(label).to_string());
    }
    (Icon::Gear, id.to_string())
}

/// `text` on one line no wider than `max_width`, ending in "…" when cut.
fn kis(ui: &egui::Ui, text: &str, color: egui::Color32, max_width: f32) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), crate::tema::body(), color);
    job.wrap = egui::text::TextWrapping { max_width: max_width.max(0.0), max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    ui.painter().layout_job(job)
}

const SATIR: f32 = 28.0;
const BASLIK: f32 = 28.0;

pub fn shelf(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let mut add: Option<String> = None;
    let mut remove: Option<String> = None;
    let mut clear = false;
    let mut side: Option<&str> = None;
    let mut start: Option<String> = None;
    let mut close = false;
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), ui.available_height()), Sense::hover());
    let p = ui.painter().clone();
    p.rect_filled(r, 0.0, t.panel);
    // Header: title, then the side switch, clear and close buttons.
    let head = Rect::from_min_size(r.min, vec2(r.width(), BASLIK));
    p.hline(head.x_range(), head.bottom(), Stroke::new(1.0, t.border));
    p.text(
        pos2(head.left() + 10.0, head.center().y),
        egui::Align2::LEFT_CENTER,
        crate::tf!("{n} tools", n = app.ui.shelf_tools.len()),
        egui::FontId::proportional(12.5),
        t.text_dim,
    );
    let mut bx = head.right() - 12.0;
    for (icon, tip, action) in [
        (Icon::Close, "Close Tool Shelf", 2u8),
        (Icon::Erase, "Clear Tool Shelf", 1),
        (if app.ui.shelf_side == "right" { Icon::ChevronLeft } else { Icon::ChevronRight }, "Move Tool Shelf to the Other Side", 0),
    ] {
        let br = Rect::from_center_size(pos2(bx, head.center().y), vec2(18.0, 18.0));
        let resp = ui.interact(br, ui.id().with(("raf_baslik", action)), Sense::click());
        if resp.hovered() {
            p.rect_filled(br, 3.0, t.control_hover);
        }
        simgeler::paint(&p, br.shrink(3.0), icon, action == 1 && app.ui.shelf_tools.is_empty());
        simgeler::describe_control(ui, &resp, tip, None, false);
        if resp.on_hover_text(crate::i18n::t(tip)).clicked() {
            match action {
                2 => close = true,
                1 => clear = true,
                _ => side = Some(if app.ui.shelf_side == "right" { "left" } else { "right" }),
            }
        }
        bx -= 22.0;
    }
    // Body: the drop zone and the tool rows.
    let body = Rect::from_min_max(pos2(r.left() + 4.0, head.bottom() + 4.0), pos2(r.right() - 4.0, r.bottom() - 4.0));
    let drop = ui.interact(body, ui.id().with("raf_govde"), Sense::hover());
    simgeler::describe_control(ui, &drop, "Tool Shelf drop area", None, false);
    let hovering = drop.dnd_hover_payload::<RafAraci>().is_some();
    if hovering {
        p.rect_filled(body, 3.0, t.accent.gamma_multiply(0.12));
        p.rect_stroke(body, 3.0, Stroke::new(1.5, t.accent), egui::StrokeKind::Inside);
    }
    if let Some(payload) = drop.dnd_release_payload::<RafAraci>()
        && gecerli(&payload.0)
    {
        add = Some(payload.0.clone());
    }
    if app.ui.shelf_tools.is_empty() {
        p.text(
            body.center(),
            egui::Align2::CENTER_CENTER,
            crate::tl!("Drag a tool here from the ribbon or a menu."),
            crate::tema::small(),
            t.text_faint,
        );
        p.text(
            pos2(body.center().x, body.center().y + 18.0),
            egui::Align2::CENTER_CENTER,
            crate::tl!("You can close it and reopen it on either side."),
            crate::tema::small(),
            t.text_faint,
        );
    }
    let tools = app.ui.shelf_tools.clone();
    for (i, id) in tools.iter().enumerate() {
        let y = body.top() + i as f32 * SATIR;
        if y + SATIR > body.bottom() {
            break;
        }
        let row = Rect::from_min_size(pos2(body.left(), y), vec2(body.width(), SATIR - 2.0));
        let resp = ui.interact(row, ui.id().with(("raf_satir", i, &id)), Sense::click_and_drag());
        if resp.hovered() {
            p.rect_filled(row, 3.0, t.control_hover.gamma_multiply(0.7));
        }
        let (icon, label) = gorunum(id);
        simgeler::paint(&p, Rect::from_min_size(pos2(row.left() + 5.0, row.center().y - 9.0), vec2(18.0, 18.0)), icon, false);
        let g = kis(ui, &label, t.text, row.width() - 54.0);
        p.galley(pos2(row.left() + 28.0, row.center().y - g.size().y / 2.0), g, t.text);
        simgeler::describe_control(ui, &resp, &label, None, false);
        resp.dnd_set_drag_payload(RafAraci(id.clone()));
        if resp.clicked() {
            start = Some(id.clone());
        }
        // A remove button on the hovered row.
        if resp.hovered() {
            let xr = Rect::from_center_size(pos2(row.right() - 12.0, row.center().y), vec2(16.0, 16.0));
            let xresp = ui.interact(xr, ui.id().with(("raf_sil", i)), Sense::click());
            simgeler::paint(&p, xr.shrink(3.0), Icon::Close, false);
            simgeler::describe_control(ui, &xresp, "Remove from Tool Shelf", None, false);
            if xresp.on_hover_text(crate::tl!("Remove from Tool Shelf")).clicked() {
                remove = Some(id.clone());
            }
        }
    }
    // The side, close and content changes all go through commands, so scripts and the control
    // channel reach them too.
    if let Some(id) = add {
        let _ = app.run("ui.shelf.add", json!({ "id": id }));
    }
    if let Some(id) = remove {
        let _ = app.run("ui.shelf.remove", json!({ "id": id }));
    }
    if clear {
        let _ = app.run("ui.shelf.clear", json!({}));
    }
    if let Some(side) = side {
        let _ = app.run("ui.shelf.side", json!({ "side": side }));
    }
    if close {
        let _ = app.run("ui.toggle.shelf", json!({ "on": false }));
    }
    if let Some(id) = start {
        app.cmd.buffer.clear();
        app.start(&id);
    }
}
