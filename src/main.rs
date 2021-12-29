use std::fs;
use colored::*;
mod structs;
use crate::structs::*;
// use walkdir::WalkDir;
// #[derive(Parser, Debug)]
// #[clap(about, version, author)]
// struct Args {

// }

fn main() {
    // let args = Args::parse();
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let file = path.unwrap().path();
        let filename = file.file_name().unwrap().to_str().unwrap();
        if let Some(ftype) = filename.split(".").nth(1) {
            let filetype = parse_file_type(ftype).unwrap();
            let color = get_color(filetype);
            match color {
                Some(Color::Magenta) => print!("{}  ", filename.magenta()),
                Some(Color::Yellow) => print!("{}  ", filename.yellow()),
                Some(Color::White) => print!("{}  ", filename.white()),
                _ => print!("idk"),
            }
        }
        else {
            print!("{}  ", filename.blue().bold());
        }
    }
    println!("");
}
