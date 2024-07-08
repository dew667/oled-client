// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde_json::json;
use std::{fmt::format, path::PathBuf};
use tauri::{App, Manager, State, Wry};
use tauri_plugin_store::{with_store, Store, StoreBuilder, StoreCollection};

#[derive(Debug)]
struct Text {
    text: String,
}

#[tauri::command]
fn greet(message: &str) -> String {
    format!("Hello, {}", message)
}

#[tauri::command]
fn save(ipAddress: &str, port: &str, app_handle: tauri::AppHandle) -> String {
    let stores = app_handle.state::<StoreCollection<Wry>>();
    let path = PathBuf::from("store.bin");

    let result = with_store(app_handle.clone(), stores, path, |store| {
        store.insert(
            "config".to_string(),
            json!({ "ip": ipAddress, "port": port }),
        )?;

        let value = store.get("config").expect("Failed to get value from store");
        println!("{}", value);
        store.save()?;
        Ok(value.clone())
    })
    .expect("with_store fail");
    format!("{}", result)
}

#[tauri::command]
fn read_config(app_handle: tauri::AppHandle) -> String {
    let stores = app_handle.state::<StoreCollection<Wry>>();
    let path = PathBuf::from("store.bin");

    let result = with_store(app_handle.clone(), stores, path, |store| {
        let value = store.get("config").expect("Failed to get value from store");
        println!("{}", value);
        Ok(value.clone())
    })
    .expect("with_store fail");

    println!("{}", result);
    format!("{}", result)
}

fn create_config_entry(app: &mut App) {
    println!("Tauri creating config entry");
    let stores = app.app_handle().state::<StoreCollection<Wry>>();
    let path = PathBuf::from("store.bin");

    with_store(app.app_handle().clone(), stores, path, |store| {
        let has_key = store.has("config".to_string());
        println!("has_key: {}", has_key);
        if (!has_key) {
            store.insert(
                "config".to_string(),
                json!({ "ip": "127.0.0.1", "port": "8080" }),
            )?;
        }
        let value = store.get("config").expect("Failed to get value from store");
        println!("{}", value);
        store.save()?;
        Ok(())
    });
}

fn init_store(app_handle: tauri::AppHandle) {
    // let stores = app_handle.state::<StoreCollection<Wry>>();
    // let path = PathBuf::from("store.bin");

    // let result = with_store(app_handle.clone(), stores, path, |store| {
    //     let list = vec!("text1", "text2", "text3");
    //     store.insert("list".to_string(), json!(list))?;

    //     let value = store.get("list").expect("Failed to get value from store");
    //     println!("{}", value);
    //     Ok(value.clone())
    // }).expect("with_store fail");

    // println!("{}", result);
}

#[tauri::command]
fn save_message(message: &str, app_handle: tauri::AppHandle) {}

#[tauri::command]
fn list_message(app_handle: tauri::AppHandle) -> String {
    format!("{}", "")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            create_config_entry(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            read_config,
            save,
            save_message,
            list_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
