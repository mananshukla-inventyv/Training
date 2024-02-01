pub mod font_data;

use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;
use std::fmt::Write;
use self::font_data::calc_font_size;

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
    pub row_type: RowType,
}

impl Row {
    fn new(mut cells: Vec<Cell>, font_size: i32, row_type: RowType) -> Row {
        let mut max_height = 0.0;
        let mut row_width = 0.0;
        let mut row_height = 0.0;
        let padding = 2.0;

        for i in 0..cells.len() {
            let cell_width = cells[i].width - 2.0 * padding;
            let mut total_chars_width = 0.0;
            let mut total_chars_height = font_size as f64 + 2.0 * padding;
            let mut full_sentence = String::new();
            let line_spacing = 2.0;

            for each_char in cells[i].content.chars() {
                if total_chars_width > cell_width {
                    total_chars_width = 0.0;
                    full_sentence.write_char('\n').expect("error in writing char");
                    total_chars_height += font_size as f64 + line_spacing;
                }
                full_sentence.write_char(each_char).expect("error in writing char");
                total_chars_width += calc_font_size(&each_char, font_size);
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
            row_type,
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

pub fn table_hmap() {
            let final_data=serde_json::from_str(&fs::read_to_string("./data/data.json").unwrap()).unwrap();

            let (head_cell_vec,data_cell_vec)=create_and_insert_cells(final_data);
            
            // write_rows(cell_vec[0], cell_vec[1], final_data);
            // let new_row = Row::new(cell_vec, final_data.dataRows.fontSize, RowType::DataRows);
            // let each_cell_width =
            //     final_data.pageWidth as f64 / final_data.headerRow.title.len() as f64;
            // let mut head_vec = Vec::new();
            // for each_heading in final_data.headerRow.title {
            //     head_vec.push(Cell {
            //         height: 0.0,
            //         width: each_cell_width,
            //         content: each_heading,
            //     });
            // }

            // let new_header = Row::new(head_vec, final_data.headerRow.fontSize, RowType::HeadRows);
            // println!("{:#?}", new_header);

            let mut final_data: Vec<Row> = Vec::new();
            let r1=Row::new(head_cell_vec,18,RowType::HeadRows);
            let r2=Row::new(data_cell_vec,12,RowType::DataRows);
            final_data.push(r1); 
            final_data.push(r2);

            fs::write(
                "./data/table_hashmap.json",
                serde_json::to_string_pretty(&final_data).expect("msg"),
            )
            .expect("msg");
        }

pub fn create_and_insert_cells(final_data:TableData) ->(Vec<Cell>,Vec<Cell>) {
    

            let each_cell_width =
                final_data.pageWidth as f64 / final_data.headerRow.title.len() as f64;

            let mut cell_vec: Vec<Cell> = Vec::new();
            for each_row in final_data.dataRows.rows {
                for each_title in each_row {
                    cell_vec.push(Cell {
                        height: 0.0,
                        width: each_cell_width,
                        content: each_title,
                    });
                }
            }

            let mut head_vec = Vec::new();
            for each_heading in final_data.headerRow.title {
                head_vec.push(Cell {
                    height: 0.0,
                    width: each_cell_width,
                    content: each_heading,
                });
            }
            (head_vec,cell_vec)
         }

         



// pub fn write_rows(head_cell_vec:Vec<Cell>,data_cell_vec:Vec<Cell>, final_data:TableData){

//     let mut row_vec=Vec::new();
//     let r1=Row::new(head_cell_vec,final_data.headerRow.fontSize, RowType::HeadRows);
//     letRow::new(data_cell_vec,final_data.dataRows.fontSize, RowType::DataRows);

//     row_vec.push();
//     row_vec.push(data_cell_vec);
    
//     fs::write(
//         "./data/table_hashmap.json",
//         serde_json::to_string_pretty(&row_vec).expect("msg"),
//     ).expect("msg");
// }