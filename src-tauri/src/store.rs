use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

use crate::models::{Album, Artist, Metadata, Playlist, Track};
use crate::{create_data_dir, Result};

pub struct Store {
    db: SqlitePool,
}

impl Store {
    pub async fn new() -> Result<Self> {
        let data_path = create_data_dir()?;
        let db_path = data_path.join("tome.db");

        let db_url = if let Ok(path) = env::var("DATABASE_URL") {
            path
        } else {
            format!("sqlite://{}", db_path.to_string_lossy())
        };

        let options = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .map_err(sqlx::Error::from)?;

        Ok(Self { db: pool })
    }

    pub async fn remove_track_from_playlist(&self, track_id: &str, playlist: &str) -> Result<u64> {
        let entry = sqlx::query!(
            r#"DELETE FROM playlist_track WHERE playlist = ? AND track_id = ? RETURNING position as "position!""#,
            playlist,
            track_id,
        )
        .fetch_one(&self.db)
        .await?;

        let changes = sqlx::query!(
            "UPDATE playlist_track SET position = position - 1 WHERE playlist = ? AND position > ?",
            playlist,
            entry.position,
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        Ok(changes)
    }

    pub async fn add_track_to_playlist(
        &self,
        track_id: &str,
        playlist: &str,
        position: i64,
    ) -> Result<u64> {
        let changes = sqlx::query!(
            "UPDATE playlist_track SET position = position + 1 WHERE playlist = ? AND position >= ?",
            playlist,
            position,
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        sqlx::query!(
            "INSERT INTO playlist_track VALUES (?, ?, ?)",
            playlist,
            track_id,
            position,
        )
        .execute(&self.db)
        .await?;

        Ok(changes)
    }

    pub async fn delete_tracks(&self, ids: &Vec<String>) -> Result<u64> {
        for id in ids {
            let playlists = sqlx::query!(
                "SELECT * FROM playlist_track WHERE playlist_track.track_id = ?",
                id
            )
            .fetch_all(&self.db)
            .await?;
            for p in playlists {
                self.remove_track_from_playlist(id, &p.playlist).await?;
            }
        }
        let params = format!("?{}", ", ?".repeat(ids.len() - 1));
        let query_str = format!("DELETE FROM track WHERE id IN ( {} )", params);
        let mut query = sqlx::query(&query_str);
        for i in ids {
            query = query.bind(i);
        }
        let res = query.execute(&self.db).await?.rows_affected();
        Ok(res)
    }

    pub async fn clean(&self) -> Result<(u64, u64, u64, u64)> {
        panic!("don't use this");
        let res = sqlx::query!("DELETE FROM artist WHERE NOT EXISTS (SELECT 1 FROM track WHERE track.artist = artist.name)")
            .execute(&self.db)
            .await?
            .rows_affected();
        let res2 = sqlx::query!("DELETE FROM album WHERE NOT EXISTS (SELECT 1 FROM track WHERE track.album_id = album.id)")
            .execute(&self.db)
            .await?
            .rows_affected();
        let res3 = sqlx::query!("DELETE FROM genre WHERE NOT EXISTS (SELECT 1 FROM track WHERE track.genre = genre.name)")
            .execute(&self.db)
            .await?
            .rows_affected();
        let res4 = sqlx::query!("DELETE FROM playlist_track WHERE NOT EXISTS (SELECT 1 FROM track WHERE track.id = playlist_track.track_id)")
            .execute(&self.db)
            .await?
            .rows_affected();
        Ok((res, res2, res3, res4))
    }

    pub async fn get_track_ids(&self) -> Result<HashSet<String>> {
        let res = sqlx::query!("SELECT id FROM track")
            .fetch_all(&self.db)
            .await?;
        Ok(res.into_iter().map(|r| r.id).collect())
    }

    pub async fn get_album(&self, id: &str) -> Result<Option<Album>> {
        let album = sqlx::query!("SELECT * FROM album WHERE id = ?", id)
            .fetch_optional(&self.db)
            .await?;

        Ok(album.map(|a| Album {
            id: a.id,
            title: a.title,
            artist: a.artist,
            track_count: a.track_count as u32,
            artwork_path: a.artwork_path.map(|a| a.into()),
        }))
    }

    pub async fn get_albums_from_artist(&self, artist: String) -> Result<Vec<Album>> {
        let res = sqlx::query!("SELECT * FROM album WHERE artist = ?", artist)
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(|album| Album {
                id: album.id,
                title: album.title,
                artist: album.artist,
                track_count: album.track_count as u32,
                artwork_path: album.artwork_path.map(|p| p.into()),
            })
            .collect();
        Ok(res)
    }

    pub async fn get_albums(&self) -> Result<Vec<Album>> {
        let res = sqlx::query!("SELECT * FROM album")
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(|album| Album {
                id: album.id,
                title: album.title,
                artist: album.artist,
                track_count: album.track_count as u32,
                artwork_path: album.artwork_path.map(|p| p.into()),
            })
            .collect();
        Ok(res)
    }

    pub async fn get_tracks_from_album(&self, album_id: String) -> Result<Vec<Track>> {
        let album = match self.get_album(&album_id).await? {
            Some(a) => a,
            None => return Ok(Vec::new()),
        };

        let res = sqlx::query!("SELECT * FROM track WHERE album_id = ?", album_id)
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(move |track| Track {
                id: track.id,
                path: track.path.into(),
                duration: track.duration as u32,
                metadata: Metadata {
                    title: track.title,
                    artist: track.artist,
                    song_artist: track.song_artist,
                    album: album.title.clone(),
                    track_number: track.track_number.map(|n| n as u32),
                    cd_number: track.cd_number.map(|n| n as u32),
                    year: track.year.map(|n| n as u32),
                    genre: track.genre,
                    artwork_path: track.artwork_path.map(|p| p.into()),
                },
            })
            .collect();
        Ok(res)
    }

    pub async fn get_tracks(&self) -> Result<Vec<Track>> {
        let res = sqlx::query!("SELECT * FROM track")
            .fetch_all(&self.db)
            .await?
            .into_iter()
            .map(|track| Track {
                id: track.id,
                path: track.path.into(),
                duration: track.duration as u32,
                metadata: Metadata {
                    title: track.title,
                    artist: track.artist,
                    song_artist: track.song_artist,
                    album: track.album_id,
                    track_number: track.track_number.map(|n| n as u32),
                    cd_number: track.cd_number.map(|n| n as u32),
                    year: track.year.map(|n| n as u32),
                    genre: track.genre,
                    artwork_path: track.artwork_path.map(|p| p.into()),
                },
            })
            .collect();
        Ok(res)
    }

    pub async fn update_album(&self, track: &Track) -> Result<u64> {
        let unique = format!("{} - {}", track.metadata.artist, track.metadata.album);
        let album_id = format!("{:x}", md5::compute(unique));
        let existing_album = self.get_album(&album_id).await?;

        let album = if let Some(mut a) = existing_album {
            a.track_count += 1;
            if a.artwork_path.is_none() {
                a.artwork_path = get_artwork(track);
            }
            a
        } else {
            Album {
                id: album_id,
                title: track.metadata.album.clone(),
                artist: track.metadata.artist.clone(),
                track_count: 1,
                artwork_path: get_artwork(track),
            }
        };

        let artwork = album
            .artwork_path
            .map(|p| String::from(p.to_string_lossy()));

        let res = sqlx::query!(
            "REPLACE INTO album VALUES (?, ?, ?, ?, ?)",
            album.id,
            album.title,
            album.artist,
            album.track_count,
            artwork,
        )
        .execute(&self.db)
        .await?
        .rows_affected();

        Ok(res)
    }

    pub async fn update_artist(&self, track: &Track) -> Result<u64> {
        let res = sqlx::query!(
            "INSERT OR IGNORE INTO artist VALUES (?)",
            track.metadata.artist
        )
        .execute(&self.db)
        .await?
        .rows_affected();
        Ok(res)
    }

    pub async fn update_genre(&self, track: &Track) -> Result<u64> {
        let res = sqlx::query!(
            "INSERT OR IGNORE INTO genre VALUES (?)",
            track.metadata.genre
        )
        .execute(&self.db)
        .await?
        .rows_affected();
        Ok(res)
    }

    pub async fn add_track(&self, track: Track) -> Result<u64> {
        let unique = format!("{} - {}", track.metadata.artist, track.metadata.album);
        let album_id = format!("{:x}", md5::compute(unique));
        let path = track.path.to_string_lossy();
        let artwork = track
            .metadata
            .artwork_path
            .map(|p| String::from(p.to_string_lossy()));
        let res = sqlx::query!(
            "INSERT INTO track VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            track.id,
            path,
            track.metadata.title,
            track.duration,
            track.metadata.artist,
            album_id,
            track.metadata.genre,
            track.metadata.song_artist,
            track.metadata.track_number,
            track.metadata.cd_number,
            track.metadata.year,
            artwork,
        )
        .execute(&self.db)
        .await?
        .rows_affected();
        Ok(res)
    }

    pub async fn get_artists(&self) -> Result<Vec<Artist>> {
        let res = sqlx::query_as!(Artist, "SELECT * FROM artist")
            .fetch_all(&self.db)
            .await?;
        Ok(res)
    }

    pub async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        let res = sqlx::query_as!(Playlist, "SELECT * FROM playlist")
            .fetch_all(&self.db)
            .await?;
        Ok(res)
    }
}

fn get_artwork(track: &Track) -> Option<PathBuf> {
    let mut path = track.path.clone();
    path.pop();

    const ARTWORK_FILENAMES: [&str; 6] = [
        "cover.png",
        "artwork.png",
        "folder.png",
        "cover.jpg",
        "artwork.jpg",
        "folder.jpg",
    ];
    for filename in ARTWORK_FILENAMES {
        let file = path.join(filename);
        if file.is_file() {
            return Some(file);
        }
    }

    if let Some(1) = track.metadata.track_number {
        track.metadata.artwork_path.clone()
    } else {
        None
    }
}
