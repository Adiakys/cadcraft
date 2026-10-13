//! 3D views of model space: VPOINT (a viewing direction, or a preset view of View ▸ 3D Views) and
//! PLAN (back to the plan view of the WCS). Both zoom to the extents, as in AutoCAD.

use cadcraft_doc::Space;
use cadcraft_geom::Vec3;
use serde_json::{Value, json};

use super::super::*;
use crate::{Result, Session};

/// The preset views and their viewing directions (towards the viewer).
pub(crate) const PRESETS: [(&str, [f64; 3]); 10] = [
    ("top", [0.0, 0.0, 1.0]),
    ("bottom", [0.0, 0.0, -1.0]),
    ("left", [-1.0, 0.0, 0.0]),
    ("right", [1.0, 0.0, 0.0]),
    ("front", [0.0, -1.0, 0.0]),
    ("back", [0.0, 1.0, 0.0]),
    ("swiso", [-1.0, -1.0, 1.0]),
    ("seiso", [1.0, -1.0, 1.0]),
    ("neiso", [1.0, 1.0, 1.0]),
    ("nwiso", [-1.0, 1.0, 1.0]),
];

pub(super) fn specs() -> Vec<CommandSpec> {
    let mut v = vec![
        CommandSpec::new("vpoint", "Viewpoint", run_vpoint)
            .menu(&["View", "3D Views", "Viewpoint"])
            .params("{direction: [x, y, z] (from the target towards the viewer)} | {preset: top|bottom|left|right|front|back|swiso|seiso|neiso|nwiso} → {direction}")
            .noundo(),
        CommandSpec::new("plan", "Plan View", |s, _| set_direction(s, "plan", Vec3::Z))
            .menu(&["View", "3D Views", "Plan View"])
            .params("{} (the plan view of the WCS, zoomed to the extents)")
            .noundo(),
    ];
    macro_rules! preset {
        ($id:literal, $name:literal, $label:literal) => {
            CommandSpec::new($id, $label, |s, _| run_vpoint(s, &json!({ "preset": $name }))).menu(&["View", "3D Views", $label]).noundo()
        };
    }
    v.extend([
        preset!("vpoint.top", "top", "Top"),
        preset!("vpoint.bottom", "bottom", "Bottom"),
        preset!("vpoint.left", "left", "Left"),
        preset!("vpoint.right", "right", "Right"),
        preset!("vpoint.front", "front", "Front"),
        preset!("vpoint.back", "back", "Back"),
        preset!("vpoint.swiso", "swiso", "SW Isometric"),
        preset!("vpoint.seiso", "seiso", "SE Isometric"),
        preset!("vpoint.neiso", "neiso", "NE Isometric"),
        preset!("vpoint.nwiso", "nwiso", "NW Isometric"),
    ]);
    v
}

fn run_vpoint(s: &mut Session, p: &Value) -> Result<Value> {
    let dir = match (p.get("direction"), str_param(p, "preset")) {
        (Some(d), _) => {
            let c: Vec<f64> = d.as_array().map(|a| a.iter().filter_map(Value::as_f64).collect()).unwrap_or_default();
            match c.as_slice() {
                [x, y, z] => Vec3::new(*x, *y, *z),
                _ => return Err(bad("vpoint", "`direction` must be [x, y, z]")),
            }
        }
        (None, Some(name)) => {
            let found = PRESETS.iter().find(|(n, _)| n.eq_ignore_ascii_case(name.trim()));
            let [x, y, z] =
                found.map(|(_, d)| *d).ok_or_else(|| bad("vpoint", format!("unknown preset `{}`", name.chars().take(40).collect::<String>())))?;
            Vec3::new(x, y, z)
        }
        (None, None) => return Err(bad("vpoint", "give `direction` or `preset`")),
    };
    set_direction(s, "vpoint", dir)
}

/// Look at model space from `dir`, without twist, and zoom to the extents.
fn set_direction(s: &mut Session, cmd: &str, dir: Vec3) -> Result<Value> {
    let len = dir.len();
    if !(len.is_finite() && len > 1e-12) {
        return Err(bad(cmd, "the direction must be a non-zero vector"));
    }
    if s.layout_space() != Space::Model {
        return Err(bad(cmd, "3D views are for model space: switch to the Model tab"));
    }
    let dir = dir * (1.0 / len);
    let st = s.state_mut()?;
    st.view_dir = dir;
    st.view_twist = 0.0;
    s.zoom_extents()?;
    s.touch();
    Ok(json!({ "direction": [dir.x, dir.y, dir.z] }))
}
