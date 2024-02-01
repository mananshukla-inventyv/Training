use std::vec;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;

#[derive(Debug,Deserialize,Serialize)]
struct Cell{
    width:i16,
    height:i16,
    content:String,
    //line_spacing:f32,
    //padding:f32,
    
}
impl Cell {
    fn new()-> Cell{
        Cell{
            content:"Hello World".to_string(),
            width:15,
            height:0
        }
    }
}

#[derive(Debug,Serialize)]
struct Row{
    total_cells:Vec<Cell>,
    width:i16,
    height:i16,  
    
}
impl Row {
    fn new(cell:Vec<Cell>)-> Row{
        Row{
            total_cells:cell,
            width:15,
            height:0
        }
    }
    fn calc_height(&mut self){
        for i in 0..self.total_cells.len(){
            if self.height<self.total_cells[i].height{
                self.height=self.total_cells[i].height;
            }           
        }     
    }
}

#[derive(Debug,Serialize)]
struct Table{
    total_rows:Vec<Row>,
    width:i16,
    height:i16,
    //table_border_all:f32,
    
}
impl Table {
    fn new(row:Vec<Row>)-> Table{
        Table{
            total_rows:row,
            width:0,
            height:0
        }
    }    
    fn calc_height(&mut self){
        for i in 0..self.total_rows.len(){
                self.height+=self.total_rows[i].height;
                self.width=60
        }    
    }
    fn change_data(&mut self,row_index:usize,cell_index:usize,new_height:i16,content:String){
        self.total_rows[row_index-1].total_cells[cell_index-1].height=new_height;
        self.total_rows[row_index-1].total_cells[cell_index-1].content=content;
        self.total_rows[row_index-1].calc_height();
        self.calc_height();
    }
}

fn main() {
    let c1 = Cell { width: 10, height: 15, content: "Apple".to_string() };
    let c2 = Cell { width: 10, height: 18, content: "Banana".to_string() };
    let c3 = Cell { width: 10, height: 12, content: "Orange".to_string() };
    let c4 = Cell { width: 10, height: 29, content: "Grape".to_string() };
    let c5 = Cell { width: 10, height: 16, content: "Watermelon".to_string() };

    let list_of_cell_1=vec![c1,c2,c3,c4,c5];
    let mut row1=Row::new(list_of_cell_1);
    row1.calc_height();
    
    let c6 = Cell { width: 10, height: 14, content: "Cherry".to_string() };
    let c7 = Cell { width: 10, height: 24, content: "Pineapple".to_string() };
    let c8: Cell = Cell::new();
    let c9 = Cell { width: 10, height: 19, content: "Strawberry".to_string() };
    let c10 = Cell { width: 10, height: 22, content: "Blueberry".to_string() };
    
    let list_of_cell_2=vec![c6,c7,c8,c9,c10];
    let mut row2=Row::new(list_of_cell_2);
    row2.calc_height();

    let c11 = Cell { width: 10, height: 11, content: "Lime".to_string() };
    let c12 = Cell { width: 10, height: 17, content: "Peach".to_string() };
    let c13 = Cell { width: 10, height: 26, content: "Kiwi".to_string() };
    let c14 = Cell { width: 10, height: 13, content: "Avocado".to_string() };
    let c15 = Cell { width: 10, height: 21, content: "Pear".to_string() };
    
    let list_of_cell_3=vec![c11,c12,c13,c14,c15]; 
    let mut row3=Row::new(list_of_cell_3);
    row3.calc_height();

    let c16 = Cell { width: 10, height: 15, content: "Melon".to_string() };
    let c17 = Cell { width: 10, height: 10, content: "Coconut".to_string() };
    let c18 = Cell { width: 10, height: 18, content: "Raspberry".to_string() };
    let c19 = Cell { width: 10, height: 23, content: "Blackberry".to_string() };
    let c20 = Cell { width: 10, height: 14, content: "Plum".to_string() };
    
    let list_of_cell_4=vec![c16,c17,c18,c19,c20];
    let mut row4=Row::new(list_of_cell_4);
    row4.calc_height();

    let c21 = Cell { width: 10, height: 20, content: "Grapefruit".to_string() };
    let c22 = Cell { width: 10, height: 16, content: "Fig".to_string() };
    let c23 = Cell { width: 10, height: 25, content: "Dragonfruit".to_string() };
    let c24 = Cell { width: 10, height: 12, content: "Mangosteen".to_string() };
    let c25 = Cell { width: 10, height: 19, content: "Papaya".to_string() };

    let list_of_cell_5=vec![c21,c22,c23,c24,c25];
    let mut row5=Row::new(list_of_cell_5);
    row5.calc_height();

    let list_of_rows:Vec<Row>=vec![row1,row2,row3,row4,row5];

    let mut table=Table::new(list_of_rows);
    table.calc_height();
    table.change_data(5, 3, 55,"abc".to_string());

    println!("{:#?}",table);

    
    let json_data=serde_json::to_string_pretty(&table);
    match json_data {
        Ok(data)=>{
            let _ = fs::write("json_data.json", data);
        }
        Err(err)=>println!("{}",err)
    }
}
