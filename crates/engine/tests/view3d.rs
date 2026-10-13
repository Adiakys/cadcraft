//! VPOINT and PLAN: model space seen from a direction (a vector or a preset of View ▸ 3D Views),
//! zoomed to the extents of what the view shows; layouts stay in plan.

use cadcraft_engine::Session;
use serde_json::json;

fn session_with_a_column() -> Session {
    let mut s = Session::new();
    // A 2 × 2 square on the ground and a 10 high column at its corner.
    s.execute("line", &json!({"points": [[0, 0], [2, 0], [2, 2], [0, 2], [0, 0]]})).unwrap();
    s.execute("3dpoly", &json!({"points": [[0, 0, 0], [0, 0, 10]]})).unwrap();
    s
}

#[test]
fn a_preset_view_shows_heights_and_zooms_to_them() {
    let mut s = session_with_a_column();
    let r = s.execute("vpoint", &json!({"preset": "front"})).unwrap();
    assert_eq!(r["direction"], json!([0.0, -1.0, 0.0]));
    let st = s.state().unwrap();
    assert!(st.projection().is_some());
    // From the front the column is 10 high and the square 2 wide: the view fits the height.
    let v = st.view();
    assert!((v.center.y - 5.0).abs() < 1e-6 && v.height >= 10.0 && v.height < 12.0, "{v:?}");
    let g = s.execute("view.get", &json!({})).unwrap();
    assert_eq!(g["direction"], json!([0.0, -1.0, 0.0]));

    // An isometric view looks down at the square and up the column.
    s.execute("vpoint.seiso", &json!({})).unwrap();
    let d = s.state().unwrap().view_dir;
    assert!((d.x - d.z).abs() < 1e-12 && (d.x + d.y).abs() < 1e-12 && d.x > 0.0);

    // PLAN goes back to the plan view.
    s.execute("plan", &json!({})).unwrap();
    assert!(s.state().unwrap().projection().is_none());
    let v = s.state().unwrap().view();
    assert!((v.center.x - 1.0).abs() < 1e-6 && (v.center.y - 1.0).abs() < 1e-6, "{v:?}");
}

#[test]
fn bad_directions_and_layouts_are_refused() {
    let mut s = session_with_a_column();
    assert!(s.execute("vpoint", &json!({"direction": [0, 0, 0]})).is_err());
    assert!(s.execute("vpoint", &json!({"direction": [1, 2]})).is_err());
    assert!(s.execute("vpoint", &json!({"preset": "sideways"})).is_err());
    assert!(s.execute("vpoint", &json!({})).is_err());
    s.execute("vpoint", &json!({"direction": [1, 1, 1]})).unwrap();
    // A layout is drawn in plan whatever model space's view is.
    let name = s.doc().unwrap().layouts[0].name.clone();
    s.execute("layout.set", &json!({"name": name})).unwrap();
    assert!(s.state().unwrap().projection().is_none());
    assert!(s.execute("vpoint", &json!({"preset": "top"})).is_err());
}
