use std::time::Duration;

use sysinfo::System;
use tauri::Window;
use tokio::time;

const STATUS_STARTED: &str = "status-started";

const STATUS_CLOSED: &str = "status-closed";

pub async fn monitor_process(window: Window, process_name: &str) {
    let mut system = System::new();
    let mut old_status = false;

    loop {
        system.refresh_processes();

        let new_status = system
            .processes()
            .iter()
            .any(|(_, process)| process.name() == process_name);
        if new_status && !old_status {
            old_status = true;
            window.emit(STATUS_STARTED, process_name).unwrap();
        } else if !new_status && old_status {
            window.emit(STATUS_CLOSED, process_name).unwrap();
            break;
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}
