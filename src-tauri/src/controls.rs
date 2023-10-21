use souvlaki::MediaControlEvent;
use souvlaki::{MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig};
use std::time::Duration;
use tokio::sync::Mutex;

use crate::models;
use crate::Result;

pub fn init_controls<T: tauri::Runtime>(app: &tauri::App<T>) -> Result<()> {
    use tauri::Manager;
    let hwnd = {
        use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
        let w = app.get_window("main").expect("main window does not exist");
        match w.raw_window_handle() {
            RawWindowHandle::Win32(f) => Some(f.hwnd),
            _ => None,
        }
    };

    let config = PlatformConfig {
        dbus_name: "tome",
        display_name: "Tome",
        hwnd,
    };

    let mut controls = MediaControls::new(config)?;

    let handle = app.handle();
    controls.attach(move |event| {
        log::debug!("MediaControls event: {:?}", event);
        match event {
            MediaControlEvent::Pause => handle.emit_all("pause", ()).unwrap(),
            MediaControlEvent::Play => handle.emit_all("play", ()).unwrap(),
            _ => {}
        }
    })?;

    app.manage(Mutex::new(controls));
    Ok(())
}

#[tauri::command]
pub async fn set_playback(
    controls: tauri::State<'_, Mutex<MediaControls>>,
    progress: Option<f32>,
    playing: bool,
) -> Result<()> {
    let progress = progress.map(|p| MediaPosition(Duration::from_secs_f32(p)));
    let playback = if playing {
        MediaPlayback::Playing { progress }
    } else if progress.is_some() {
        MediaPlayback::Paused { progress }
    } else {
        MediaPlayback::Stopped
    };
    log::debug!("Playback: {:?}", playback);

    controls.lock().await.set_playback(playback)?;
    Ok(())
}

#[tauri::command]
pub async fn set_metadata(
    controls: tauri::State<'_, Mutex<MediaControls>>,
    track: Option<models::Track>,
) -> Result<()> {
    let cover_url = track.as_ref().and_then(|t| {
        t.metadata
            .artwork_path
            .as_ref()
            .and_then(|path| path.to_str())
            .map(|path| format!("file://{}", path))
    });

    let metadata = track
        .as_ref()
        .map(|t| MediaMetadata {
            title: Some(&t.metadata.title),
            album: Some(&t.metadata.album),
            artist: Some(&t.metadata.artist),
            cover_url: cover_url.as_deref(),
            duration: Some(Duration::from_secs(t.duration as u64)),
        })
        .unwrap_or_default();

    controls.lock().await.set_metadata(metadata)?;
    Ok(())
}
