use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::{os::windows::process::CommandExt, path::Path};
use tauri::{Manager, Window};

use crate::tray;

const LAUNCHER_FILE_NAME: &str = "launcher.exe";

const CONFIG_FILE_NAME: &str = "config.ini";

#[derive(Serialize)]
pub struct FileProp {
    pub file: String,
    pub path: String,
}

impl FileProp {
    /** 拼接真实文件路径 */
    fn format_real_path(&self) -> String {
        format!("{}\\{}", self.path, self.file)
    }

    pub fn check_path_valid(&self) -> bool {
        if self.path.len() == 0 || self.file.len() == 0 {
            return false;
        }
        let file_path = self.format_real_path();
        let file = Path::new(&file_path);
        return file.exists();
    }

    pub fn open_exe(&self) -> bool {
        let is_exist = self.check_path_valid();
        if is_exist {
            Command::new("cmd")
                .creation_flags(0x08000000) // 隐藏cmd窗口
                .arg("/c")
                .arg(format!("cd {} && start {}", self.path, self.file))
                .spawn();
        }
        return is_exist;
    }

    pub fn read_local_version(&mut self) -> String {
        // modify filename -> config_name
        self.file = String::from(CONFIG_FILE_NAME);
        if !self.check_path_valid() {
            return String::new();
        }
        let real_config_path = self.format_real_path();
        return fs::read_to_string(real_config_path).unwrap();
    }
}

pub trait PathOperation {
    /** 改变窗口可见状态 */
    fn change_window_status(window: Window, status: bool);
    // /** 检测游戏进程状态 */
    fn check_game_status(process: &str) -> bool;
    // /** 检测本地游戏版本 */
    fn check_local_version(&self, exe_path: &str) -> String;
    // /** 检测路径有效性 */
    fn check_path_valid(&self, dir: &str, file: &str) -> bool;
    /** 拼接真实文件路径 */
    fn format_real_path(&self, dir: &str, exe: &str) -> String {
        format!("{}\\{}", dir, exe)
    }
    // /** 检测文件路径有效性并调用cmd打开exe文件 */
    fn open_exe(&self, dir: &str, file: &str) -> bool;
    /** 获取launcher启动器路径 */
    fn pick_launcher_folder() -> FileProp;
    /** 获取exe游戏文件路径 */
    fn specify_file_manually(&self, need_check_config: bool) -> FileProp;
}

#[derive(Serialize, Deserialize)]
pub enum CommandOption<'a> {
    AppointFile,
    ChangeWindowStatus { status: bool },
    CheckGameStatus { process: &'a str },
    CheckLocalVersion { exe_path: &'a str },
    CheckPathValid { dir: &'a str, file: &'a str },
    OpenExe { dir: &'a str, file: &'a str },
    PickFolder,
}

impl PathOperation for CommandOption<'_> {
    fn change_window_status(window: Window, status: bool) {
        if status {
            tray::show_window(window);
        } else {
            window.hide().unwrap();
        }
    }

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

    fn check_local_version(&self, exe_path: &str) -> String {
        if !self.check_path_valid(exe_path, CONFIG_FILE_NAME) {
            return String::new();
        }
        let real_config_path = self.format_real_path(exe_path, CONFIG_FILE_NAME);
        return fs::read_to_string(real_config_path).unwrap();
    }

    fn check_path_valid(&self, dir: &str, file: &str) -> bool {
        if dir.len() == 0 || file.len() == 0 {
            return false;
        }
        let file_path = self.format_real_path(dir, file);
        let file = Path::new(&file_path);
        return file.exists();
    }

    fn open_exe(&self, dir: &str, file: &str) -> bool {
        if !self.check_path_valid(dir, file) {
            return false;
        }
        let _ = Command::new("cmd")
            .creation_flags(0x08000000) // 隐藏cmd窗口
            .arg("/c")
            .arg(format!("cd {} && start {}", dir, file))
            .spawn();
        return true;
    }

    fn pick_launcher_folder() -> LauncherProp {
        let title = "请指定游戏启动器(launcher.exe)所在路径";

        let mut prop = LauncherProp {
            launcher_path: String::new(),
            launcher: String::new(),
        };

        let path = FileDialog::new()
            .set_title(&title.repeat(3))
            .set_directory("/")
            .pick_folder();

        // launcher文件夹路径
        if let Some(dir) = path {
            prop.launcher_path = dir.display().to_string();
        }
        if prop.launcher_path.len() > 0 {
            prop.launcher = String::from(LAUNCHER_FILE_NAME);
        }

        prop
    }

    fn specify_file_manually(&self, need_check_config: bool) -> FileProp {
        let title = if need_check_config {
            "指定游戏文件（严格模式：需要config.ini文件）"
        } else {
            "指定游戏文件（非严格模式）"
        };

        let suffix = "exe";

        let mut prop = FileProp {
            exe_path: String::new(),
            exe: String::new(),
        };

        let path = FileDialog::new()
            .set_title(title)
            .set_directory("/")
            .add_filter(suffix, &[suffix])
            .pick_file();

        if let Some(dir) = path {
            if let Some(parent) = dir.parent() {
                prop.exe_path = parent.display().to_string();
            }
            if let Some(file) = dir.file_name() {
                prop.exe = file.to_string_lossy().to_string();
            }
        };
        prop
    }
}
