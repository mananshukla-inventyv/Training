use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref FILE: String = fs::read_to_string("./data/fonrData.json").unwrap();
    static ref EACH_FONT_SIZE: HashMap<String, f64> = { serde_json::from_str(&FILE).unwrap() };
}
pub fn calc_font_size(font: &char, font_size: i32) -> f64 {
    EACH_FONT_SIZE.get(&font.to_string()).unwrap() * font_size as f64
}
// use lazy_static::lazy_static;
// use std::collections::HashMap;
// lazy_static! {
//     static ref FONT_MAP: HashMap<char, f64> = {
//         let mut char_width_map = HashMap::new();
//         char_width_map.insert('0', 0.5);
//         char_width_map.insert('1', 0.5);
//         char_width_map.insert('2', 0.5);
//         char_width_map.insert('3', 0.5);
//         char_width_map.insert('4', 0.5);
//         char_width_map.insert('5', 0.5);
//         char_width_map.insert('6', 0.5);
//         char_width_map.insert('7', 0.5);
//         char_width_map.insert('8', 0.5);
//         char_width_map.insert('9', 0.5);
//         char_width_map.insert(' ', 0.25);
//         char_width_map.insert('!', 0.333);
//         char_width_map.insert('\"', 0.555);
//         char_width_map.insert('#', 0.5);
//         char_width_map.insert('$', 0.5);
//         char_width_map.insert('%', 1.0);
//         char_width_map.insert('&', 0.83300006);
//         char_width_map.insert('\'', 0.27800003);
//         char_width_map.insert('(', 0.333);
//         char_width_map.insert(')', 0.333);
//         char_width_map.insert('*', 0.5);
//         char_width_map.insert('+', 0.57000005);
//         char_width_map.insert(':', 0.25);
//         char_width_map.insert('-', 0.333);
//         char_width_map.insert('.', 0.25);
//         char_width_map.insert('/', 0.27800003);
//         char_width_map.insert(',', 0.333);
//         char_width_map.insert(';', 0.333);
//         char_width_map.insert('<', 0.57000005);
//         char_width_map.insert('=', 0.57000005);
//         char_width_map.insert('>', 0.57000005);
//         char_width_map.insert('?', 0.5);
//         char_width_map.insert('@', 0.93000007);
//         char_width_map.insert('A', 0.72200006);
//         char_width_map.insert('B', 0.66700006);
//         char_width_map.insert('C', 0.72200006);
//         char_width_map.insert('D', 0.72200006);
//         char_width_map.insert('E', 0.66700006);
//         char_width_map.insert('F', 0.611);
//         char_width_map.insert('G', 0.77800006);
//         char_width_map.insert('H', 0.77800006);
//         char_width_map.insert('I', 0.38900003);
//         char_width_map.insert('J', 0.5);
//         char_width_map.insert('K', 0.77800006);
//         char_width_map.insert('L', 0.66700006);
//         char_width_map.insert('M', 0.94400007);
//         char_width_map.insert('N', 0.72200006);
//         char_width_map.insert('O', 0.77800006);
//         char_width_map.insert('P', 0.611);
//         char_width_map.insert('Q', 0.77800006);
//         char_width_map.insert('R', 0.72200006);
//         char_width_map.insert('S', 0.55600005);
//         char_width_map.insert('T', 0.66700006);
//         char_width_map.insert('U', 0.72200006);
//         char_width_map.insert('V', 0.72200006);
//         char_width_map.insert('W', 1.0);
//         char_width_map.insert('X', 0.72200006);
//         char_width_map.insert('Y', 0.72200006);
//         char_width_map.insert('Z', 0.66700006);
//         char_width_map.insert('[', 0.333);
//         char_width_map.insert('\\', 0.27800003);
//         char_width_map.insert(']', 0.333);
//         char_width_map.insert('^', 0.58100003);
//         char_width_map.insert('_', 0.5);
//         char_width_map.insert('`', 0.333);
//         char_width_map.insert('a', 0.5);
//         char_width_map.insert('b', 0.55600005);
//         char_width_map.insert('c', 0.44400004);
//         char_width_map.insert('d', 0.55600005);
//         char_width_map.insert('e', 0.44400004);
//         char_width_map.insert('f', 0.333);
//         char_width_map.insert('g', 0.5);
//         char_width_map.insert('h', 0.55600005);
//         char_width_map.insert('i', 0.27800003);
//         char_width_map.insert('j', 0.333);
//         char_width_map.insert('k', 0.55600005);
//         char_width_map.insert('l', 0.27800003);
//         char_width_map.insert('m', 0.83300006);
//         char_width_map.insert('n', 0.55600005);
//         char_width_map.insert('o', 0.5);
//         char_width_map.insert('p', 0.55600005);
//         char_width_map.insert('q', 0.55600005);
//         char_width_map.insert('r', 0.44400004);
//         char_width_map.insert('s', 0.38900003);
//         char_width_map.insert('t', 0.333);
//         char_width_map.insert('u', 0.55600005);
//         char_width_map.insert('v', 0.5);
//         char_width_map.insert('w', 0.72200006);
//         char_width_map.insert('x', 0.5);
//         char_width_map.insert('y', 0.5);
//         char_width_map.insert('z', 0.44400004);
//         char_width_map.insert('{', 0.39400002);
//         char_width_map.insert('|', 0.22000001);
//         char_width_map.insert('}', 0.39400002);
//         char_width_map.insert('~', 0.52000004);
//         char_width_map
//     };
// }

// pub fn calc_font_size(c: char, font_size: f64) -> f64 {
//     let char_width = *FONT_MAP.get(&c).unwrap();
//     char_width * font_size
// }
