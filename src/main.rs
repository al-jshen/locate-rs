use std::env;
use std::fs::{File, read_to_string};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use std::ffi::OsStr;
use walkdir::WalkDir;
use regex::Regex;
use clap::{App, Arg, ArgMatches, crate_authors, crate_version};

fn cache(cachepath: &str, filters: &Vec<&str>) {

    let file_filters = filters[0].trim().split(" ").into_iter().map(|f| OsStr::new(f)).collect::<Vec<_>>();
    let path_filters = filters[1].trim().split(" ").into_iter().map(|f| Path::new(f)).collect::<Vec<_>>();

    let mut output = File::create(cachepath).unwrap();
    for entry in WalkDir::new("/").follow_links(true)
        .into_iter()
        .filter_entry(|d| (&d.path() == &Path::new("/")) || (!file_filters.contains(&d.path().file_name().unwrap()) && !path_filters.contains(&d.path().parent().unwrap())))
        .filter_map(|e| e.ok()) {
        write!(output, "{}\n", entry.path().display()).unwrap();
    }
}

fn find(path: &str, pattern: &str) {
    let regexpattern = Regex::new(pattern).unwrap();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered.lines()
        .map(|line| line.unwrap())
        .filter(|line| regexpattern.is_match(line))
        .for_each(|m| println!("{}", m));
}

fn main() {

    let cfg_file = read_to_string("/etc/locate-rs.conf").unwrap();
    
    let options: ArgMatches = App::new("locate clone built with Rust")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Performs fast search of files using regex.")
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
        let filters = cfg_file.trim().split("\n").collect::<Vec<_>>();
        cache("/tmp/locate-rs.cache", &filters);
    }

}
