use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use clap::Parser;
use formatter::format::{FormatOptions, Formatter};

#[derive(Debug, Parser)]
#[clap(version = "0.1.0", author = "sor4chi")]
struct Args {
    #[arg(help = "file path to format, default is stdin")]
    file_path: Option<String>,

    #[arg(
        short = 't',
        long = "tabs",
        help = "use tab for indent, default is space",
        default_missing_value = "true",
        num_args = 0..=1,
        require_equals = true
    )]
    use_tabs: Option<bool>,

    #[arg(
        short = 's',
        long = "spaces",
        help = "use spaces for indent, -s 2 means 2 spaces, default is 4"
    )]
    spaces: Option<usize>,

    #[arg(
        short = 'c',
        long = "trailing_commas",
        help = "use trailing comma for array and object",
        default_missing_value = "true",
        num_args = 0..=1,
        require_equals = true
    )]
    trailing_commas: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let fp = match args.file_path {
        Some(path) => path,
        None => {
            panic!("file path is required");
        }
    };
    let path = Path::new(&fp);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("failed to open file: {}", e);
        }
    };
    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            panic!("failed to read file: {}", e);
        }
    }
    let mut formatter = Formatter::new(Some(FormatOptions {
        use_tabs: args.use_tabs.unwrap_or(false),
        spaces: args.spaces.unwrap_or(4),
        trailing_commas: args.trailing_commas.unwrap_or(false),
    }));
    let formatted = formatter.format(&buf);

    // write to file
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("failed to create file: {}", e);
        }
    };

    match file.write_all(formatted.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            panic!("failed to write file: {}", e);
        }
    }
}
