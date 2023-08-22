use std::fs;
use std::path::PathBuf;

use lofty::{Accessor, AudioFile, PictureType, TaggedFileExt};
use walkdir::{DirEntry, WalkDir};

use crate::models::{Album, Artist, Metadata, Playlist, Track};
use crate::store::Store;
use crate::{create_cache_dir, Result};

const AUDIO_FILE_EXTS: [&str; 10] = [
    "m4a", "aac", "ape", "aif", "aiff", "aifc", "flac", "mp3", "ogg", "wav",
];

#[tauri::command]
pub async fn update_library(store: tauri::State<'_, Store>, search_paths: Vec<&str>) -> Result<()> {
    let mut files = vec![];
    for path in search_paths {
        files.append(&mut get_audio_files(path)?);
    }

    log::debug!("have {} audio files", files.len());

    let cache_dir = create_cache_dir()?;
    let prev_hashes = store.get_track_ids().await?;
    for file in &files {
        let c = md5::compute(file.path().to_string_lossy().as_bytes());
        let hash = format!("{:x}", c);
        if !prev_hashes.contains(&hash) {
            let track = extract_track(hash, &file, &cache_dir)?;
            store.update_genre(&track).await?;
            store.update_artist(&track).await?;
            store.update_album(&track).await?;
            store.add_track(track).await?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_artists(store: tauri::State<'_, Store>) -> Result<Vec<Artist>> {
    store.get_artists().await
}

#[tauri::command]
pub async fn get_playlists(store: tauri::State<'_, Store>) -> Result<Vec<Playlist>> {
    store.get_playlists().await
}

#[tauri::command]
pub async fn get_albums(
    store: tauri::State<'_, Store>,
    artist: Option<String>,
) -> Result<Vec<Album>> {
    if let Some(a) = artist {
        store.get_albums_from_artist(a).await
    } else {
        store.get_albums().await
    }
}

#[tauri::command]
pub async fn get_tracks(
    store: tauri::State<'_, Store>,
    album: Option<String>,
) -> Result<Vec<Track>> {
    if let Some(a) = album {
        store.get_tracks_from_album(a).await
    } else {
        store.get_tracks().await
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn none_if_empty(x: impl Into<String>) -> Option<String> {
    let s = x.into();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn process_genre(s: impl Into<String>) -> String {
    let g: String = s.into();
    let mut start_num = false;
    let mut num = String::new();
    for c in g.chars() {
        match c {
            '(' => start_num = true,
            ')' => start_num = false,
            x => {
                if start_num {
                    num.push(x)
                }
            }
        }
    }

    use lofty::id3::v1::GENRES;
    num.parse::<usize>()
        .ok()
        .and_then(|i| GENRES.get(i))
        .map(|x| x.to_string())
        .unwrap_or(g)
}

fn extract_track(hash: String, file: &DirEntry, cache_dir: &PathBuf) -> Result<Track> {
    let tag_file = lofty::read_from_path(file.path())?;
    if let Some(tag) = tag_file.primary_tag().or(tag_file.first_tag()) {
        let song_artist = tag.artist().and_then(none_if_empty);
        let album_artist = tag
            .get_string(&lofty::ItemKey::AlbumArtist)
            .and_then(none_if_empty)
            .map(|x| x.into());

        let mut artwork_path = None;
        if let Some(cover_art) = tag
            .get_picture_type(PictureType::CoverFront)
            .or(tag.get_picture_type(PictureType::Other))
        {
            let path = cache_dir.join(&hash);
            fs::write(&path, cover_art.data())?;
            artwork_path = Some(path);
        }

        let track = Track {
            id: hash,
            metadata: Metadata {
                title: tag
                    .title()
                    .and_then(none_if_empty)
                    .unwrap_or(file.path().file_name().unwrap().to_string_lossy().into()),
                artist: album_artist
                    .clone()
                    .or(song_artist.clone())
                    .unwrap_or_default(),
                song_artist: album_artist
                    .and_then(|a| song_artist.and_then(|s| if s == a { None } else { Some(s) })),
                album: tag.album().and_then(none_if_empty).unwrap_or_default(),
                genre: tag.genre().and_then(none_if_empty).map(process_genre),
                cd_number: tag.disk(),
                track_number: tag.track(),
                year: tag.year(),
                artwork_path,
            },
            duration: tag_file.properties().duration().as_secs() as u32,
            path: file.path().into(),
        };
        Ok(track)
    } else {
        let track = Track {
            id: hash,
            metadata: Metadata {
                title: file.path().file_name().unwrap().to_string_lossy().into(),
                ..Default::default()
            },
            duration: tag_file.properties().duration().as_secs() as u32,
            path: file.path().into(),
        };
        Ok(track)
    }
}

fn get_audio_files(path: &str) -> Result<Vec<DirEntry>> {
    let walker = WalkDir::new(path).into_iter();
    let mut files = vec![];
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let file = entry?;
        if file.file_type().is_file() {
            if let Some(ext) = file.path().extension().and_then(|s| s.to_str()) {
                if AUDIO_FILE_EXTS.contains(&&*ext.to_lowercase()) {
                    files.push(file);
                }
            }
        }
    }
    Ok(files)
}
