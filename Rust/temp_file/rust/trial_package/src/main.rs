use module_task::imp_modules::calc_font_size;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::{cell, fs};
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Serialize)]
pub enum RowType {
    DataRows,
    HeadRows
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    pub width: f64,
    pub height: f64,
    pub content: String,
    //line_spacing:f32,
    //padding:f32,
}
impl Cell {
    fn new() -> Cell {
        Cell {
            content: "Hello World".to_string(),
            width: 15.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub total_cells: Vec<Cell>,
    pub width: f64,
    pub height: f64,
    pub row_type:RowType,
    // pub font_size:i16,
}
impl Row {
    fn new(mut cells: Vec<Cell>, font_size: i32,row_type:RowType) -> Row {
        let mut max_height = 0.0;
        let mut row_width = 0.0;
        let mut row_height = 0.0;
        let padding = 2.0;
        for i in 0..cells.len() {

            let cell_width = cells[i].width - 2.0*padding;
            let mut total_chars_width = 0.0;

            // let cell_pading = padding*2.0;
            let mut total_chars_height = font_size as f64 + 2.0*padding;
            let mut full_sentence = String::new();
            let line_spacing = 2.0;
            for each_char in cells[i].content.chars() {
                // println!(" chars_total_w {}", total_chars_width);
                // println!("Cell_width {}", cell_width);
                if total_chars_width > cell_width {
                    // cell_width += fix_cell_width ;
                    total_chars_width=0.0;
                    full_sentence
                    .write_char('\n')
                    .expect("error in writing char");
                total_chars_height += font_size as f64 + line_spacing;
            }
                full_sentence
                .write_char(each_char)
                .expect("error in writing char");
                total_chars_width += calc_font_size(&each_char, font_size);
            
            

            
                    
                if max_height < total_chars_height {
                    max_height = total_chars_height;
                }
                    // let each_cell_height= if each_cell_height<total_chars_height{total_chars_height} else {each_cell_height};
                
            }
            
            cells[i].content = full_sentence;
            cells[i].height = total_chars_height;
            row_width = cells[0].width * cells.len() as f64;
            row_height = max_height;
        }

        Row {
            total_cells: cells,
            width: row_width,
            height: row_height,
            row_type
        }
    }

    fn calc_height(&mut self) {
        for i in 0..self.total_cells.len() {
            if self.height < self.total_cells[i].height {
                self.height = self.total_cells[i].height;
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub total_rows: Vec<Row>,
    pub width: f64,
    pub height: f64,
    //table_border_all:f32,
}
impl Table {
    fn new(row: Vec<Row>) -> Table {
        Table {
            total_rows: row,
            width: 0.0,
            height: 0.0,
        }
    }
    fn calc_height(&mut self) {
        for i in 0..self.total_rows.len() {
            self.height += self.total_rows[i].height;
        }
    }
    fn change_data(
        &mut self,
        row_index: usize,
        cell_index: usize,
        new_height: f64,
        content: String,
    ) {
        self.total_rows[row_index - 1].total_cells[cell_index - 1].height = new_height;
        self.total_rows[row_index - 1].total_cells[cell_index - 1].content = content;
        self.total_rows[row_index - 1].calc_height();
        self.calc_height();
    }
}

#[derive(Debug, Deserialize)]

pub struct TableData {
    headerRow: HeadRows,
    dataRows: DataRows,
    pageWidth: i32,
}

#[derive(Debug, Deserialize)]

pub struct HeadRows {
    fontSize: i32,
    title: Vec<String>,
}
#[derive(Debug, Deserialize)]

pub struct DataRows {
    fontSize: i32,
    rows: Vec<Vec<String>>,
}

pub fn main() {
    // let each_font_size_data=fs::read_to_string("./data/fonrData.json");

    match fs::read_to_string("./data/data.json") {
        Ok(data) => {
            let final_data: TableData = serde_json::from_str(&data).unwrap();
            // println!("{:?}", final_data);
            let each_cell_width =
                final_data.pageWidth as f64 / final_data.headerRow.title.len() as f64;
            // println!("{}", each_cell_width);
            // let mut table_width = 0.0;
            // let mut table_height: f64 = 0.0;
            // let mut row_vec = Vec::new();
            let mut cell_vec: Vec<Cell> = Vec::new();
            for each_row in final_data.dataRows.rows {
                // println!("{:?}", each_row);
                // let mut row_width: f64 = 0.0;
                // let mut row_height: f64 = 0.0;
                // let font_size = final_data.dataRows.fontSize;
                // let mut full_sentence = String::new();
                // let mut total_chars_height = 0.0;
                // for each_title in each_row {
                    //     let total_chars = each_title.chars();
                    //     let mut full_sentence = String::new();
                    //     let padding = 4.0;
                    //     let line_spacing = 2.0;
                    //     let mut total_chars_width = 0.0;
                    //     let mut cell_width = each_cell_width;
                    
                    //     for each_char in total_chars {
                        //         total_chars_width += (*each_font_size.get(&each_char.to_string()).unwrap())
                        //             * font_size as f64;
                        
                        //         full_sentence
                        //             .write_char(each_char)
                        //             .expect("error in writing char");
                        //         if total_chars_width > cell_width {
                            //             cell_width += each_cell_width;
                            //             full_sentence
                            //                 .write_char('\n')
                            //                 .expect("error in writing char");
                            
                            //             total_chars_height += font_size as f64;
                            //             total_chars_height += line_spacing;
                            //             // let each_cell_height= if each_cell_height<total_chars_height{total_chars_height} else {each_cell_height};
                            //         }
                            //     }
                            // total_chars_width -= padding;
                for each_title in each_row {
                    
                    cell_vec.push(Cell {
                        height: 0.0,
                        width: each_cell_width,
                        content: each_title,
                    });
                }
                // row_height = total_chars_height;
                // row_width = each_cell_width * 16.0;
            }
            let new_row = Row::new(cell_vec, final_data.dataRows.fontSize,RowType::DataRows);
            println!("{:#?}",new_row);
            let mut head_vec=Vec::new();
            for each_heading in final_data.headerRow.title{
                head_vec.push(Cell {
                    height: 0.0,
                    width: each_cell_width,
                    content: each_heading,
                });
            }
            let new_header=Row::new(head_vec, final_data.headerRow.fontSize,RowType::HeadRows);
            println!("{:#?}", new_header);

            let mut final_data:Vec<Row> =Vec::new();
            final_data.push(new_header);
            final_data.push(new_row);
            fs::write("./data/table_hashmap.json", serde_json::to_string_pretty(&final_data).expect("msg")).expect("msg");
            // let mut r = Row {
            //     total_cells: cell_vec,
            //     width: row_width,
            //     height: row_height,
            // };
            // row_vec.push(r);
            // // r.calc_height();

            // table_width = row_width;
            // table_height = row_height;
        }
        // let mut table = Table {
        //     total_rows: row_vec,
        //     width: table_width,
        //     height: table_height,
        // };
        // table.calc_height();
        // println!("{:?}", table);
        // fs::write(
        //     "./data/table_hashmap_data.json",
        //     serde_json::to_string_pretty(&table).unwrap()
        // )
        // .unwrap();
        _ => (),
    }
}
