use tauri::{Builder, Manager};
use crate::handlers::{connect, request}; 

mod handlers;

#[tauri::command]
fn fetch_data(url: String) -> String {
  request(url).unwrap() 
}

fn main() {
  let app = Builder::default()
    .setup(|app| {
      let window = app.new_window("API Client").unwrap();
      connect(window);
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
