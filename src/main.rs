use clap::Parser;
use get_input::get_input;
use std::{env, path::PathBuf};

mod generator_svg;
mod save_png;

/// ## uses
///
/// ```
/// thumbnail_maker test 18 note/Rust/foo
/// ```

#[derive(Parser, Debug)]
struct Args {
    /// ## uses
    /// ```
    /// thumbnail_maker sub_title article_num save_folder
    /// ```
    number: Option<String>,
    sub_title: Option<String>,
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let subtitle_text = match args.sub_title {
        Some(sub_title) => sub_title,
        None => {
            println!("Please input subtitle.👇");
            get_input()
        }
    };
    let number = format!(
        "#{}",
        match args.number {
            Some(num) => num,
            None => {
                println!("Please input number.👇");
                get_input()
            }
        }
    );

    let svg = generator_svg::svg_maker(&subtitle_text, &number);

    let save_dir = match args.path {
        Some(p) => PathBuf::from(p),
        None => match env::current_dir() {
            Ok(cp) => cp,
            Err(_) => return,
        },
    };
    let file_name = format!("{}-{}", &number, &subtitle_text);
    let png_path = save_dir.join(format!("{}.png", file_name));
    let tmp_svg_path = save_dir.join(format!("{}.svg", file_name));

    svg::save(&tmp_svg_path, &svg).unwrap();

    match save_png::save_png(png_path, tmp_svg_path) {
        Ok(_) => println!("save png."),
        Err(_) => eprintln!("error"),
    };
}
