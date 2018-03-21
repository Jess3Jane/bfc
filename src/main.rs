extern crate bfc;
use bfc::parser::parse;
use bfc::interpreter;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut argiter = args();
    argiter.next();
    let filename = match argiter.next() {
        Some(arg) => arg,
        None => panic!("Please provide a file to run"),
    };

    let mut f = File::open(filename).unwrap();
    let mut program = String::new();
    f.read_to_string(&mut program).unwrap();

    parse(&program).unwrap().run();
}
