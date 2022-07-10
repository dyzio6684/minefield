use crate::widgets::Cell;

pub fn generate_cells(w: usize, h: usize) -> Vec<Cell> {
    let mut temp = Vec::<Cell>::with_capacity(w * h);
    for x in 0..w {
        for y in 0..h {
            temp.push(Cell {
                x: (x * 24) as i32,
                y: (y * 24) as i32,
            });
        }
    }
    temp
}