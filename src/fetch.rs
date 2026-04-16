use crate::{
    cpu::get_cpu_info, 
    os::{self, get_os_lines, print_side_by_side}, 
    ram::{get_ram, get_ram_model}, 
    shell::{get_shell_info, get_terminal},
    style::{self, Print}
};

pub fn print_all() {
    let mut info_lines = Vec::new();
    info_lines.extend(get_os_lines()); 
    info_lines.push("".to_string());
    info_lines.extend(get_cpu_info());
    info_lines.push("".to_string());
    info_lines.push("RAM Modules:".to_string());
    for line in get_ram_model().lines() {
        info_lines.push(format!("  {}", line));
    }
    let ram_stats = get_ram();
    info_lines.push(format!("Usage: {} / {}", ram_stats[1], ram_stats[0]));
    info_lines.push("".to_string());
    info_lines.extend(get_shell_info());
    info_lines.push(format!("Terminal: {}", get_terminal()));

    let current_style = style::get_style();
    let banner = os::get_banner();

    match current_style.print {
        Print::Side => {
            print_side_by_side(&banner, info_lines);
        }
        Print::Under => {
            println!("{}", banner);
            for line in info_lines {
                println!("{}", line);
            }
        }
    }
}