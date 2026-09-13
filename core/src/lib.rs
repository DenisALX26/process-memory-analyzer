use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Unknown,
}

impl From<char> for ProcessState {
    fn from(state_char: char) -> Self {
        match state_char {
            'R' => ProcessState::Running,
            'S' | 'D' => ProcessState::Sleeping,
            'T' | 't' => ProcessState::Stopped,
            'Z' => ProcessState::Zombie,
            _ => ProcessState::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub threads: u32,
    pub memory_rss_kb: u64,
}