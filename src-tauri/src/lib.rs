pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use commands::connection::{
    connect, delete_connection, disconnect, get_active_connections, get_connection_status,
    get_connections, save_connection, test_connection,
};
use commands::query::{execute_query, get_databases, get_schemas, get_table_schema, get_tables, get_tables_by_schema, switch_database};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            test_connection,
            save_connection,
            get_connections,
            delete_connection,
            connect,
            disconnect,
            get_connection_status,
            get_active_connections,
            execute_query,
            get_tables,
            get_tables_by_schema,
            get_table_schema,
            get_databases,
            get_schemas,
            switch_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
