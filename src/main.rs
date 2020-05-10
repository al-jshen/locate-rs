use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use walkdir::WalkDir;
//use rayon::prelude::*;


fn cache(path: &str, cachepath: &str) {

    let mut output = File::create(cachepath).unwrap();

    for entry in WalkDir::new(path).follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        write!(output, "{}\n", entry.path().display()).unwrap();
    }
 
}

fn main() {

    let args: Vec<String> = env::args().collect();
    cache(&args[1], &args[2]);
    println!("Cached.")
}
