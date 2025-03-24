use std::{env::args, io::{stdin, Read}, process::exit};

use peg_to_json::parser;

fn main() {
    let args = args().collect::<Vec<_>>();
    let (proc, args) = args.split_first().unwrap();
    if !args.is_empty() {
        if args[0] == "-h" || args[0] == "--help" {
            println!("Usage: {proc} [-h | --help]");
            println!("Convert ABNF like PEG declaration to JSON");
            println!("Input from stdin");
            return;
        }
        eprintln!("unexpected args, try -h or --help");
        exit(2)
    }
    let buf = &mut String::new();
    stdin().lock().read_to_string(buf).unwrap();
    match parser::decl_list(buf) {
        Ok(obj) => {
            println!("{}", obj.pretty(2));
        },
        Err(e) => {
            eprintln!("error: {e}");
        },
    }
}
