mod sled_manager;
mod commands;

use commands::*;
use sled_manager::init_sled_manager;
use tauri::{Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(init_sled_manager())
        .invoke_handler(tauri::generate_handler![
            // Connection management
            create_connection,
            remove_connection,
            get_connections,
            get_connection,
            
            // Database operations
            get_trees,
            get_stats,
            create_tree,
            remove_tree,
            
            // Key-value operations
            get,
            set,
            remove,
            
            // Query operations
            range_query,
            prefix_query,
            
            // Import/Export operations
            import_data,
            export_data,
            import_from_path,
        ])
        .setup(|app| {
            // 在开发模式下打开开发者工具
            #[cfg(debug_assertions)]
            {
                println!("Setting up Tauri app...");
                let window = app.get_webview_window("main").unwrap();
                
                // 确保窗口可见
                window.show().unwrap();
                window.set_focus().unwrap();
                window.set_title("Sled Visualizer").unwrap();
                
                // 设置窗口位置
                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: 100, y: 100 })).unwrap();
                
                // 打开开发者工具
                window.open_devtools();
                
                // 添加一些日志输出
                println!("Tauri app setup completed");
                
                // 添加一个延迟，确保窗口完全加载
                std::thread::sleep(std::time::Duration::from_millis(1000));
                
                // 再次确保窗口可见和聚焦
                window.show().unwrap();
                window.set_focus().unwrap();
                window.center().unwrap();
                
                // 添加额外的日志，以便调试
                println!("Window setup completed successfully");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
