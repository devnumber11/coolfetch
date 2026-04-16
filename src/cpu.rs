use sysinfo::System;

use crate::banners::Banners;
use crate::banner;
use crate::os::print_side_by_side;
use crate::out;

fn get_cpu_name() -> String { //just returning name
    let mut sys = System::new_all();
    sys.refresh_cpu();
    if let Some(cpu) = sys.cpus().first() {
        cpu.brand().to_string()
    } else {
        "\x1b[31mUnknown processor\x1b[0m".to_string()
    }
}
pub fn get_cpu_info() -> Vec<String> { //detailed information
    let mut sys = System::new_all();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();
    let logical_cores = sys.cpus().len();
    let total_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / logical_cores as f32;
    if let Some(cpu) = sys.cpus().first() {
        vec![
            format!("CPU: {}", cpu.brand()),
            format!("Frequency: {} MHz", cpu.frequency()),
            format!("L-cores: {}", logical_cores),
            format!("P-cores: {}", sys.physical_core_count().unwrap_or(0)),
            format!("CPU usage: {:.1}%", total_usage)
        ]
    } else {
        vec!["\x1b[31mUnknown processor\x1b[0m".to_string()]
    }
}
pub fn print_cpu(cp: Option<u8>, compact: bool) {
    let banner = match cp {
        Some(0) => banner(Banners::AMDEpyc),
        Some(1) => banner(Banners::AMDRyzen),
        Some(2) => banner(Banners::INTEL),
        None => {
            let name = get_cpu_name().to_lowercase();
            match name.as_str() {
                v if v.contains("epyc")  => banner(Banners::AMDEpyc),
                v if v.contains("ryzen") => banner(Banners::AMDRyzen),
                v if v.contains("intel") => banner(Banners::INTEL),
                _ => "Banner not found".to_string(),
            }
        }
        _ => { 
            out!("\x1b[31mUnknown processor\x1b[0m"); 
            return; 
        }
    };
    let info = if compact { vec![get_cpu_name()] } else { get_cpu_info() };
    print_side_by_side(&banner, info);
}