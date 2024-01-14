// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rfd::FileDialog;
use serde::Serialize;
use std::path::Path;
use std::process::Command;
use tauri::{AppHandle, Manager, Window};

mod tray;

#[derive(Serialize)]
pub struct Hoyo {
    root: String,
    launcher: String,
    game: String,
    exe: String,
}

#[derive(Serialize)]
pub struct Appoint {
    game: String,
    exe: String,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

// 强制切换窗口显示/隐藏
#[tauri::command]
fn change_window_status(_: AppHandle, window: Window, status: bool) {
    if status {
        tray::show_window(window);
    } else {
        window.hide().unwrap();
    }
}

// 检测路径有效性
#[tauri::command]
fn check_path_valid(dir: &str, file: &str) -> bool {
    if dir.len() == 0 || file.len() == 0 {
        return false;
    }
    let file_path = dir.to_owned() + "\\" + file;
    let res = Path::new(&file_path);
    return res.exists();
}

// 检测游戏进程状态
#[tauri::command]
fn check_game_status(process: &str) -> bool {
    let output = Command::new("cmd")
        .creation_flags(0x08000000) // 隐藏cmd窗口
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

// 检查文件路径有效性并调用cmd打开exe文件
#[tauri::command]
fn open_exe(dir: &str, file: &str) -> bool {
    let res = check_path_valid(dir, file);
    if !res {
        return false;
    }
    let _ = Command::new("cmd")
        .creation_flags(0x08000000) // 隐藏cmd窗口
        .arg("/c")
        .arg(format!("cd {} && start {}", dir, file))
        .spawn();
    return true;
}

// 手动指定游戏文件
#[tauri::command]
fn appoint_file() -> Appoint {
    let path = FileDialog::new()
        .set_title("手动指定游戏文件")
        .set_directory("/")
        .add_filter("exe", &["exe"])
        .pick_file();
    // 根路径
    let mut game = "".to_string();
    let mut exe = "".to_string();
    if let Some(dir) = path {
        if let Some(parent) = dir.parent() {
            game = parent.display().to_string();
        }
        if let Some(file) = dir.file_name() {
            exe = file.to_string_lossy().to_string()
        }
    };
    // 文件名
    Appoint { game, exe }
}

// 获取Hoyo游戏文件路径
#[tauri::command]
fn pick_folder(key: &str, title: &str) -> Hoyo {
    let mut root = "".to_string();
    let mut launcher = "".to_string();
    let mut game = "".to_string();
    let mut exe = "".to_string();
    let mut long_title = "".to_string();
    for i in 0..3 {
        long_title = long_title + title;
        if i == 2 {
            long_title = long_title + "重要的事情说三遍";
        }
    }
    let path = FileDialog::new()
        .set_title(&long_title)
        .set_directory("/")
        .pick_folder();
    // 插入文件夹路径
    if let Some(dir) = path {
        root = dir.display().to_string();
    };
    // 判空
    if root.len() == 0 {
        return Hoyo {
            root,
            launcher,
            game,
            exe,
        };
    }
    launcher = "launcher.exe".to_string();
    // match判断
    match key {
        "bh3" => {
            game = root.to_string() + "\\Games";
            exe = "BH3.exe".to_string();
            Hoyo {
                root,
                launcher,
                game,
                exe,
            }
        }
        "ys" => {
            game = root.to_string() + "\\Genshin Impact Game";
            exe = "YuanShen.exe".to_string();
            Hoyo {
                root,
                launcher,
                game,
                exe,
            }
        }
        "star" => {
            game = root.to_string() + "\\Game";
            exe = "StarRail.exe".to_string();
            Hoyo {
                root,
                launcher,
                game,
                exe,
            }
        }
        _ => Hoyo {
            root,
            launcher,
            game,
            exe,
        },
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
            tray::show_window(app.get_window("main").unwrap());
        }))
        .invoke_handler(tauri::generate_handler![
            pick_folder,
            open_exe,
            check_path_valid,
            check_game_status,
            appoint_file,
            change_window_status
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
