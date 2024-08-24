use std::{clone, io::Empty};

use crate::database::container_controller::ContainerController;

const DEFAULT_SCALER: f64 = 0.2;

#[derive(Debug, Clone, Copy)]
enum ContainerCell {
    Empty,
    Edge,
    Plant{
        val: &'static str,
        is_root: bool,
        size: i64
    },
    Sensor
}

impl  ContainerCell {
    fn symbol(self) -> Option<char>{
        match self {
            ContainerCell::Empty => Some(' '),
            ContainerCell::Edge => Some('+'),
            ContainerCell::Plant { val, is_root, size } => Some(val.chars().nth(0).unwrap_or('x')),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerIMG {
    cells: Vec<Vec<ContainerCell>>,
    scaler: f64
}

impl ContainerIMG {
    pub fn null() -> Self {
        ContainerIMG {
            cells: Vec::new(),
            scaler: DEFAULT_SCALER
        }
    }

    pub fn new(container: &ContainerController, scaler: Option<f64>) -> Self {
        let mut container_img = ContainerIMG::null();
        match scaler {
            Some(val) => container_img.scaler = val,
            None => {}
        }

        let col_len = (container.width() * container_img.scaler) as usize;
        let row_len = (container.length() * container_img.scaler) as usize;
        container_img.cells.push(vec![ContainerCell::Edge; row_len]);
        for _ in 0..col_len {
            let mut row = vec![ContainerCell::Edge];
            row.resize(row_len-1, ContainerCell::Empty);
            row.push(ContainerCell::Edge);
            container_img.cells.push(row);
        }
        container_img.cells.push(vec![ContainerCell::Edge; row_len]);
        

        container_img
    }

    pub fn add_plant(mut self, plant_symbol: &'static str, size: i64, x_pos: f64, y_pos: f64) -> Self{
        let plant_cell = ContainerCell::Plant { 
            val: plant_symbol, 
            is_root: true, 
            size: size 
        };

        self.cells[(x_pos*self.scaler) as usize][(y_pos*self.scaler) as usize] = plant_cell;

        self
    }

    pub fn img_str(self) -> String{
        let mut img_str = String::new();

        for row in self.cells.iter() {
            for col in row.iter(){
                match col.symbol() {
                    Some(val) => img_str.push(val),
                    None => {}
                }
                img_str.push(' ');
            }
            img_str.push('\n');
        }
        println!("{}", img_str);

        img_str
    }
}