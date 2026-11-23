//! Native Smart Controller UI foundation.
//!
//! This first UI milestone provides Stage handles as transient editor state only. It does not
//! create document bindings, evaluate controllers, mutate targets, or persist data.

use crate::app::{App, Drag, Tool};
use anicraft_engine::geom::Vec2;
use egui::{Color32, Painter, Pos2, Stroke, Ui};

/// Create a transient handle, or select the nearest existing handle on click.
pub fn click(app: &mut App, at: Pos2, zoom: f32) {
    let tolerance = 10.0 / zoom.max(0.001);
    if let Some(index) = nearest_handle(&app.smart_controller_points, at, tolerance) {
        app.smart_controller_selected = Some(index);
    } else {
        app.smart_controller_points.push(Vec2::new(at.x, at.y));
        app.smart_controller_selected = Some(app.smart_controller_points.len() - 1);
    }
}

/// Begin a drag only when an existing handle is hit. Empty-stage clicks remain the create gesture.
pub fn drag_start(app: &mut App, at: Pos2, zoom: f32) -> Drag {
    let tolerance = 10.0 / zoom.max(0.001);
    match nearest_handle(&app.smart_controller_points, at, tolerance) {
        Some(index) => {
            app.smart_controller_selected = Some(index);
            Drag::SmartControllerHandle { index }
        }
        None => Drag::None,
    }
}

/// Move a transient handle to the current stage-space pointer position.
pub fn drag_to(app: &mut App, index: usize, at: Pos2) {
    if move_handle(&mut app.smart_controller_points, index, at) {
        app.smart_controller_selected = Some(index);
    }
}

/// Return the nearest handle within a stage-space tolerance.
pub fn nearest_handle(points: &[Vec2], at: Pos2, tolerance: f32) -> Option<usize> {
    if !tolerance.is_finite() || tolerance < 0.0 || !at.x.is_finite() || !at.y.is_finite() {
        return None;
    }
    points.iter().enumerate().filter_map(|(index, point)| {
        if !point.x.is_finite() || !point.y.is_finite() { return None; }
        let distance = (point.x - at.x).hypot(point.y - at.y);
        (distance <= tolerance).then_some((index, distance))
    }).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(index, _)| index)
}

/// Update one transient handle. Invalid coordinates or indices leave the state unchanged.
pub fn move_handle(points: &mut [Vec2], index: usize, at: Pos2) -> bool {
    if !at.x.is_finite() || !at.y.is_finite() { return false; }
    let Some(point) = points.get_mut(index) else { return false; };
    *point = Vec2::new(at.x, at.y);
    true
}

/// Draw transient controller handles above the rendered artwork.
pub fn draw(app: &App, painter: &Painter, to_screen: impl Fn(Vec2) -> Pos2) {
    if app.tool != Tool::SmartController && app.smart_controller_selected.is_none() { return; }
    for (index, point) in app.smart_controller_points.iter().enumerate() {
        let centre = to_screen(*point);
        let selected = app.smart_controller_selected == Some(index);
        let radius = if selected { 7.0 } else { 5.5 };
        let fill = if selected { Color32::from_rgb(255, 190, 70) } else { Color32::from_rgb(65, 170, 230) };
        painter.circle_filled(centre, radius, fill);
        painter.circle_stroke(centre, radius + 3.0, Stroke::new(1.5, Color32::WHITE));
    }
}

/// Native Properties UI for the current transient handle draft.
pub fn show(app: &mut App, ui: &mut Ui) {
    ui.label(tr!("Create and select controller handles on the Stage. Drag a handle to reposition it."));
    ui.separator();
    match app.smart_controller_selected.and_then(|i| app.smart_controller_points.get(i).copied().map(|p| (i, p))) {
        Some((index, point)) => {
            ui.label(tr!("Selected handle {index}", index = index + 1));
            ui.horizontal(|ui| {
                ui.label(tr!("Stage position"));
                ui.monospace(format!("X {:.1} · Y {:.1}", point.x, point.y));
            });
            if ui.button(tr!("Remove draft handle")).clicked() {
                app.smart_controller_points.remove(index);
                app.smart_controller_selected = if app.smart_controller_points.is_empty() { None } else { Some(index.min(app.smart_controller_points.len() - 1)) };
            }
        }
        None => { ui.label(tr!("No handle selected. Click the Stage to add one.")); }
    }
    if !app.smart_controller_points.is_empty() && ui.button(tr!("Clear all draft handles")).clicked() {
        app.smart_controller_points.clear();
        app.smart_controller_selected = None;
    }
    ui.separator();
    ui.label(egui::RichText::new(tr!("Draft UI only — not saved")).color(Color32::from_rgb(220, 170, 90)).strong());
    ui.label(tr!("Bone binding, mapping evaluation, target application, persistent storage and live playback are not connected yet. These handles are temporary editor guides and do not change artwork."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_handle_selects_the_closest_point_within_tolerance() {
        let points = [Vec2::new(0.0, 0.0), Vec2::new(18.0, 0.0), Vec2::new(40.0, 0.0)];
        assert_eq!(nearest_handle(&points, Pos2::new(16.0, 0.0), 5.0), Some(1));
        assert_eq!(nearest_handle(&points, Pos2::new(30.0, 0.0), 5.0), None);
    }

    #[test]
    fn invalid_tolerance_and_non_finite_points_are_ignored() {
        let points = [Vec2::new(f32::NAN, 0.0), Vec2::new(1.0, 1.0)];
        assert_eq!(nearest_handle(&points, Pos2::new(1.0, 1.0), f32::NAN), None);
        assert_eq!(nearest_handle(&points, Pos2::new(1.0, 1.0), 1.0), Some(1));
    }

    #[test]
    fn dragging_moves_only_the_selected_transient_handle() {
        let mut points = [Vec2::new(1.0, 2.0), Vec2::new(10.0, 20.0)];
        assert!(move_handle(&mut points, 1, Pos2::new(15.0, 25.0)));
        assert_eq!(points, [Vec2::new(1.0, 2.0), Vec2::new(15.0, 25.0)]);
    }

    #[test]
    fn invalid_drag_coordinates_or_handle_indices_do_not_mutate_state() {
        let mut points = [Vec2::new(1.0, 2.0)];
        assert!(!move_handle(&mut points, 0, Pos2::new(f32::NAN, 4.0)));
        assert!(!move_handle(&mut points, 9, Pos2::new(3.0, 4.0)));
        assert_eq!(points, [Vec2::new(1.0, 2.0)]);
    }
}
