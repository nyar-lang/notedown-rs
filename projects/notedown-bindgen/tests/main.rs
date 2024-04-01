#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::{fs::File, io::Write, path::Path};
use wit_bindgen_core::Files;
use wit_bindgen_rust::Opts;
use wit_parser::Resolve;

#[test]
fn ready() {
    println!("it, works!")
}
