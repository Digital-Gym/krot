// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusty_ytdl::{Video, VideoQuality, VideoOptions, VideoSearchOptions};


#[tauri::command]
async fn start_download(url: &str) -> Result<String, String>{
    match parse(url, VideoQuality::Lowest, VideoSearchOptions::VideoAudio).await{
        Ok(video) => match download(video, "test.mp4").await{
          Ok(_) => return Ok(format!("Success!")),
          Err(err) => Err(format!("{}", err)),
        },
        Err(err) => Err(format!("{}", err)),
      }
}

async fn parse(url:&str, quality: VideoQuality, filter: VideoSearchOptions) -> Result<Video, String>{
    let video_options = VideoOptions {quality, filter, ..Default::default()};
  
    match Video::new_with_options(url, video_options){
    Ok(x) => return Ok(x),
    Err(err) => return Err(err.to_string()),
    };
  }
  
  
async fn download(video: Video, str_path: &str) -> Result<bool, String>{
    let temp = format!(r"{}", str_path);
    let path = std::path::Path::new(temp.as_str());
    match video.download(path).await{
      Ok(_) => return Ok(true),
      Err(err) => return Err(err.to_string()),
    };
  }
  

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}