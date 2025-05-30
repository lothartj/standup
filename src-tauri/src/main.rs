// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::Manager;

static MONITOR_INTERVAL: u64 = 1000; // Check every second
static IDLE_THRESHOLD: u64 = 7200000; // 2 hours in milliseconds

#[derive(Clone, serde::Serialize)]
struct ScreenTimePayload {
    exceeded: bool,
}

#[tauri::command]
async fn start_monitoring(window: tauri::Window) {
    let is_monitoring = Arc::new(AtomicBool::new(true));
    let is_monitoring_clone = is_monitoring.clone();

    tauri::async_runtime::spawn(async move {
        let mut last_active = SystemTime::now();
        let mut notification_shown = false;

        while is_monitoring_clone.load(Ordering::SeqCst) {
            let current_time = SystemTime::now();
            let elapsed = current_time
                .duration_since(last_active)
                .unwrap_or(Duration::from_secs(0))
                .as_millis() as u64;

            if elapsed >= IDLE_THRESHOLD && !notification_shown {
                window
                    .emit(
                        "screen-time-exceeded",
                        ScreenTimePayload { exceeded: true },
                    )
                    .unwrap();
                notification_shown = true;
            } else if elapsed < IDLE_THRESHOLD && notification_shown {
                window
                    .emit(
                        "screen-time-exceeded",
                        ScreenTimePayload { exceeded: false },
                    )
                    .unwrap();
                notification_shown = false;
                last_active = current_time;
            }

            tokio::time::sleep(Duration::from_millis(MONITOR_INTERVAL)).await;
        }
    });
}

#[tauri::command]
fn stop_monitoring() {
    // Implementation for stopping the monitoring will be added later
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_monitoring, stop_monitoring])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
