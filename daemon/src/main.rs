use std::io::Write;
use std::os::unix::net::UnixListener;
use std::thread;
use std::time::Duration;
use std::fs;

#[cfg(target_os = "linux")]
mod sys_linux;
#[cfg(target_os = "linux")]
use sys_linux::{get_process_info, get_running_pids};

#[cfg(target_os = "windows")]
mod sys_windows;
#[cfg(target_os = "windows")]
use sys_windows::{get_process_info, get_running_pids};

fn main() {
    let socket_path = "/tmp/pm_analyzer.sock";
    let _ = fs::remove_file(socket_path);
    
    let listener = UnixListener::bind(socket_path).expect("Could not open the IPC socket");
    println!("Daemon started. Waiting for connections on {}...", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected!");
                
                thread::spawn(move || {
                    loop {
                        let pids = get_running_pids();
                        let mut processes = Vec::new();

                        for pid in pids {
                            if let Some(info) = get_process_info(pid) {
                                processes.push(info);
                            }
                        }
                        processes.sort_by(|a, b| b.memory_rss_kb.cmp(&a.memory_rss_kb));
                        
                        if let Ok(json) = serde_json::to_string(&processes) {
                            let payload = format!("{}\n", json);
                            
                            if stream.write_all(payload.as_bytes()).is_err() {
                                println!("Client disconnected.");
                                break;
                            }
                        }
                        thread::sleep(Duration::from_secs(2));
                    }
                });
            }
            Err(e) => eprintln!("IPC connection error: {}", e),
        }
    }
}