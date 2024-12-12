#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod controller;
mod model;
mod service;
use controller::compass_controller::{start_udp_service, AppState};
use model::compass::Compass;
use std::sync::Arc;
use tauri::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let bind_addr = "127.0.0.1:37003";
  let app_state = start_udp_service(bind_addr)
      .await
      .expect("Failed to start UDP service");

  tauri::Builder::default()
      .manage(app_state) // 전역 상태 전달
      .invoke_handler(tauri::generate_handler![get_latest_data]) // 명령 등록
      .run(tauri::generate_context!())?;

  Ok(())
}

#[tauri::command]
async fn get_latest_data(
  state: State<'_, Arc<AppState>>,
) -> Result<Option<Compass>, String> {
  let data_guard = state.data.read().await; // 데이터를 비동기적으로 읽기
  Ok(data_guard.clone()) // 데이터를 Result로 감싸서 반환
}
