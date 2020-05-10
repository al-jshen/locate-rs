use std::env;
use std::fs::{File, read_to_string};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use walkdir::WalkDir;
//use rayon::prelude::ParallelIterator;
//use rayon::iter::ParallelBridge;
use regex::Regex;
use clap::{App, Arg, ArgMatches, crate_authors, crate_version};


fn cache(cachepath: &str, filters: &Vec<&str>) {

    let pathFilters = filters.into_iter().map(|f| Path::new(f)).collect::<Vec<_>>();

    let mut output = File::create(cachepath).unwrap();

    for entry in WalkDir::new("/home").follow_links(true)
        .into_iter()
        .filter_entry(|d| !pathFilters.contains(&d.path()))
        .filter_map(|e| e.ok()) {
        write!(output, "{}\n", entry.path().display()).unwrap();
    }
}

fn find(path: &str, pattern: &str) -> Vec<()> {
    let pattern = Regex::new(format!(r#"{}"#, regex::escape(pattern)).as_str()).unwrap();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered.lines()
//        .into_iter()
//        .par_bridge()
        .map(|line| line.unwrap())
        .filter(|line| pattern.is_match(line))
        .map(|m| println!("{}", m))
        .collect::<Vec<_>>()
}

fn main() {

    let cfg_file = read_to_string("/etc/locate-rs.conf").unwrap();


    let options: ArgMatches = App::new("locate clone built with Rust")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Performs parallelized search for files using regex.")
        .arg(Arg::with_name("cache")
            .help("path of cache file")
            .short("c")
            .long("c"))
        .arg(Arg::with_name("search")
            .help("pattern")
            .short("s")
            .long("s")
            .takes_value(true))
        .get_matches();



    if let Some(pattern) = options.value_of("search") {
        find("/tmp/locate-rs.cache", pattern);
    }

    if options.is_present("cache") {
        let filters = cfg_file.trim().split(" ").collect::<Vec<_>>();
        cache("/tmp/locate-rs.cache", &filters);
    }

}
