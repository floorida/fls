use std::fs;
use colored::*;
mod structs;
use clap::Parser;
use crate::structs::*;
// use walkdir::WalkDir;
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short)]
    icons: bool,
}

fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        let file = path.unwrap().path();
        let filename = file.file_name().unwrap().to_str().unwrap();
        // FIXME: since files may not necessarily contain 
        // an extension, they might be confused with folders
        if file.is_file() {
            if let Some(ftype) = filename.split(".").nth(1) {
                let filetype = parse_file_type(ftype).unwrap();
                let filetypec = parse_file_type(ftype).unwrap();
                let color = get_color(filetype);
                let icon = get_icon(filetypec).unwrap();
                if args.icons == true {
                    match color {
                        Some(Color::Magenta) => print!("{} {}   ", icon, filename.magenta()),
                        Some(Color::Yellow) => print!("{} {}   ", icon, filename.yellow()),
                        Some(Color::White) => print!("{}   ", filename.white()),
                        Some(Color::Green) => print!("{} {}   ", icon, filename.green()),
                        Some(Color::Blue) => print!("{} {}   ", icon, filename.blue().underline()),
                        _ => print!("idk"),
                    }
                }
                else {
                    match color {
                        Some(Color::Magenta) => print!("{}   ", filename.magenta()),
                        Some(Color::Yellow) => print!("{}   ", filename.yellow()),
                        Some(Color::White) => print!("{}   ", filename.white()),
                        Some(Color::Green) => print!("{}   ", filename.green()),
                        Some(Color::Blue) => print!("{}   ", filename.blue().underline()),
                        _ => print!("idk"),
                    }
                }

            }
            else {
                print!("{}  ", filename.white());
            }
        }
        else {
            print!("{}  ", filename.blue().bold());
        }  
    }
    println!("");
}