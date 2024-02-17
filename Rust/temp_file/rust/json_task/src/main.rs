use serde::{Deserialize, Serialize};
// use serde_json::Value;
use std::fs;


#[derive(Debug, Serialize, Deserialize)]
struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    marks: Vec<u16>,
    percent:f32,
    grade:String
}
fn main() {
    let stud_data=fs::read_to_string("StudentData.json").expect("Wrong Path");
    
    // let data:Value = serde_json::from_str(&stud_data).unwrap();
    let mut data:Vec<Student> = serde_json::from_str(&stud_data).expect("err");
    
    println!("{:#?}",data);

    for i in 0..data.len(){
        let (percent,grade)=calc_percent_grade(&data[i].marks);
        data[i].percent=percent;
        data[i].grade=grade;
    }
    let sr_data = serde_json::to_string_pretty(&data).unwrap();
    fs::write("final_data.json", &sr_data).expect("msg");
    
    // let d:Vec<Student>=for item in data.iter(){
    //     let marks:Vec<u16>=serde_json::from_value(item["marks"].clone()).expect("abc");
    //     let (percent,grade)=calc_percent_grade(&marks);
    //     Student{
    //         name:serde_json::from_value(item["name"].clone()).expect("Cant get data"),
    //         phone:serde_json::from_value(item["phone"].clone()).expect("Cant get data"),
    //         email:serde_json::from_value(item["email"].clone()).expect("Cant get data"),
    //         city:serde_json::from_value(item["city"].clone()).expect("Cant get data"),
    //         address:serde_json::from_value(item["address"].clone()).expect("Cant get data"),
    //         marks:serde_json::from_value(item["marks"].clone()).expect("Cant get data"),
    //         percent,
    //         grade,
    //     }.
    // };
    // println!({:?},d); b 

    // let final_data:Vec<Student>= data.iter().map(|item|{
    //     let marks:Vec<u16>=serde_json::from_value(item["marks"].clone()).expect("abc");
    //     let (percent,grade)=calc_percent_grade(&marks);

    //     Student{
    //         name:serde_json::from_value(&item["name"]).expect("Cant get data"),
    //         phone:serde_json::from_value(&item["phone"]).expect("Cant get data"),
    //         email:serde_json::from_value(&item["email"]).expect("Cant get data"),
    //         city:serde_json::from_value(&item["city"]).expect("Cant get data"),
    //         address:serde_json::from_value(&item["address"]).expect("Cant get data"),
    //         marks:serde_json::from_value(&item["marks"]).expect("Cant get data"),
    //         percent,
    //         grade,

    //     }

    
    // }).collect();
    
    // let final_data=serde_json::to_string_pretty(&final_data).expect("err");

    // fs::write("final_data.json",final_data).expect("err");
    // File::create("final_data.json").expect("err").write(final_data);
    // println!("{:?}",data[0]);
    // for items in &data{

    // }
    
}

fn calc_percent_grade(marks:&Vec<u16>)->(f32,String){
    let total:u16=marks.iter().sum(); 
    let percent=(total as f32)/marks.len() as f32;
    let grade= match percent as u16 {
        0..=20=>{"E".to_string()},
        21..=40=>{"D".to_string()},
        41..=60=>{"C".to_string()},
        61..=80=>{"B".to_string()},
        81..=100=>{"A".to_string()},
        _=>{panic!("invalid marks!")}
        
    };
    (percent,grade)  

 

}
