use sysinfo::{System, Pid};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::out;

pub fn out_pids(target: Option<String>) {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut found_processes: HashMap<Pid, (String, PathBuf)> = HashMap::new();
    
    out!("Format:");
    out!("\x1b[38;5;21m PID\x1b[0m | Name | \x1b[33mPath\x1b[0m");

    for (pid, process) in sys.processes() {
        let name = process.name();
        let path_buf = process.exe().map(|p| p.to_path_buf()).unwrap_or_default();

        let path_str = if path_buf.as_os_str().is_empty() {
            "\x1b[90m<None>\x1b[0m".to_string()
        } else {
            format!("{}", path_buf.display())
        };

        out!("\x1b[36m{:<8}\x1b[0m | {:<25} | \x1b[33m{}\x1b[0m", pid, name, path_str);

        if let Some(ref target_name) = target {
            if name.contains(target_name) {
                found_processes.insert(*pid, (name.to_string(), path_buf));
            }
        }
    }
    if let Some(target_name) = target {
        out!("\n\x1b[1mSearch results for \"{}\":\x1b[0m", target_name);
        
        if !found_processes.is_empty() {
            for (pid, (name, path)) in &found_processes {
                let p_str = if path.as_os_str().is_empty() {
                    "<Access denied, or idk what>".to_string()
                } else {
                    format!("{}", path.display())
                };
                out!("\x1b[32m[FOUND]\x1b[0m PID: {} | Name: {} | Path: {}", pid, name, p_str);
            }
        } else {
            out!("\x1b[31m[NOT FOUND]\x1b[0m Target \"{}\" wasn't detected", target_name);
        }
    }
}