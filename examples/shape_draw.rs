use lyon_path::geom::point;
use lyon_path::Path;
use lyon_usvg::*;
use std::rc::Rc;
use usvg::NodeExt;

fn main() {
    let path = {
        let mut builder = Path::svg_builder();
        builder.move_to(point(100., 100.));
        builder.cubic_bezier_to(point(100., 300.), point(300., 0.), point(400., 100.));
        builder.line_to(point(300., 400.));
        builder.quadratic_bezier_to(point(200., 500.), point(100., 400.));
        builder.close();
        builder.build()
    };
    let tree = usvg::Tree::create(usvg::Svg {
        size: usvg::Size::new(500., 500.).unwrap(),
        view_box: usvg::ViewBox {
            rect: usvg::Rect::new(0., 0., 500., 500.).unwrap(),
            aspect: usvg::AspectRatio::default(),
        },
    });
    tree.root().append_kind(usvg::NodeKind::Path(usvg::Path {
        data: Rc::new(path.into_path_data()),
        fill: Some(usvg::Fill::default()),
        ..Default::default()
    }));
    println!("{}", tree.to_string(&Default::default()));
}
