use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref FILE: String = fs::read_to_string("./data/fonrData.json").unwrap();
    static ref EACH_FONT_SIZE: HashMap<String, f64> =  serde_json::from_str(&FILE).unwrap() ;
}
pub fn calc_font_size(font: &char, font_size: i32) -> f64 {
    EACH_FONT_SIZE.get(&font.to_string()).unwrap() * font_size as f64
}
