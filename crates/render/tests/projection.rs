//! Parallel 3D views of model space (`Options::projection`): plan view draws as before, 3D lines
//! and flat objects at an elevation land where the view puts them, and block references carry
//! their insertion point's z.

use std::sync::Arc;

use cadcraft_doc::{Block, Circle, Common, Drawing, Entity, EntityKind, Handle, Insert, Line, Space};
use cadcraft_geom::{Mat4, Vec3};
use cadcraft_render::{Options, build};

fn front() -> Mat4 {
    Mat4::view(Vec3::new(0.0, -1.0, 0.0), Vec3::ZERO, 0.0)
}

fn bounds(d: &Drawing, projection: Option<Mat4>) -> (f64, f64, f64, f64) {
    let l = build(d, &Space::Model, &Options { projection, ..Options::default() });
    (l.bounds.min.x, l.bounds.min.y, l.bounds.max.x, l.bounds.max.y)
}

fn near(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    [a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3].iter().all(|d| d.abs() < 1e-6)
}

fn line(a: Vec3, b: Vec3) -> EntityKind {
    EntityKind::Line(Line { a, b })
}

#[test]
fn plan_view_draws_as_without_a_projection() {
    let mut d = Drawing::new_metric();
    d.add(&Space::Model, Common::default(), line(Vec3::new(1.0, 2.0, 5.0), Vec3::new(8.0, 3.0, 9.0))).unwrap();
    d.add(&Space::Model, Common::default(), EntityKind::Circle(Circle { center: Vec3::new(4.0, 4.0, 3.0), radius: 2.0 })).unwrap();
    let plan = build(&d, &Space::Model, &Options::default());
    let viewed = build(&d, &Space::Model, &Options { projection: Some(Mat4::view(Vec3::Z, Vec3::ZERO, 0.0)), ..Options::default() });
    assert_eq!(plan.verts, viewed.verts);
}

#[test]
fn front_view_shows_heights() {
    let mut d = Drawing::new_metric();
    // A vertical line: a point in plan, a line 10 high from the front.
    d.add(&Space::Model, Common::default(), line(Vec3::new(3.0, 7.0, 0.0), Vec3::new(3.0, 7.0, 10.0))).unwrap();
    assert!(near(bounds(&d, Some(front())), (3.0, 0.0, 3.0, 10.0)));

    // A circle at elevation 5, seen edge-on: a horizontal segment at height 5.
    let mut d = Drawing::new_metric();
    d.add(&Space::Model, Common::default(), EntityKind::Circle(Circle { center: Vec3::new(0.0, 0.0, 5.0), radius: 2.0 })).unwrap();
    assert!(near(bounds(&d, Some(front())), (-2.0, 5.0, 2.0, 5.0)));
}

#[test]
fn block_references_carry_their_elevation() {
    let mut d = Drawing::new_metric();
    let mut blk = Block::new("B");
    blk.entities.push(Entity::new(Handle(1), line(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0))));
    d.blocks.insert("B".into(), Arc::new(blk));
    let ins = Insert {
        block: "B".into(),
        insert: Vec3::new(10.0, 0.0, 4.0),
        scale: Vec3::new(2.0, 2.0, 2.0),
        rotation: 0.0,
        attribs: vec![],
        cols: 1,
        rows: 1,
        col_spacing: 0.0,
        row_spacing: 0.0,
    };
    d.add(&Space::Model, Common::default(), EntityKind::Insert(ins)).unwrap();
    assert!(near(bounds(&d, Some(front())), (10.0, 4.0, 12.0, 4.0)));
}

#[test]
fn isometric_view_of_a_cube() {
    let mut d = Drawing::new_metric();
    let c = |x: f64, y: f64, z: f64| Vec3::new(x, y, z);
    for (a, b) in [
        (c(0.0, 0.0, 0.0), c(1.0, 0.0, 0.0)),
        (c(1.0, 0.0, 0.0), c(1.0, 1.0, 0.0)),
        (c(1.0, 1.0, 0.0), c(0.0, 1.0, 0.0)),
        (c(0.0, 1.0, 0.0), c(0.0, 0.0, 0.0)),
        (c(0.0, 0.0, 0.0), c(0.0, 0.0, 1.0)),
        (c(1.0, 1.0, 0.0), c(1.0, 1.0, 1.0)),
    ] {
        d.add(&Space::Model, Common::default(), line(a, b)).unwrap();
    }
    let iso = Mat4::view(Vec3::new(1.0, -1.0, 1.0), Vec3::ZERO, 0.0);
    let (x0, y0, x1, y1) = bounds(&d, Some(iso));
    // Wider than the cube's 1-unit sides (a diagonal across), and its height shows.
    assert!((x1 - x0 - 2f64.sqrt()).abs() < 1e-6, "width {}", x1 - x0);
    assert!(y1 - y0 > 1.0, "height {}", y1 - y0);
}
