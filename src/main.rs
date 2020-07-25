use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use regex::Regex;
use std::env;
use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use walkdir::WalkDir;
use dirs::home_dir;
// use jwalk::{WalkDir};

fn cache(cachepath: &str, filters: &[&str]) {
    let file_filters = filters[0]
        .trim()
        .split(' ')
        .map(|f| OsStr::new(f))
        .collect::<Vec<_>>();
    let path_filters = filters[1]
        .trim()
        .split(' ')
        .map(|f| Path::new(f))
        .collect::<Vec<_>>();

    let mut output = File::create(cachepath).unwrap();
    for entry in WalkDir::new("/")
        .follow_links(true)
        .into_iter()
        .filter_entry(|d| {
            (d.path() == Path::new("/"))
                || (!file_filters.contains(&d.path().file_name().unwrap())
                    && !path_filters.contains(&d.path().parent().unwrap()))
        })
        .filter_map(|e| e.ok())
    {
        writeln!(output, "{}", entry.path().display()).unwrap();
    }
}

fn find(path: &str, pattern: &str) {
    let regexpattern = Regex::new(pattern).unwrap();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered
        .lines()
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
            .help("regenerate cache at '~/.cache/rlocate-rs.cache' using config options from /etc/locate-rs.conf")
            .short("c")
            .long("c"))
        .arg(Arg::with_name("search")
            .help("regex pattern to search for")
            .index(1)
            .conflicts_with("cache"))
        .get_matches();

    if let Some(pattern) = options.value_of("search") {
        find(&[home_dir().unwrap().to_str().unwrap(), "/.cache/locate-rs.cache"].join(""), pattern);
    }

    if options.is_present("cache") {
        let filters = cfg_file.trim().split('\n').collect::<Vec<_>>();
        cache(&[home_dir().unwrap().to_str().unwrap(), "/.cache/locate-rs.cache"].join(""), &filters);
    }
}
