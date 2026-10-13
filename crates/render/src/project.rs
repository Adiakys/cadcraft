//! Parallel 3D views ([`crate::Options::projection`]). A flat object lies in a plane `z =
//! elevation`, which a parallel view maps to the list by a 2D affine map: it draws through that
//! map like in plan view. 3D lines, polylines, faces and meshes are projected point by point.

use cadcraft_doc::{Entity, EntityKind, Face3d, Line, PolyfaceMesh, Polyline3d, Solid};
use cadcraft_geom::{Mat4, Vec3};

/// The elevation of the plane a flat object lies in.
pub(crate) fn elevation(k: &EntityKind) -> f64 {
    match k {
        EntityKind::Point(p) => p.p.z,
        EntityKind::Circle(c) => c.center.z,
        EntityKind::Arc(a) => a.center.z,
        EntityKind::Ellipse(el) => el.center.z,
        EntityKind::LwPolyline(p) => p.elevation,
        EntityKind::Text(t) => t.insert.z,
        EntityKind::AttDef(a) => a.text.insert.z,
        EntityKind::MText(t) => t.insert.z,
        EntityKind::Insert(i) => i.insert.z,
        EntityKind::Hatch(h) => h.elevation,
        EntityKind::Dimension(dm) => dm.defpt.z,
        EntityKind::Leader(l) => l.vertices.first().map_or(0.0, |v| v.z),
        EntityKind::MLeader(m) => m.landing.z,
        EntityKind::Image(i) => i.insert.z,
        EntityKind::Table(t) => t.insert.z,
        EntityKind::Tolerance(t) => t.insert.z,
        EntityKind::Ray(r) | EntityKind::XLine(r) => r.base.z,
        _ => 0.0,
    }
}

/// A 3D object in view coordinates (x, y on the list; z towards the viewer), drawn as it is with
/// an identity transform; `None` for flat objects, which draw through [`Mat4::plane`].
pub(crate) fn in_view(e: &Entity, view: &Mat4) -> Option<Entity> {
    let p = |q: &Vec3| view.apply(*q);
    let kind = match &e.kind {
        EntityKind::Line(l) => EntityKind::Line(Line { a: p(&l.a), b: p(&l.b) }),
        EntityKind::Polyline3d(pl) => EntityKind::Polyline3d(Polyline3d { points: pl.points.iter().map(p).collect(), closed: pl.closed }),
        EntityKind::Face3d(f) => EntityKind::Face3d(Face3d { corners: f.corners.map(|c| p(&c)), hidden_edges: f.hidden_edges }),
        EntityKind::PolyfaceMesh(m) => {
            EntityKind::PolyfaceMesh(PolyfaceMesh { vertices: m.vertices.iter().map(p).collect(), faces: m.faces.clone() })
        }
        EntityKind::Solid(s) => EntityKind::Solid(Solid { corners: s.corners.map(|c| p(&c)) }),
        EntityKind::Trace(s) => EntityKind::Trace(Solid { corners: s.corners.map(|c| p(&c)) }),
        _ => return None,
    };
    Some(Entity { handle: e.handle, common: e.common.clone(), kind })
}

/// A block reference's transform in 3D: its 2D transform `m` (insertion point, rotation, x and y
/// scale, base point) with the insertion point's z and the z scale.
pub(crate) fn insert_transform(m: &cadcraft_geom::Mat3, ins: &cadcraft_doc::Insert, base: Vec3) -> Mat4 {
    let mut t = Mat4::from_mat3(m);
    t.m[2][2] = ins.scale.z;
    t.m[2][3] = ins.insert.z - base.z * ins.scale.z;
    t
}
