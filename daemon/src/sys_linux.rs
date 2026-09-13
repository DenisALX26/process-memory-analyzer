use pm_core::{ProcessInfo, ProcessState};
use std::fs;
use std::path::Path;

pub fn get_running_pids() -> Vec<u32> {
    let mut pids = Vec::new();
    let proc_dir = Path::new("/proc");
    if let Ok(entries) = fs::read_dir(proc_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(pid) = file_name.parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }
    pids
}

pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_contents = fs::read_to_string(&stat_path).ok()?;
    let start_name = stat_contents.find('(')?;
    let end_name = stat_contents.rfind(')')?;
    let name = stat_contents[start_name + 1..end_name].to_string();
    let rest = &stat_contents[end_name + 2..];
    let parts: Vec<&str> = rest.split_whitespace().collect();
    if parts.len() < 18 { return None; }
    let state_char = parts[0].chars().next().unwrap_or('U');
    let state = ProcessState::from(state_char);
    let threads: u32 = parts[17].parse().unwrap_or(1);
    let statm_path = format!("/proc/{}/statm", pid);
    let statm_contents = fs::read_to_string(&statm_path).ok()?;
    let statm_parts: Vec<&str> = statm_contents.split_whitespace().collect();
    let rss_pages: u64 = statm_parts.get(1)?.parse().unwrap_or(0);
    let memory_rss_kb = rss_pages * 4;
    Some(ProcessInfo { pid, name, state, threads, memory_rss_kb })
}