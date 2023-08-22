#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::path::PathBuf;

use dotenvy::dotenv;
use serde::{Serialize, Serializer};

mod controls;
mod library;
mod models;
mod store;
mod tray;

use store::Store;
use tauri::api::path::local_data_dir;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Controls error")]
    Controls(souvlaki::Error),
    #[error(transparent)]
    Tagging(#[from] lofty::LoftyError),
    #[error(transparent)]
    FileScan(#[from] walkdir::Error),
    #[error("Missing data directory")]
    MissingDataDir,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

impl From<souvlaki::Error> for Error {
    fn from(e: souvlaki::Error) -> Self {
        Self::Controls(e)
    }
}

type Result<T> = std::result::Result<T, Error>;

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn create_data_dir() -> Result<PathBuf> {
    let local_dir = local_data_dir().ok_or(Error::MissingDataDir)?;
    let data_path = local_dir.join("tome");
    fs::create_dir_all(&data_path)?;
    Ok(data_path)
}

fn create_cache_dir() -> Result<PathBuf> {
    let data_path = create_data_dir()?;
    let cache_path = data_path.join("cache");
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();
    let store = Store::new().await?;

    tauri::Builder::default()
        .setup(|app| {
            controls::init_controls(app)?;
            Ok(())
        })
        .system_tray(tray::new_tray())
        .on_system_tray_event(tray::on_event)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            controls::update_controls,
            library::update_library,
            library::get_artists,
            library::get_albums,
            library::get_playlists,
            library::get_tracks,
        ])
        .manage(store)
        .run(tauri::generate_context!())?;
    Ok(())
}
