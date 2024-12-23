use crate::services::bing::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_img_list(number: u8) -> Vec<BingImage> {
    let res = get_cached_bing_image_res(number).await;
    if let Some(img_list) = res.ok() {
        img_list.images.into_iter().map(|mut i| {
            i.url = i.get_full_url();
            i
        }).collect()
    } else {
        vec![]
    }
}

#[tauri::command]
pub async fn open_url(url: String) {
    open::that(url).unwrap()
}