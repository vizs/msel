/*
 * msel - select multiple items as arguments/from stdin (todo)
 * and echo all the selections to a file in
 * a space separated list.
 *
 * made by viz (https://github.com/vizs)
 *
 * depends: termion
 *
 * licensed under BSD 2-Clause "Simplified" License
 */

const DELIM: &str = " ";
const OFILE: &str = "/tmp/msel.out";
const USAGE: &str = r#"usage: msel [OPTIONS] text...
options:
    -h              print this help message and exit
    -d              change the output delimiter
    -f              change the output file path"#;

use msel;
use std::{env, process::exit, fs};

struct Config {
    delim: String,
    items: Vec<String>,
    out: String,
}

fn usage() {
    eprintln!("{}", USAGE);
    exit(0);
}

fn parse_args() -> Config {
    let argv: Vec<String> = env::args().collect();
    let mut delim: String = String::from(DELIM);
    let mut out: String = String::from(OFILE);

    if argv.len() == 1 || argv[1] == "-h" {
        usage();
    }

    let mut items: Vec<String> = vec![];
    let mut n = 1;

    while n != argv.len() {
        let a = argv[n].to_string();
        match a.as_str() {
            "-d" => { delim = argv[n+1].to_string(); n += 1; },
            "-f" => { out = argv[n+1].to_string(); n += 1; },
            _ => items.push(a.to_string()),
        }
        n += 1;
    }

    Config { delim: delim, items: items, out: out }
}

fn main() {
    let config = parse_args();
    let mut items = msel::Items::new(&config.items);
    msel::ui::run(&mut items);

    let mut result: String = String::new();

    for (n, i) in items.sel_items.iter().enumerate() {
        result.push_str(i);
        if n != items.sel_items.len() - 1 {
            result.push_str(&format!("{}", config.delim));
        }
    }

    fs::write(&config.out, &result)
        .unwrap_or_else(|_| {
            eprintln!("error: couldn't write to out file!");
            exit(1);
        });
}
