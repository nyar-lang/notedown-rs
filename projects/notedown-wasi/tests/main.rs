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

#[test]
#[ignore]
fn export() {
    let config = Opts { rustfmt: false, runtime_path: Some("wit_bindgen::rt".to_string()), ..Opts::default() };
    let mut generator = config.build();
    let mut resolve = Resolve::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let (pkg, _) = resolve.push_dir(&here.join("wit")).unwrap();
    let world = resolve.select_world(pkg, None).unwrap();
    let mut files = Files::default();
    generator.generate(&resolve, world, &mut files).unwrap();

    let wit = here.join("src").join("exports");
    // 检查目录是否存在并生成
    if !wit.exists() {
        std::fs::create_dir_all(&wit).unwrap();
    }

    for (file, text) in files.iter() {
        let mut file = File::create(wit.join(file)).unwrap();
        file.write_all(text).unwrap();
    }
}
