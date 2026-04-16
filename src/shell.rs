use std::env;
use sysinfo::{Pid, System};

use crate::{banners::Banners, os::print_side_by_side};

const SHELL_ENV_VARS: &[&str] = &["SHELL", "ComSpec"];
const TERMINAL_ENV_VARS: &[(&str, &str)] = &[
    ("TERM_PROGRAM", ""),
    ("WT_SESSION", "Windows Terminal"),
    ("ALACRITTY_LOG", "Alacritty"),
    ("KITTY_WINDOW_ID", "Kitty"),
    ("TERM", ""),
];
const KNOWN_SHELLS: &[(&str, &str)] = &[
    ("pwsh", "PowerShell Core"),
    ("powershell", "PowerShell"),
    ("cmd", "cmd.exe"),
    ("bash", "bash"),
    ("zsh", "zsh"),
    ("dash", "dash"),
    ("fish", "fish"),
    ("nu", "Nushell"),
    ("elvish", "Elvish"),
];

fn get_process_name(sys: &System, pid: Pid) -> Option<String> {
    sys.process(pid).map(|p| p.name().to_string())
}
fn get_parent_pid(sys: &System, pid: Pid) -> Option<Pid> {
    sys.process(pid)?.parent()
}
fn resolve_shell_name(raw: &str) -> String {
    let lower = raw.to_lowercase();
    let filename = std::path::Path::new(&lower)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    KNOWN_SHELLS
        .iter()
        .find(|(key, _)| filename.contains(key))
        .map(|(_, name)| name.to_string())
        .unwrap_or(filename)
}

pub fn get_shell() -> String {
    if env::var("PSModulePath").is_ok() {
        return "PowerShell".to_string();
    }
    
    for var in SHELL_ENV_VARS {
        if let Ok(val) = env::var(var) {
            return resolve_shell_name(&val);
        }
    }
    
    let mut sys = System::new_all();
    sys.refresh_processes();
    let current = Pid::from(std::process::id() as usize);

    let mut pid = current;
    for _ in 0..5 {
        if let Some(ppid) = get_parent_pid(&sys, pid) {
            if let Some(name) = get_process_name(&sys, ppid) {
                let resolved = resolve_shell_name(&name);
                if !resolved.to_lowercase().contains("cargo") {
                    return resolved;
                }
                pid = ppid;
                continue;
            }
        }
        break;
    }
    "Unknown".to_string()
}

pub fn get_terminal() -> String {
    for (var, label) in TERMINAL_ENV_VARS {
        if env::var(var).is_ok() {
            return if label.is_empty() {
                env::var(var).unwrap()
            } else {
                label.to_string()
            };
        }
    }
    let mut sys = System::new_all();
    sys.refresh_processes();
    let current = Pid::from(std::process::id() as usize);

    get_parent_pid(&sys, current)
        .and_then(|shell_pid| get_parent_pid(&sys, shell_pid))
        .and_then(|term_pid| get_process_name(&sys, term_pid))
        .unwrap_or_else(|| "Unknown".to_string())
}
pub fn get_shell_info() -> Vec<String> {
    vec![
        format!("Shell:    {}", get_shell()),
        format!("Terminal: {}", get_terminal()),
    ]
}
pub fn print_st() {
    let shell = get_shell();
    let banner = [
        ("powershell", Banners::Pwsh),
        ("cmd",        Banners::CMD),
        ("zsh",        Banners::Zsh),
        ("bash",       Banners::Bash),
    ]
    .into_iter()
    .find(|(key, _)| shell.to_lowercase().contains(key))
    .map(|(_, b)| b.get_banner())
    .unwrap_or_else(|| format!("\x1b[31mUnsupported shell\x1b[0m: {shell}"));

    print_side_by_side(&banner, get_shell_info());
}