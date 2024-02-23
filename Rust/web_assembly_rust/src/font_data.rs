use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    // #[derive(Debug)]
    // pub static ref FILE: String = fs::read_to_string("./fonrData.json").unwrap();
    #[derive(Debug)]
    pub static ref EACH_FONT_SIZE: HashMap<String, f32> = {
        let mut mp: HashMap<String, f32> = HashMap::new();
        mp.insert("0".to_string(), 0.5);
        mp.insert("1".to_string(), 0.5);
        mp.insert("2".to_string(), 0.5);
        mp.insert("3".to_string(), 0.5);
        mp.insert("4".to_string(), 0.5);
        mp.insert("5".to_string(), 0.5);
        mp.insert("6".to_string(), 0.5);
        mp.insert("7".to_string(), 0.5);
        mp.insert("8".to_string(), 0.5);
        mp.insert("9".to_string(), 0.5);
        mp.insert("".to_string(), 0.0);
        mp.insert(" ".to_string(), 0.25);
        mp.insert("!".to_string(), 0.333);
        mp.insert("\"".to_string(), 0.555);
        mp.insert("#".to_string(), 0.5);
        mp.insert("$".to_string(), 0.5);
        mp.insert("%".to_string(),1.0);
        mp.insert("&".to_string(),0.83300006);
        mp.insert("'".to_string(),0.27800003);
        mp.insert("(".to_string(),0.333);
        mp.insert(")".to_string(),0.333);
        mp.insert( "*".to_string(), 0.5);
        mp.insert("+".to_string(),0.57000005);
        mp.insert(":".to_string(),0.25);
        mp.insert("-".to_string(),0.333);
        mp.insert(".".to_string(),0.25);
        mp.insert("/".to_string(),0.27800003);
        mp.insert(",".to_string(),0.333);
        mp.insert(";".to_string(),0.333);
        mp.insert("<".to_string(),0.57000005);
        mp.insert("=".to_string(),0.57000005);
        mp.insert(">".to_string(),0.57000005);
        mp.insert("?".to_string(),0.5);
        mp.insert("@".to_string(),0.93000007);
        mp.insert("A".to_string(),0.72200006);
        mp.insert("B".to_string(),0.66700006);
        mp.insert("C".to_string(),0.72200006);
        mp.insert("D".to_string(),0.72200006);
        mp.insert("E".to_string(),0.66700006);
        mp.insert("F".to_string(),0.611);
        mp.insert("G".to_string(),0.77800006);
        mp.insert("H".to_string(),0.77800006);
        mp.insert("I".to_string(),0.38900003);
        mp.insert("J".to_string(),0.5);
        mp.insert("K".to_string(),0.77800006);
        mp.insert("L".to_string(),0.66700006);
        mp.insert("M".to_string(),0.94400007);
        mp.insert("N".to_string(),0.72200006);
        mp.insert("O".to_string(),0.77800006);
        mp.insert("P".to_string(),0.611);
        mp.insert("Q".to_string(),0.77800006);
        mp.insert("R".to_string(),0.72200006);
        mp.insert("S".to_string(),0.55600005);
        mp.insert("T".to_string(),0.66700006);
        mp.insert("U".to_string(),0.72200006);
        mp.insert("V".to_string(),0.72200006);
        mp.insert("W".to_string(),1.0);
        mp.insert("X".to_string(),0.72200006);
        mp.insert("Y".to_string(),0.72200006);
        mp.insert("Z".to_string(),0.66700006);
        mp.insert("[".to_string(),0.333);
        mp.insert("\\".to_string(), 0.27800003);
        mp.insert("]".to_string(), 0.333);
        mp.insert("^".to_string(), 0.58100003);
        mp.insert("_".to_string(), 0.5);
        mp.insert("`".to_string(), 0.333);
        mp.insert("a".to_string(), 0.5);
        mp.insert("b".to_string(), 0.55600005);
        mp.insert("c".to_string(), 0.44400004);
        mp.insert("d".to_string(), 0.55600005);
        mp.insert("e".to_string(), 0.44400004);
        mp.insert("f".to_string(), 0.333);
        mp.insert("g".to_string(), 0.5);
        mp.insert("h".to_string(), 0.55600005);
        mp.insert("i".to_string(), 0.27800003);
        mp.insert("j".to_string(), 0.333);
        mp.insert("k".to_string(), 0.55600005);
        mp.insert("l".to_string(), 0.27800003);
        mp.insert("m".to_string(), 0.83300006);
        mp.insert("n".to_string(), 0.55600005);
        mp.insert("o".to_string(), 0.5);
        mp.insert("p".to_string(), 0.55600005);
        mp.insert("q".to_string(), 0.55600005);
        mp.insert("r".to_string(), 0.44400004);
        mp.insert("s".to_string(), 0.38900003);
        mp.insert("t".to_string(), 0.333);
        mp.insert("u".to_string(), 0.55600005);
        mp.insert("v".to_string(), 0.5);
        mp.insert("w".to_string(), 0.72200006);
        mp.insert("x".to_string(), 0.5);
        mp.insert("y".to_string(), 0.5);
        mp.insert("z".to_string(), 0.44400004);
        mp.insert("{".to_string(), 0.39400002);
        mp.insert("|".to_string(), 0.22000001);
        mp.insert("}".to_string(), 0.39400002);
        mp.insert("~".to_string(), 0.52000004);
        mp
    };
}
pub fn calc_font_size(font: &char, font_size: usize) -> f32 {
    if let Some(item) = EACH_FONT_SIZE.get(&font.to_string()) {
        println!("Error!!!!");
    }
    EACH_FONT_SIZE.get(&font.to_string()).unwrap() * font_size as f32
}
