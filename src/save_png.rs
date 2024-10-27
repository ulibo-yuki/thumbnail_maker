use std::{fs, path::PathBuf};
use resvg::{self, render, tiny_skia::Pixmap, usvg::Transform};

pub fn save_png(save_png_path: PathBuf, tmp_svg_path: PathBuf) -> Result<(), ()>{
    let svg_data = fs::read_to_string(&tmp_svg_path).unwrap();

    println!("{:?}", &svg_data);

    let options = resvg::usvg::Options::default();
    let rtree = resvg::usvg::Tree::from_str(&svg_data, &options).unwrap();
    let mut pixmap = Pixmap::new(1280, 670).expect("failed siza");

    render(&rtree, Transform::from_scale(1.0, 1.0) , &mut pixmap.as_mut());

    match fs::remove_file(&tmp_svg_path) {
        Ok(_) => println!("tmp file removed."),
        Err(_) => eprintln!("failed tmp file."),
    };

    match pixmap.save_png(save_png_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("error: {}", e);
            Err(())
        },
    }
}
