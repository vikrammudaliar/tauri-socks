use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

#[derive(Default)]
struct AppState {
    process_handle: Arc<Mutex<Option<std::process::Child>>>,
}

#[tauri::command]
fn start_socks_proxy(
    app_handle: AppHandle,
    state: tauri::State<AppState>,
) -> Result<String, String> {
    let mut handle = state.process_handle.lock().unwrap();

    if handle.is_some() {
        return Err("Proxy is already running.".into());
    }

    // Resolve the PEM file path using `resolve`
    let pem_path = app_handle
        .path()
        .resolve("keys/id_rsa.pem", BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve PEM file path: {}", e))?;

    if !pem_path.exists() {
        return Err(format!("PEM file not found at: {:?}", pem_path));
    }

    // SSH details
    let user = "ubuntu";
    let host = "52.59.90.242";
    let port = 22;
    let proxy_port = 1080;

    let _ssh_command = format!(
        "ssh -i {} -D {} {}@{} -p {} -N",
        pem_path.display(),
        proxy_port,
        user,
        host,
        port
    );

    match Command::new("ssh")
        .arg("-i")
        .arg(pem_path.display().to_string())
        .arg("-D")
        .arg(proxy_port.to_string())
        .arg(format!("{}@{}", user, host))
        .arg("-p")
        .arg(port.to_string())
        .arg("-N")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let stderr = child.stderr.take().unwrap();
            let stdout = child.stdout.take().unwrap();

            // Log SSH output
            thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                let stdout_reader = BufReader::new(stdout);
                let stderr_reader = BufReader::new(stderr);

                for line in stdout_reader.lines() {
                    if let Ok(log) = line {
                        println!("[INFO] {}", log);
                    }
                }

                for line in stderr_reader.lines() {
                    if let Ok(log) = line {
                        eprintln!("[ERROR] {}", log); // Log errors explicitly
                    }
                }
            });

            *handle = Some(child);
            Ok("Proxy started successfully.".into())
        }
        Err(err) => Err(format!("Failed to start proxy: {}", err)),
    }
}

#[tauri::command]
fn stop_socks_proxy(state: tauri::State<AppState>) -> Result<String, String> {
    let mut handle = state.process_handle.lock().unwrap();

    if let Some(mut child) = handle.take() {
        match child.kill() {
            Ok(_) => Ok("Proxy stopped successfully.".into()),
            Err(err) => Err(format!("Failed to stop proxy: {}", err)),
        }
    } else {
        Err("Proxy is not running.".into())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            start_socks_proxy,
            stop_socks_proxy
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
