use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Styles {
    Fat,
    Neo,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Print {
    Under,
    Side,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Style {
    pub print: Print,
    pub style: Styles,
}

const CONFIG: &str = "config.json";

pub fn get_style() -> Style { //------------------------------------------------------------reads the current style from config.json
    let config_data = fs::read_to_string(CONFIG)
        .expect("Can't read config.json");
    serde_json::from_str(&config_data)
        .expect("Can't parse config.json")
}

pub fn set_style(new_style: Styles) { //----------------------------------------------------updates the style in config.json
    let config_data = fs::read_to_string(CONFIG)
        .expect("Can't read config.json");
    
    let mut style: Style = serde_json::from_str(&config_data)
        .expect("Can't parse config.json");
    
    style.style = new_style;
    
    let updated = serde_json::to_string_pretty(&style)
        .expect("Can't serialize");
    
    fs::write(CONFIG, updated)
        .expect("Can't write config.json");
}

pub fn set_side(new_print: Print) {
    let config_data = fs::read_to_string(CONFIG)
        .expect("Can't read config.json");
    
    let mut style: Style = serde_json::from_str(&config_data)
        .expect("Can't parse config.json");
    
    style.print = new_print;
    
    let updated = serde_json::to_string_pretty(&style)
        .expect("Can't serialize");
    
    fs::write(CONFIG, updated)
        .expect("Can't write config.json");
}