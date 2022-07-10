use std::collections::HashSet;

use rand::Rng;

use crate::widgets::Cell;

fn generate_mines(mines: usize, max: usize) -> HashSet<usize> {
    let mut temp = HashSet::<usize>::with_capacity(mines);
    let mut rand = rand::thread_rng();
    let mut counter = mines;

    while counter > 0 {
        let num = rand.gen_range(0..max);
        if temp.contains(&num) {
            continue;
        } else {
            temp.insert(num);
            counter -= 1;
        }
    }

    temp
}

pub fn generate_cells(width: usize, height: usize, mines: usize) -> Vec<Cell> {
    let mine_pos = generate_mines(mines, width * height);

    let mut temp = Vec::<Cell>::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            temp.push(Cell {
                x: (x * 24) as i32,
                y: (y * 24) as i32,
                mine: mine_pos.contains(&(y * width + x)),
            });
        }
    }
    temp
}