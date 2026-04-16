use std::io::stdin;
use std::println as out;
use std::env;

use crate::banners::Banners;
use crate::pid::out_pids;
use crate::ram::print_ram;
use crate::fetch::print_all;

pub mod banners;
pub mod os;
pub mod style;
pub mod cpu;
pub mod shell;
pub mod pid;
pub mod ram;
pub mod fetch;

const VRS: &str = "0.0.1";

// Хелпер команда чтобы сделать код кратче
pub fn banner(x: Banners) -> String {
    x.get_banner()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help_urani();
        return;
    }

    let side = style::get_style().print == style::Print::Side;
    let style = style::get_style().style;

    match args[1].as_str() {
        "os" => os::print_os(false, side),
        "cpu" => cpu::print_cpu(None, false),
        "shell" => shell::print_st(),
        "tm" => {
            if args.len() < 3 {
                out_pids(None);
            } else {
                let target = args[2].clone();
                out_pids(Some(target));
            }
        }
        "ram" => print_ram(),
        "fetch" => print_all(),
        "banner" => {
            if args.len() < 3 {
                out!("\x1b[31mUsage:\x1b[0m banner <name>");
                return;
            }
            let banner = match args[2].to_lowercase().as_str() {
                "pwsh" => banner(Banners::Pwsh),
                "bash" => banner(Banners::Bash),
                "zsh" => banner(Banners::Zsh),
                "cmd" => banner(Banners::CMD),
                "arch" => banner(Banners::Arch),
                "ubuntu" => banner(Banners::Ubuntu),
                "win11" => banner(Banners::Win11),
                "win10" => banner(Banners::Win10),
                "win8" => banner(Banners::Win8),
                "win7" => banner(Banners::Win7),
                "apple" => banner(Banners::MacOS),
                "debian" | "dash" => banner(Banners::Debian), // У dash shell нету официального логотипа поэтому можно использовать debian
                "macos" => banner(Banners::MacOS),
                "intel" => banner(Banners::INTEL),
                "ryzen" => banner(Banners::AMDRyzen),
                "epyc" => banner(Banners::AMDEpyc),
                _ => "\x1b[31mUnknown banner\x1b[0m".to_string(),
            };
            out!("{}", banner);
        }
        "style" => {
            if args.len() < 3 {
                out!("Current style: \x1b[36m{style:#?}\x1b[0m");
                return;
            }
            let new_style = match args[2].to_lowercase().as_str() {
                "neo" => style::Styles::Neo,
                "fat" => style::Styles::Fat,
                _     => { out!("\x1b[31mUnknown style\x1b[0m"); return; }
            };
            if new_style == style::get_style().style {
                out!("\x1b[33mStyle is already set to {}\x1b[0m", args[2]);
                return;
            }
            style::set_style(new_style);
            out!("\x1b[32mStyle updated!\x1b[0m");
        }
        "bout" => {
            out!("\x1b[32mCoolfetch \x1b[0m{VRS}\n");
            out!("Language? [ru / en]:");
            let mut ответ = String::new();
            stdin().read_line(&mut ответ).expect("Can't read line");
            match ответ.as_str().trim() {
                "ru" => out!("Coolfetch это fetch для того чтобы сделать терминал красивее\nСсылка на github: https://github.com/devnumber11/coolfetch"),
                "en" => out!("Coolfetch is a fetch for making the terminal prettier\nLink to github: https://github.com/devnumber11/coolfetch"),
                _ => out!("Unsupported language")
            }
        }
        "side" => {
            if args.len() < 3 {
                let s = if side { "Side" } else { "Under" };
                out!("Current print side: \x1b[36m{s}\x1b[0m");
                return;
            }
            let new_print = match args[2].to_lowercase().as_str() {
                "side"  => style::Print::Side,
                "under" => style::Print::Under,
                _       => { out!("\x1b[31mUnknown print side\x1b[0m"); return; }
            };
            if new_print == style::get_style().print {
                out!("\x1b[33mPrint side is already set to {}\x1b[0m", args[2]);
                return;
            }
            style::set_side(new_print);
            out!("\x1b[32mPrint side updated!\x1b[0m");
        }
        _ => {
            out!("\x1b[31mUnknown command\x1b[0m");
            help_urani();
        }
    }
}

fn help_urani() {
    out!("\x1b[32mCoolfetch \x1b[0m{VRS}");
    out!("-\x1b[33m bout\x1b[0m    -- About the project");
    out!("-\x1b[33m os\x1b[0m      -- Information about your OS");
    out!("-\x1b[33m cpu\x1b[0m     -- Information about your CPU");
    out!("-\x1b[33m shell\x1b[0m   -- Information about your shell");
    out!("-\x1b[33m banner\x1b[0m  -- Show a banner by name");
    out!("-\x1b[33m style\x1b[0m   -- Get/set banner style (fat/neo)");
    out!("-\x1b[33m side\x1b[0m    -- Get/set print layout (side/under)");
    out!("-\x1b[33m tm\x1b[0m      -- Print active processes, you can also find target in this format: \x1b[33mtm\x1b[0m <target_process>\x1b[0m")
}
