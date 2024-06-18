// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hoyo;
mod tray;

use hoyo::HoyoProp;
use std::{os::windows::process::CommandExt, process::Command};
use tauri::{Manager, Window};
use tray::{show_window, SingleInstancePayload, WINDOW_CMD_HIDE_CONSTANT};

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
fn check_game_status(process: &str) -> bool {
    let output = Command::new("cmd")
        .creation_flags(WINDOW_CMD_HIDE_CONSTANT) // 隐藏cmd窗口
        .arg("/c")
        .arg(format!("tasklist | findstr {}", process))
        .output()
        .expect("-1");
    let output_str = String::from_utf8_lossy(&output.stdout);
    match output_str.find(process) {
        Some(_) => return true,
        None => return false,
    };
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
            app.emit_all(
                "single-instance",
                SingleInstancePayload { args: argv, cwd },
            )
            .unwrap();
            tray::show_window(app.get_window("main").unwrap());
        }))
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
