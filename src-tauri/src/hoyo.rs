use rfd::FileDialog;
use serde::Serialize;
use std::fs;
use std::process::Command;
use std::{os::windows::process::CommandExt, path::Path};

const LAUNCHER_FILE_NAME: &str = "launcher.exe";

const CONFIG_FILE_NAME: &str = "config.ini";

const WINDOW_CMD_HIDE_CONSTANT: u32 = 0x08000000;

#[derive(Serialize)]
pub struct HoyoProp {
    pub file: String,
    pub path: String,
}

impl HoyoProp {
    pub fn new(path: &str, file: &str) -> HoyoProp {
        HoyoProp {
            path: String::from(path),
            file: String::from(file),
        }
    }

    /** 拼接真实文件路径 */
    fn format_real_path(&self) -> String {
        format!("{}\\{}", self.path, self.file)
    }

    pub fn check_path_valid(&self) -> bool {
        if self.path.len() == 0 || self.file.len() == 0 {
            return false;
        }
        let file_path = self.format_real_path();
        Path::new(&file_path).exists()
    }

    pub fn open_exe_file(&self) -> bool {
        let is_exist = self.check_path_valid();
        if is_exist {
            Command::new("cmd")
                .creation_flags(WINDOW_CMD_HIDE_CONSTANT) // 隐藏cmd窗口
                .arg("/c")
                .arg(format!("cd {} && start {}", self.path, self.file))
                .spawn()
                .is_ok()
        } else {
            false
        }
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

    pub fn specify_exe_file(&mut self, need_check_config: bool) {
        let title = if need_check_config {
            "指定游戏文件（严格模式：需要config.ini文件）"
        } else {
            "指定游戏文件（非严格模式）"
        };

        let suffix = "exe";

        let path = FileDialog::new()
            .set_title(title)
            .set_directory("/")
            .add_filter(suffix, &[suffix])
            .pick_file();

        if let Some(dir) = path {
            if let Some(parent) = dir.parent() {
                self.path = parent.display().to_string();
            }
            if let Some(file) = dir.file_name() {
                self.file = file.to_string_lossy().to_string();
            }
        };
    }

    pub fn specify_launcher_file(&mut self) {
        let title = "请指定游戏启动器(launcher.exe)所在路径";

        let path = FileDialog::new()
            .set_title(&title.repeat(3))
            .set_directory("/")
            .pick_folder();

        // launcher文件夹路径
        if let Some(dir) = path {
            self.path = dir.display().to_string();
        }
        if self.path.len() > 0 {
            self.file = String::from(LAUNCHER_FILE_NAME);
        }
    }
}
