mod relay;

use relay::{
    connect_relay, detect_relay_connected_at_port, disconnect_relay, list_ports, relay_blink,
    relay_get, relay_get_all, relay_set, relay_status,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(relay::RelayState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Relay
            connect_relay,
            detect_relay_connected_at_port,
            disconnect_relay,
            list_ports,
            relay_status,
            relay_get,
            relay_get_all,
            relay_set,
            relay_blink,
            // Default
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
