use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn range(min: i32, max: i32) -> Vec<i32> {
    (min..=max).collect::<Vec<_>>()
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (x * self.width + y) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (x + delta_row) % self.height;
                let neighbor_col = (y + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                let neighbors = self.live_neighbor_count(x, y);
                let next_cell = match (cell, neighbors) {
                    (Cell::Alive, x) if ![2, 3].contains(&x) => Cell::Dead,
                    (Cell::Alive, x) if [2, 3].contains(&x) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (x, _) => x,
                };
                next[index] = next_cell;
            })
        });

        self.cells = next;
    }

    pub fn new() -> Universe {
        Universe::default()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Default for Universe {
    fn default() -> Self {
        let [width, height] = [64, 64];
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cells
            .as_slice()
            .chunks(self.width as usize)
            .for_each(|line| {
                line.iter().for_each(|cell| {
                    let symbol = if *cell == Cell::Dead { " " } else { "●" };
                    write!(f, "{}", symbol).unwrap();
                });
                writeln!(f).unwrap();
            });

        Ok(())
    }
}
