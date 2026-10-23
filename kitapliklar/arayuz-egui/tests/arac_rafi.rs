//! The tool shelf: opened on either side, closed and reopened, and filled by dragging command
//! buttons from the ribbon or a menu. The tools live in the interface state.

use egui::{Event, Modifiers, PointerButton, Pos2, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::json;
use spcad_engine::Session;
use spcad_ui_egui::{CadApp, Services};

fn harness() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.in_window_menu = false;
    app.integrated_titlebar = true;
    let mut h = Harness::builder().with_size(vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h
}

fn drag(h: &mut Harness<'_, CadApp>, from: Pos2, to: Pos2) {
    h.event(Event::PointerMoved(from));
    h.run_steps(2);
    h.event(Event::PointerButton { pos: from, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE });
    h.run_steps(2);
    for step in 1..=8 {
        h.event(Event::PointerMoved(from.lerp(to, step as f32 / 8.0)));
        h.run_steps(1);
    }
    h.event(Event::PointerButton { pos: to, button: PointerButton::Primary, pressed: false, modifiers: Modifiers::NONE });
    h.run_steps(4);
}

#[test]
fn the_shelf_is_closed_by_default_and_opens_on_the_side_asked() {
    let mut h = harness();
    assert!(!h.state().ui.show_shelf, "no side palette is open by default");
    assert!(h.query_by_role_and_label(egui::accesskit::Role::Tab, "Tool Shelf").is_none());
    h.state_mut().run("ui.shelf.left", json!({})).unwrap();
    h.run_steps(4);
    assert!(h.state().ui.show_shelf);
    assert_eq!(h.state().ui.shelf_side, "left");
    assert!(h.query_by_role_and_label(egui::accesskit::Role::Tab, "Tool Shelf").is_some());
    h.state_mut().run("ui.shelf.right", json!({})).unwrap();
    h.run_steps(4);
    assert!(h.state().ui.show_shelf, "moving it keeps it open");
    assert_eq!(h.state().ui.shelf_side, "right");
    // Closing keeps the tools and the side; reopening restores them.
    h.state_mut().run("ui.shelf.add", json!({"id": "line"})).unwrap();
    h.state_mut().run("ui.toggle.shelf", json!({"on": false})).unwrap();
    h.run_steps(4);
    assert!(!h.state().ui.show_shelf);
    assert_eq!(h.state().ui.shelf_tools, ["line"]);
    h.state_mut().run("ui.toggle.shelf", json!({})).unwrap();
    h.run_steps(4);
    assert!(h.state().ui.show_shelf);
    assert_eq!(h.state().ui.shelf_side, "right");
}

fn shelf_split_size(node: &serde_json::Value) -> Option<serde_json::Value> {
    fn has_shelf(node: &serde_json::Value) -> bool {
        match node {
            serde_json::Value::Array(items) => items.iter().any(has_shelf),
            serde_json::Value::Object(map) => {
                map.get("panels").and_then(|p| p.as_array()).is_some_and(|p| p.iter().any(|v| v.as_str() == Some("shelf")))
                    || map.get("panel").and_then(|v| v.as_str()) == Some("shelf")
                    || map.values().any(has_shelf)
            }
            _ => false,
        }
    }
    if let Some(split) = node.get("Split") {
        let first = split.get("first").is_some_and(has_shelf);
        let second = split.get("second").is_some_and(has_shelf);
        if first || second {
            // The nearest split around the shelf is the one that sizes it.
            let branch = if first { split.get("first") } else { split.get("second") };
            if let Some(inner) = branch.and_then(shelf_split_size) {
                return Some(inner);
            }
            return split.get("size").cloned();
        }
    }
    match node {
        serde_json::Value::Array(items) => items.iter().find_map(shelf_split_size),
        serde_json::Value::Object(map) => map.values().find_map(shelf_split_size),
        _ => None,
    }
}

#[test]
fn the_shelf_keeps_its_width_on_both_sides() {
    let mut h = harness();
    for (command, key) in [("ui.shelf.left", "FixedFirst"), ("ui.shelf.right", "FixedSecond")] {
        h.state_mut().run(command, json!({})).unwrap();
        h.run_steps(3);
        let docking = serde_json::to_value(&h.state().ui.docking).unwrap();
        let size = shelf_split_size(&docking).unwrap_or_default();
        assert_eq!(size[key], json!(220.0), "{command} sizes the shelf, not the drawing: {size}");
    }
}

#[test]
fn shelf_tools_are_added_once_removed_and_cleared() {
    let mut h = harness();
    h.state_mut().run("ui.shelf.add", json!({"id": "line"})).unwrap();
    h.state_mut().run("ui.shelf.add", json!({"id": "circle"})).unwrap();
    h.state_mut().run("ui.shelf.add", json!({"id": "circle"})).unwrap();
    assert_eq!(h.state().ui.shelf_tools, ["line", "circle"], "a command sits on the shelf once");
    assert!(h.state_mut().run("ui.shelf.add", json!({"id": "no.such.command"})).is_err(), "unknown commands are refused");
    h.state_mut().run("ui.shelf.remove", json!({"id": "line"})).unwrap();
    assert_eq!(h.state().ui.shelf_tools, ["circle"]);
    h.state_mut().run("ui.shelf.clear", json!({})).unwrap();
    assert!(h.state().ui.shelf_tools.is_empty());
    // The shelf is part of the saved interface state.
    h.state_mut().run("ui.shelf.add", json!({"id": "line"})).unwrap();
    let saved = serde_json::to_value(&h.state().ui).unwrap();
    assert_eq!(saved["shelfTools"], json!(["line"]));
}

#[test]
fn dragging_a_ribbon_button_onto_the_shelf_adds_it() {
    let mut h = harness();
    h.state_mut().run("ui.shelf.left", json!({})).unwrap();
    h.run_steps(4);
    assert!(h.state().ui.shelf_tools.is_empty());
    let from = h.get_by_label("Line").rect().center();
    let to = h.get_by_label("Tool Shelf drop area").rect().center();
    drag(&mut h, from, to);
    assert_eq!(h.state().ui.shelf_tools, ["line"], "the dragged command lands on the shelf");
    // Clicking the shelf row starts the command (the ribbon holds another "Line").
    {
        let rows: Vec<_> = h.query_all_by_label("Line").collect();
        if let Some(row) = rows.iter().max_by(|a, b| a.rect().center().y.total_cmp(&b.rect().center().y)) {
            row.click();
        }
    }
    h.run_steps(3);
    assert_eq!(h.state().session.running.as_ref().map(|command| command.id.as_str()), Some("line"));
}

#[test]
fn every_shelf_entry_resolves_to_an_icon_and_a_label() {
    for id in ["line", "circle", "mleaderedit", "ui.dialog.layers", "no.such.command"] {
        let (icon, label) = spcad_ui_egui::paletler::raf::gorunum(id);
        assert!(!label.is_empty(), "{id} has a label");
        assert!(spcad_ui_egui::paletler::raf::gecerli(id) == (id != "no.such.command"), "{id}");
        let _ = icon;
    }
}
