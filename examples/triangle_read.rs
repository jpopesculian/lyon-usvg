use lyon_usvg::*;
use std::ops::Deref;

fn main() {
    let svg = r#"
    <svg width="500" height="500" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
        <defs/>
        <path fill="none" stroke="none" d="M 100 100 L 400 100 L 250 400 Z"/>
    </svg>
    "#;
    let tree = usvg::Tree::from_str(svg, &Default::default()).unwrap();
    let path = match tree.root().last_child().unwrap().borrow().deref() {
        usvg::NodeKind::Path(path) => path.data.to_path(),
        _ => panic!("should be a path"),
    };
    println!("{:?}", path);
}
