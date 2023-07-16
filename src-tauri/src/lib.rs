use std::time::{SystemTime, Duration};

use bincode::{encode_to_vec, config, decode_from_slice};
use blucher_data::commands::Command;
use tauri::{Manager, AppHandle};
use tokio::{time::sleep, sync::{Mutex, mpsc}};
use tracing::info;

type AsyncProcInputTx = Mutex<mpsc::Sender<String>>;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn sent_message() {
    
    let message = Command::SetThrust { thrust: 1.0 };

    let data = encode_to_vec(message, config::standard()).unwrap();

    println!("data encoded to {} bytes", data.len());

    let ( decoded, _ ) = decode_from_slice::<Command, _>(&data, config::standard()).unwrap();

    match &decoded{
        Command::SetThrust { thrust } => {
            println!("Got SetThrust Command with thrust {}", thrust);
        },
        Command::Heartbeat => {
            println!("Got Heartbeat Command");
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    let (async_proc_output_tx, async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .manage(Mutex::new(async_proc_output_tx))
        .invoke_handler(tauri::generate_handler![on_command])
        .setup(|app| {

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                process_input(&app_handle).await
            });

            tauri::async_runtime::spawn(async move {
                process_output(async_proc_output_rx).await
            });

            Ok(())
        })
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![on_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_input_update<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    manager
        .emit_all("input_update", Payload { message: message.clone() })
        .unwrap();
    info!("sent to WebView: {message:#?}");
}

#[tauri::command]
async fn on_command(
    message: String,
    state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    info!(?message, "Received from WebView");

    let async_proc_output_tx = state.inner().lock().await;

    async_proc_output_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
    
}

async fn process_output(mut output_rx: mpsc::Receiver<String>) {
    while let Some(output) = output_rx.recv().await {
        info!("Sent command {} over socket", output);
    }
}

async fn process_input(manager: &AppHandle) {
    let start = SystemTime::now();

    loop
    {
        sleep(Duration::from_secs(1)).await;
        info!("input received over socket, sending hello to web view");

        let message = format!("Time: {}", start.elapsed().unwrap().as_secs());

        on_input_update(message, manager);
    }
}
