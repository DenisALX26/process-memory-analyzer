use pm_core::ProcessInfo;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;

fn main() {
    let socket_path = "/tmp/pm_analyzer.sock";
    
    let stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(_) => {
            eprintln!("Error: Could not connect. Check whether the daemon is running!");
            return;
        }
    };

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        if let Ok(response) = line {
            if let Ok(processes) = serde_json::from_str::<Vec<ProcessInfo>>(&response) {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                
                println!("{:<8} | {:<25} | {:<10} | {:<8} | {:<12}", "PID", "NAME", "STATE", "THREADS", "MEMORY (KB)");
                println!("{:-<73}", "-");

                for proc in processes.iter().take(15) {
                    println!(
                        "{:<8} | {:<25} | {:<10?} | {:<8} | {:<12}",
                        proc.pid,
                        if proc.name.len() > 25 { &proc.name[..25] } else { &proc.name },
                        proc.state,
                        proc.threads,
                        proc.memory_rss_kb
                    );
                }
            }
        } else {
            break;
        }
    }
}