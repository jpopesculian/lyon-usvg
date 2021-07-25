use lyon_path::geom::point;
use lyon_path::Polygon;
use lyon_usvg::*;
use std::rc::Rc;
use usvg::NodeExt;

fn main() {
    let triangle = Polygon {
        points: &[point(100., 100.), point(400., 100.), point(250., 400.)],
        closed: true,
    };
    let tree = usvg::Tree::create(usvg::Svg {
        size: usvg::Size::new(500., 500.).unwrap(),
        view_box: usvg::ViewBox {
            rect: usvg::Rect::new(0., 0., 500., 500.).unwrap(),
            aspect: usvg::AspectRatio::default(),
        },
    });
    tree.root().append_kind(usvg::NodeKind::Path(usvg::Path {
        data: Rc::new(triangle.path_events().into_path_data()),
        fill: Some(usvg::Fill::default()),
        ..Default::default()
    }));
    println!("{}", tree.to_string(&Default::default()));
}
