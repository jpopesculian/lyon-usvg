use lyon_path::geom::Point;
use lyon_path::{Event, Path};
use num_traits::ToPrimitive;
use usvg::{PathData, PathSegment};

pub trait IntoPathSegment {
    fn into_path_segment(self) -> Option<PathSegment>;
}

impl<EndT, CtrlT> IntoPathSegment for Event<Point<EndT>, Point<CtrlT>>
where
    EndT: ToPrimitive,
    CtrlT: ToPrimitive,
{
    fn into_path_segment(self) -> Option<PathSegment> {
        match self {
            Self::Begin { at } => Some(PathSegment::MoveTo {
                x: at.x.to_f64().unwrap(),
                y: at.y.to_f64().unwrap(),
            }),
            Self::Line { to, .. } => Some(PathSegment::LineTo {
                x: to.x.to_f64().unwrap(),
                y: to.y.to_f64().unwrap(),
            }),
            Self::Cubic {
                to, ctrl1, ctrl2, ..
            } => Some(PathSegment::CurveTo {
                x1: ctrl1.x.to_f64().unwrap(),
                y1: ctrl1.y.to_f64().unwrap(),
                x2: ctrl2.x.to_f64().unwrap(),
                y2: ctrl2.y.to_f64().unwrap(),
                x: to.x.to_f64().unwrap(),
                y: to.y.to_f64().unwrap(),
            }),
            Self::Quadratic { from, to, ctrl } => {
                let from = Point::new(from.x.to_f64().unwrap(), from.y.to_f64().unwrap());
                let to = Point::new(to.x.to_f64().unwrap(), to.y.to_f64().unwrap());
                let ctrl = Point::new(ctrl.x.to_f64().unwrap(), ctrl.y.to_f64().unwrap());
                let ctrl1 = from + (ctrl - from) * (2. / 3.);
                let ctrl2 = to + (ctrl - to) * (2. / 3.);
                Some(PathSegment::CurveTo {
                    x1: ctrl1.x,
                    y1: ctrl1.y,
                    x2: ctrl2.x,
                    y2: ctrl2.y,
                    x: to.x,
                    y: to.y,
                })
            }
            Self::End { close: true, .. } => Some(PathSegment::ClosePath),
            Self::End { close: false, .. } => None,
        }
    }
}

pub trait IntoPathData {
    fn into_path_data(self) -> PathData;
}

impl<T, EndT, CtrlT> IntoPathData for T
where
    T: IntoIterator<Item = Event<Point<EndT>, Point<CtrlT>>>,
    EndT: ToPrimitive,
    CtrlT: ToPrimitive,
{
    fn into_path_data(self) -> PathData {
        PathData(
            self.into_iter()
                .filter_map(|e| e.into_path_segment())
                .collect(),
        )
    }
}

pub trait ToPath {
    fn to_path(&self) -> Path;
}

impl ToPath for PathData {
    fn to_path(&self) -> Path {
        let mut builder = Path::svg_builder();
        for segment in self.iter() {
            match *segment {
                usvg::PathSegment::MoveTo { x, y } => {
                    builder.move_to(Point::new(x as f32, y as f32));
                }
                usvg::PathSegment::LineTo { x, y } => {
                    builder.line_to(Point::new(x as f32, y as f32));
                }
                usvg::PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    builder.cubic_bezier_to(
                        Point::new(x1 as f32, y1 as f32),
                        Point::new(x2 as f32, y2 as f32),
                        Point::new(x as f32, y as f32),
                    );
                }
                usvg::PathSegment::ClosePath => {
                    builder.close();
                }
            }
        }
        builder.build()
    }
}
