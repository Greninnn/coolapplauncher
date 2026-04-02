// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::sources::steam::get_image_for_id;
mod sources;
#[tokio::main]
async fn main() {
    println!("{:#?}", get_image_for_id("1384160").await.get(0));
    app_launcher_lib::run();
}
