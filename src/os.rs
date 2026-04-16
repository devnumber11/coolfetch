use crate::banners::Banners;
use crate::out;
use crate::banner;

use std;
use sysinfo::System;

pub fn get_os(compact: bool) ->String {
    let os = std::env::consts::OS; //-------------------------------------------------------------windows, linux, macos
    let /*mut*/ os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string()); //----------windows 10, windows 11, ubuntu 20.04, etc
    /*os_version = "10".to_string();*/
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string()); //--10.0.19044, 5.4.0-80-generic, etc
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string()); //------------DESKTOP-12345, etc
    let banner = match os.to_lowercase() {
        v if v.contains("windows") => {
            match os_version.as_str() {
                v if v.contains("7") => banner(Banners::Win7),
                v if v.contains("8") => banner(Banners::Win8),
                v if v.contains("10") => banner(Banners::Win10),
                v if v.contains("11") => banner(Banners::Win11),
                _ => format!("Banner not found")
            }
        }
        v if v.contains("macos") => banner(Banners::MacOS),
        _ => format!("Banner not found")
    };
    if compact {
        format!("\n{banner}\nOS: {os}\nVersion: {os_version}\nKernel: {kernel_version}")
    } else {
        format!("\n{banner}\nOS: {os}\nVersion: {os_version}\nKernel: {kernel_version}\nHost Name: {host_name}")

    }
}
pub fn get_os_lines() -> Vec<String> { // Онли текст
    vec![
        format!("OS: {}", std::env::consts::OS),
        format!("Version: {}", sysinfo::System::os_version().unwrap_or_default()),
        format!("Kernel: {}", sysinfo::System::kernel_version().unwrap_or_default()),
        format!("Host name: {}", System::host_name().unwrap_or_else(|| "Unknown".to_string()))
    ]
}
pub fn get_banner() -> String {
    let os = std::env::consts::OS; //-------------------------------------------------------------windows, linux, macos
    let /*mut*/ os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    /*os_version = "10".to_string();*/ //for tests
    match os.to_lowercase().as_str() {
        v if v.contains("windows") => {
            match os_version.as_str() {
                v if v.contains("xp") => format!("You are really using Windows XP?\nHere's a banner of windows 7 for you:\n{}", banner(Banners::Win7)),
                v if v.contains("7") =>  banner(Banners::Win7),
                v if v.contains("8") => banner(Banners::Win8),
                v if v.contains("10") => banner(Banners::Win10),
                v if v.contains("11") => banner(Banners::Win11),
                _ => format!("Banner not found")
            }
        }
        v if v.contains("macos") => banner(Banners::MacOS),
        v if v.contains("linux") => {
            let distro = get_linux_distro();
            match distro.to_lowercase().as_str() {
                v if v.contains("arch") => banner(Banners::Arch),
                v if v.contains("ubuntu") => banner(Banners::Ubuntu),
                _ => "Banner not found".to_string()
            }
        }
        _ => format!("Banner not found")
    }
}
pub fn print_os(compact: bool, side: bool) {
    if side {
        let info = vec![
            format!("OS: {}", std::env::consts::OS),
            format!("Version: {}", System::os_version().unwrap_or_else(|| "Unknown".to_string())),
            format!("Kernel: {}", System::kernel_version().unwrap_or_else(|| "Unknown".to_string())),
            format!("Host Name: {}", System::host_name().unwrap_or_else(|| "Unknown".to_string())),
        ];
        let banner = get_banner();
        print_side_by_side(&banner, info);
        return;
    } else {
        out!("{}", get_os(compact));
    }
}
pub fn print_side_by_side(banner: &str, info: Vec<String>) {
    let banner_lines: Vec<&str> = banner.lines().collect();
    let max_lines = std::cmp::max(banner_lines.len(), info.len());
    let width = 40;

    for i in 0..max_lines {
        let banner_part = banner_lines.get(i).unwrap_or(&"");
        let fignya = String::new();
        let info_part = info.get(i).unwrap_or(&fignya);

        let visible_len = strip_ansi(banner_part).chars().count();
        let padding = if visible_len < width { width - visible_len } else { 0 };

        out!("{}{} {}", banner_part, " ".repeat(padding), info_part);
    }
}
fn get_linux_distro() -> String {
    let os_release = std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default();
    for line in os_release.lines() {
        if line.starts_with("ID=") {
            return line.replace("ID=", "").replace('"', "").trim().to_string();
        }
    }
    "unknown".to_string()
}
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c2 in chars.by_ref() {
                if c2 == 'm' { break; }
            }
        } else {
            result.push(c);
        }
    }
    result
}
pub fn print_any_os(os: String) {
    let info = vec![
        format!("OS: {os}"),
        format!("Host Name: {}", System::host_name().unwrap_or_else(|| "Unknown".to_string())),
    ];
    let banner = get_any_banner(os);
    print_side_by_side(&banner, info);
}
fn get_any_banner(os: String) -> String {
    match os.to_lowercase().as_str() {
        v if v.contains("windows") =>  banner(Banners::Win7),
        v if v.contains("macos") => banner(Banners::MacOS),
        v if v.contains("arch") => banner(Banners::Arch),
        v if v.contains("ubuntu") => banner(Banners::Ubuntu),
        _ => format!("Banner not found")
    }
}