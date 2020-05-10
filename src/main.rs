use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use walkdir::WalkDir;
use rayon::prelude::ParallelIterator;
use rayon::iter::ParallelBridge;
use regex::Regex;


fn cache(path: &str, cachepath: &str) {

    let mut output = File::create(cachepath).unwrap();

    for entry in WalkDir::new(path).follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        write!(output, "{}\n", entry.path().display()).unwrap();
    }
}

fn find(path: &str, pattern: &str) -> Vec<()> {
    let pattern = Regex::new(format!(r#"{}"#, regex::escape(pattern)).as_str()).unwrap();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered.lines()
        .into_iter()
        .par_bridge()
        .map(|line| line.unwrap())
        .filter(|line| pattern.is_match(line))
        .map(|m| println!("{}", m))
        .collect::<Vec<_>>()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    find(&args[2], &args[1]);

}
