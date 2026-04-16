use serde::Deserialize;
use std::collections::HashMap;

use crate::style;

#[derive(Debug, Deserialize)]
pub struct Banner {
    #[serde(flatten)]
    ban: HashMap<String, String>,
}
pub enum Banners {
    //Windows
    Win7, Win8, Win10, Win11,
    MacOS,
    Arch,
    Ubuntu,
    Debian,
    AMDEpyc,
    AMDRyzen,
    INTEL,
    Pwsh,
    CMD,
    Bash,
    Zsh,
    Dash
}
impl Banners {
    fn key(&self) -> (&str, &str) {
        match self {
            Banners::Win7 => ("win7_fat", "win7_neo"),
            Banners::Win8 => ("win8_fat", "win8_neo"),
            Banners::Win10 => ("win10_fat", "win10_neo"),
            Banners::Win11 => ("win11_fat", "win11_neo"),
            Banners::MacOS => ("apple_fat", "apple_neo"),
            Banners::Arch => ("arch_fat", "arch_neo"),
            Banners::Ubuntu => ("ubuntu_fat", "ubuntu_neo"),
            Banners::Debian | Banners::Dash => ("debian_fat", "debian_neo"),
            Banners::AMDEpyc => ("amd_epyc_fat", "amd_epyc_neo"),
            Banners::AMDRyzen => ("amd_ryzen_fat", "amd_ryzen_neo"),
            Banners::INTEL => ("intel_fat", "intel_neo"),
            Banners::Pwsh => ("pwsh_fat", "pwsh_neo"),
            Banners::CMD => ("cmd_fat", "cmd_neo"),
            Banners::Bash => ("bash_fat", "bash_neo"),
            Banners::Zsh => ("zsh_fat", "zsh_neo")
        }
    }
    pub fn get_banner(&self) -> String {
        let yml_data = include_str!("../banners.yml");
        let storage: Banner = serde_yaml::from_str(yml_data)
            .expect("Can't read banners.yml");
        let (fat, neo) = self.key();
        let key = if style::get_style().style == crate::style::Styles::Neo { neo } else { fat };
        storage.ban.get(key)
            .cloned()
            .unwrap_or_else(|| "Banner not found".to_string())
    }
}