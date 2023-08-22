use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Track {
    pub id: String,
    pub path: PathBuf,
    pub metadata: Metadata,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub song_artist: Option<String>,
    pub album: String,
    pub track_number: Option<u32>,
    pub cd_number: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
    pub artwork_path: Option<PathBuf>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: "No Name".to_string(),
            artist: "Unknown Artist".to_string(),
            song_artist: None,
            album: "Unknown Album".to_string(),
            track_number: None,
            cd_number: None,
            year: None,
            genre: None,
            artwork_path: None,
        }
    }
}

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Artist {
    pub name: String,
}

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub track_count: u32,
    pub artwork_path: Option<PathBuf>,
}

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Genre {
    pub name: String,
}

#[derive(Serialize, TS, Debug)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Playlist {
    pub title: String,
}
