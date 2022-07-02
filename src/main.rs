#![allow(unused)]

use clap::Parser;
use serde::{Serialize, Deserialize};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // Domain/Area
    group: String,
    // Specific command from list
    command: String
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
        let yamlconfig =
"
shows: 
  - name: RuPaul's Drag Race All Stars
    tmdbid: 85723
    seasons: [6, 7]
    start: Whatever.Drag.Race.
    include_formats: [mp4, mkv]
    exclude_formats: [tgz, tar]
    exclude_string: [german]
  - name: Star Trek Strange New Worlds
    seasons: [1]
    tmdbid: 103516
    start: Star.Trek.Strange.New.Worlds.

movies:
  - name: Top Gun Maverick
    tmdbid: 361743
    start: Top.Gun.Maverick
    exclude_string: [cam, german]

";

    let download_list: DownloadList = serde_yaml::from_str(&yamlconfig).unwrap();
    // let args = Cli::parse();
    // println!("Commands: ");
    println!("{:#?}", download_list);
}