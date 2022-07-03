#![allow(unused)]

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;
use serde::{Serialize, Deserialize};

/// Control file downloading
#[derive(Parser, Debug)]
struct Cli {
    /// Path to Download List YAML File
    #[clap(parse(from_os_str), default_value="download_list.yaml")]
    download_list_filename: PathBuf,
    /// Specific show or movie to download
    #[clap(long)]
    matching: Option<String>
}

fn default_show_format_excludes() -> Vec<String> {
    return vec!["tar".to_string(), "tgz".to_string()];
}

fn default_show_filename_excludes() -> Vec<String> {
    return vec!["german".to_string()];
}

fn default_movie_filename_excludes() -> Vec<String> {
    return vec!["cam".to_string(), "german".to_string()];
}

#[derive(Serialize, Deserialize, Debug)]
struct TVShow {
    name: String,
    tmdbid: u64,
    seasons: Vec<u8>,
    start: String,

    #[serde(default)]
    include_formats: Vec<String>,

    #[serde(default = "default_show_format_excludes")]
    exclude_formats: Vec<String>,

    #[serde(default)]
    include_string: Vec<String>,
    #[serde(default = "default_show_filename_excludes")]
    exclude_string: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    name: String,
    tmdbid: u64,
    start: String,

    #[serde(default)]
    include_formats: Vec<String>,

    #[serde(default)]
    exclude_formats: Vec<String>,

    #[serde(default)]
    include_string: Vec<String>,
    #[serde(default = "default_movie_filename_excludes")]
    exclude_string: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DownloadList {
    shows: Vec<TVShow>,
    movies: Vec<Movie>
}

fn main() {
    let args = Cli::parse();
    println!("Args: {:#?}", args);

    let download_list_file = File::open(args.download_list_filename).expect("File not found!");
    let download_list: DownloadList = serde_yaml::from_reader(BufReader::new(download_list_file)).unwrap();
    println!("Download List: {:#?}", download_list);
}