use comfy_table::Table;
use std::fs::read_dir;
use std::io::{stderr, Write};
use std::{env, process};

struct File {
    name: String,
    is_dir: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = match args.len() {
        1 => ".".to_string(),
        2 => args[1].to_string(),
        _ => {
            stderr().write_all(b"Too many arguments\n").unwrap();
            process::exit(1);
        }
    };
    let result = read_dir(dir).unwrap();

    let mut table = Table::new();

    for entry in result {
        let value = &entry.unwrap();
        let file = File {
            name: value.file_name().to_str().unwrap().to_string(),
            is_dir: value.path().is_dir(),
        };
        table.add_row([&file.name, &file.is_dir.to_string()]);
    }
    println!("{table}")
}
