pub mod font_data;
use self::font_data::calc_font_size;
use font_data::EACH_FONT_SIZE;
use serde::{Deserialize, Serialize};
use serde_json::{self, error};
use std::fmt::Write;
use wasm_bindgen::{convert::TryFromJsValue, prelude::*};
#[derive(Debug, Serialize, Deserialize)]
pub enum RowType {
    DataRows,
    HeadRows,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    pub width: f32,
    pub height: f32,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub total_cells: Vec<Cell>,
    pub width: f32,
    pub height: f32,
    pub row_type: RowType,
}

impl Row {
    fn new(mut cells: Vec<Cell>, font_size: usize, row_type: RowType) -> Row {
        let mut max_height = 0.0;
        let mut row_width = 0.0;
        let mut row_height = 0.0;
        let padding = 2.0;

        for i in 0..cells.len() {
            let cell_width = cells[i].width - 2.0 * padding;
            let mut total_chars_width = 0.0;
            let mut total_chars_height = font_size as f32 + 2.0 * padding;
            let mut full_sentence = String::new();
            let line_spacing = 2.0;

            for each_char in cells[i].content.chars() {
                if total_chars_width > cell_width {
                    total_chars_width = 0.0;
                    full_sentence
                        .write_char('\n')
                        .expect("error in writing char");
                    total_chars_height += font_size as f32 + line_spacing;
                }
                full_sentence
                    .write_char(each_char)
                    .expect("error in writing char");
                total_chars_width += calc_font_size(&each_char, font_size);
            }

            cells[i].content = full_sentence;
            cells[i].height = total_chars_height;
            row_width = cells[0].width * cells.len() as f32;
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub total_rows: Vec<Row>,
    pub width: f32,
    pub height: f32,
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
        new_height: f32,
        content: String,
    ) {
        self.total_rows[row_index - 1].total_cells[cell_index - 1].height = new_height;
        self.total_rows[row_index - 1].total_cells[cell_index - 1].content = content;
        self.total_rows[row_index - 1].calc_height();
        self.calc_height();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableData {
    headerRow: HeadRows,
    dataRows: DataRows,
    pageWidth: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeadRows {
    fontSize: usize,
    title: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataRows {
    fontSize: usize,
    rows: Vec<Vec<String>>,
}
#[wasm_bindgen]
pub fn table_hmap(data: String) -> Result<JsValue, JsValue> {
    // println!("{:?}",table_hmap(d.to_string()));
    // println!("{:?}",FILE);
    let t_data: Result<TableData, error::Error> = serde_json::from_str(&data);
    match t_data {
        Ok(table_data) => {
            // return Ok(JsValue::from_str(&serde_json::to_string(&table_data).unwrap()));
            // let (head_cell_vec, data_cell_vec) = create_and_insert_cells(table_data);
            let each_cell_width =
                table_data.pageWidth as f32 / table_data.headerRow.title.len() as f32;

            let mut cell_vec: Vec<Cell> = Vec::new();
            for each_row in table_data.dataRows.rows {
                for each_title in each_row {
                    cell_vec.push(Cell {
                        height: 0.0,
                        width: each_cell_width,
                        content: each_title,
                    });
                }
            }

            let mut head_vec = Vec::new();
            for each_heading in table_data.headerRow.title {
                head_vec.push(Cell {
                    height: 0.0,
                    width: each_cell_width,
                    content: each_heading,
                });
            }

            let mut final_data: Vec<Row> = Vec::new();
            let r1 = Row::new(head_vec, 18, RowType::HeadRows);
            let r2 = Row::new(cell_vec, 12, RowType::DataRows);
            final_data.push(r1);
            final_data.push(r2);
            let t1 = Table::new(final_data);
            match serde_json::to_string(&t1) {
                Ok(data_to_be_returned) => Ok(JsValue::from(data_to_be_returned)),
                Err(err) => Err(format!("Err in writing back to js is {}", err).into()),
            }
        }
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    }
}

// pub fn create_and_insert_cells(final_data: TableData) -> (Vec<Cell>, Vec<Cell>) {

// }
