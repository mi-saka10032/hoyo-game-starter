// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hoyo;
mod monitor;
mod tray;

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use hoyo::HoyoProp;
use monitor::monitor_process;
use tauri::{Manager, State, Window};
use tray::{show_window, SingleInstancePayload};

/** 改变窗口可见状态 */
#[tauri::command]
fn change_window_status(window: Window, status: bool) {
    if status {
        show_window(window);
    } else {
        window.hide().unwrap();
    }
}

/** 检测游戏进程状态 */
#[tauri::command]
async fn check_game_status(
    window: Window,
    process: &str,
    state: State<'_, Arc<Mutex<HashSet<String>>>>,
) -> Result<(), String> {
    let process_name = process.to_string();
    let mut prefix_state = state.lock().map_err(|e| e.to_string())?;

    if prefix_state.insert(String::clone(&process_name)) {
        drop(prefix_state);
        let state_clone = Arc::clone(&state.inner());

        tokio::spawn(async move {
            monitor_process(window, &process_name).await;

            match state_clone.lock() {
                Ok(mut suffix_state) => {
                    suffix_state.remove(&process_name);
                    drop(suffix_state);
                }
                Err(_) => {}
            }
        });
    }

    Ok(())
}

/** 检测路径有效性 */
#[tauri::command]
fn check_path_valid(path: &str, file: &str) -> bool {
    HoyoProp::new(path, file).check_path_valid()
}

#[tauri::command]
fn open_exe_file(path: &str, file: &str) -> bool {
    HoyoProp::new(path, file).open_exe_file()
}

#[tauri::command]
fn pick_exe_file(path: &str, file: &str, need_check_config: bool) -> HoyoProp {
    let mut prop = HoyoProp::new(path, file);
    prop.specify_exe_file(need_check_config);
    return prop;
}

#[tauri::command]
fn pick_launcher_file(path: &str, file: &str) -> HoyoProp {
    let mut prop = HoyoProp::new(path, file);
    prop.specify_launcher_file();
    return prop;
}

#[tauri::command]
fn read_local_version(path: &str) -> String {
    HoyoProp::new(path, "").read_local_version()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", SingleInstancePayload { args: argv, cwd })
                .unwrap();
            tray::show_window(app.get_window("main").unwrap());
        }))
        .manage(Arc::new(Mutex::new(HashSet::<String>::new())))
        .invoke_handler(tauri::generate_handler![
            change_window_status,
            check_game_status,
            check_path_valid,
            open_exe_file,
            pick_exe_file,
            pick_launcher_file,
            read_local_version,
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
