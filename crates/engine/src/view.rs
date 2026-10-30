use super::{Deserialize, Serialize, Vec2};

/// A 2D view of a space: world centre and visible height in drawing units.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct View {
    pub center: Vec2,
    pub height: f64,
}

impl Default for View {
    fn default() -> Self {
        View { center: Vec2::new(6.0, 4.5), height: 10.0 }
    }
}

pub(crate) fn usable_view(v: &View) -> bool {
    v.center.is_finite() && v.height.is_finite() && v.height > 1e-12
}

/// How model space is drawn (VSCURRENT): edges only, or 3D faces and meshes filled and lit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisualStyle {
    #[default]
    Wireframe2d,
    Shaded,
    ShadedWithEdges,
}

impl VisualStyle {
    /// The name shown in the view label and the View ▸ Visual Styles menu.
    pub fn label(self) -> &'static str {
        match self {
            VisualStyle::Wireframe2d => "2D Wireframe",
            VisualStyle::Shaded => "Shaded",
            VisualStyle::ShadedWithEdges => "Shaded with Edges",
        }
    }
    /// The display list's `shaded` and `shaded_edges` options.
    pub fn shading(self) -> (bool, bool) {
        (self != VisualStyle::Wireframe2d, self == VisualStyle::ShadedWithEdges)
    }
}

/// Tiled model viewports (VPORTS): the session side of the drawing's `*Active` configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModelTiles {
    /// The tile corners these views belong to (when the drawing's tiles change, by VPORTS or
    /// undo, its views take over).
    pub rects: Vec<(Vec2, Vec2)>,
    /// Each tile's view; the current tile shows the model space view itself.
    pub views: Vec<View>,
    /// The current tile.
    pub active: usize,
}

/// One tile of the model space drawing area, for the UI ([`Session::model_tiles`]): its corners
/// as fractions of the drawing area (0,0 lower left) and its view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModelTile {
    pub min: Vec2,
    pub max: Vec2,
    pub view: View,
    pub active: bool,
}
