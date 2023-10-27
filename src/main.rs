use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_names: Vec<PathBuf>,
    #[arg(short='b', long, action = clap::ArgAction::SetTrue)]
    number_nonblank: bool,
    #[arg(short='E', action = clap::ArgAction::SetTrue)]
    show_ends: bool,
    #[arg(short='n', action = clap::ArgAction::SetTrue)]
    number: bool,
    #[arg(short='T', action = clap::ArgAction::SetTrue)]
    show_tabs: bool,
}

fn main() {
    let args = Args::parse();

    print_file(
        &args.file_names,
        args.number_nonblank,
        args.show_ends,
        args.number,
        args.show_tabs,
    );
}

fn print_file(
    files: &Vec<PathBuf>,
    number_non_blank: bool,
    show_ends: bool,
    number: bool,
    show_tabs: bool,
) {
    let mut line_number = 1;
    for file in files {
        validate_file(file).unwrap();
        let contents = fs::read_to_string(file).expect("File Does not exist");
        for cur_line in contents.lines() {
            let mut line = String::from(cur_line);

            if number_non_blank && !line.is_empty() {
                line = format!("{} {}", line_number, line);
                line_number += 1;
            }

            if !number_non_blank && number {
                line = format!("{} {}", line_number, line);
                line_number += 1;
            }

            if show_tabs {
                line = line.replace('\t', "^I");
            }

            if show_ends {
                line = format!("{}$", line);
            }

            println!("{}", line);
        }
    }
}

fn validate_file(file: &Path) -> Result<bool, String> {
    if !file.exists() {
        Err(format!(
            "cat_clone: {:?} No such file or directory",
            file.file_name().unwrap()
        ))
    } else if file.is_dir() {
        Err(format!(
            "cat_clone: {:?}: Is a directory",
            file.file_name().unwrap()
        ))
    } else if file.is_symlink() {
        Err(format!(
            "cat_clone: {:?}: Is a symlink",
            file.file_name().unwrap()
        ))
    } else {
        Ok(true)
    }
}
