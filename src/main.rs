use once_cell::sync::Lazy;
use std::path::PathBuf;
use structopt::StructOpt;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;
use anyhow::Result;

static DEFAULT_PATH: Lazy<String> = Lazy::new(|| format!("{}", env::var("PWD").as_deref().unwrap_or(".")));

const SKIP_FOLDERS: &'static [&'static str] = &["target/"];

#[derive(Debug, StructOpt)]
#[structopt(name = "lines_of_code", about = "A CLI tool to count the lines of code in the provided folder")]
struct Opts {
    #[structopt(short, long, default_value = ".rs", about = "Provide an extension of a file like .rs")]
    file_extension: String,

    #[structopt(short, long, parse(from_os_str), default_value = &DEFAULT_PATH)]
    path: PathBuf,
}

fn main() {
    let opt: Opts = Opts::from_args();

    let mut lines_of_code: i32 = 0;
    let mut number_of_files: i32 = 0;

    for f in WalkDir::new(opt.path.clone()).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if match_file(f.path().display().to_string(), opt.file_extension.clone()) {
            if let Ok(cnt) = count_lines(f.path().to_path_buf()) {
                lines_of_code += cnt;
                number_of_files += 1;
            }
        }
    }

    println!("Found {} line{} of code in {} file{} in folder: {}", lines_of_code, if lines_of_code == 0 || lines_of_code > 1 {"s"} else {""}, number_of_files, if number_of_files == 0 || number_of_files > 1 {"s"} else {""}, opt.path.clone().as_path().display().to_string());
}

fn match_file(f_name: String, file_extension: String) -> bool {
    if !f_name.ends_with(file_extension.as_str()) {
        return false;
    }

    for path in SKIP_FOLDERS.into_iter() {
        if f_name.contains(path) {
            println!("Skipping file: {}", f_name.clone());
            return false
        }
    }

    true
}

fn count_lines(path: PathBuf) -> Result<i32> {
    let f = BufReader::new(File::open(path).expect("Cannot open file"));
    let mut cnt = 0;

    for _ in f.lines() {
        cnt += 1;
    }

    Ok(cnt)
}