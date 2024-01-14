use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window,
};

pub fn show_window(window: Window) {
    if !window.is_visible().unwrap() {
        window.show().unwrap();
    }
    if window.is_minimized().unwrap() {
        window.unminimize().unwrap();
    }
    if !window.is_focused().unwrap() {
        window.set_focus().unwrap();
    }
}

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "打开主窗口")) // 显示应用窗口
        // .add_item(CustomMenuItem::new("hide".to_string(), "隐藏主窗口")) // 显示应用窗口
        // .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("quit".to_string(), "退出")); // 退出

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    // 匹配点击事件
    match event {
        // 左键点击
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            show_window(window);
        }
        // 根据菜单 id 进行事件匹配
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                let _ = window.clone();
                std::process::exit(0);
            }
            "show" => {
                show_window(window);
            }
            /* "hide" => {
                window.hide().unwrap();
            } */
            _ => {}
        },
        _ => {}
    }
}
