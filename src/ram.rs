use sysinfo::System;
use std::process::Command;

use crate::out;

fn get_windows_ram() -> String {
    let output = Command::new("powershell")
        .args([
            "-Command", 
            "Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, PartNumber, Speed | ConvertTo-Csv -NoTypeInformation"
        ])
        .output();
    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            let mut formatted_sticks = Vec::new();
            for line in result.lines().skip(1) {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim_matches('"')).collect();
                if parts.len() >= 3 {
                    formatted_sticks.push(format!(" - {}: {} | {} MHz", parts[0], parts[1], parts[2]));
                }
            }
            if formatted_sticks.is_empty() { "  Unknown Model".to_string() } else { formatted_sticks.join("\n") }
        },
        Err(_) => "  PowerShell not found".to_string(),
    }
}
fn get_linux_ram() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("dmidecode -t memory | grep -E 'Manufacturer|Part Number|Speed' | awk -F: '{print $2}' | sed 'N;N;s/\\n/ /g'")
        .output();

    match output {
        Ok(out) if !out.stdout.is_empty() => {
            let result = String::from_utf8_lossy(&out.stdout);
            result.lines().map(|l| format!(" - {}", l.trim())).collect::<Vec<_>>().join("\n")
        },
        _ => {
            " - Linux RAM (Use sudo for detailed model)".to_string()
        }
    }
}
pub fn get_ram_model() -> String {
    if cfg!(target_os = "windows") {
        get_windows_ram()
    } else if cfg!(target_os = "linux") {
        get_linux_ram()
    } else {
        " - Unsupported OS".to_string()
    }
}
pub fn get_ram() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let free = sys.free_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    vec![
        format!("{:.2} GB", total),
        format!("{:.2} GB", used),
        format!("{:.2} GB", free),
    ]
}
pub fn print_ram() {
    let models = get_ram_model();
    let stats = get_ram();

    out!("RAM stick");
    out!("{}", models);
    out!("\nTotal capacity: {}", stats[0]);
    out!("Currently used: {}", stats[1]);
    out!("Available free: {}", stats[2]);
}