use rand::Rng;

use crate::widgets::Cell;

fn generate_mines(mines: usize, max: usize) -> Vec<usize> {
    let mut temp = Vec::<usize>::with_capacity(mines);
    let mut rand = rand::thread_rng();

    for _ in 0..mines {
        'gen: loop {
            let num = rand.gen_range(0..max);
            for i in temp.iter() {
                if *i == num {
                    continue 'gen;
                }
                temp.push(num);
                break 'gen;
            }
        }
    }

    temp
}

fn has_mine(pos: usize, mine_pos: &Vec<usize>) -> bool {
    for i in mine_pos {
        if *i == pos {
            return true;
        }
    }

    false
}

pub fn generate_cells(width: usize, height: usize, mines: usize) -> Vec<Cell> {
    let mine_pos = generate_mines(mines, width * height);

    let mut temp = Vec::<Cell>::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            temp.push(Cell {
                x: (x * 24) as i32,
                y: (y * 24) as i32,
                mine: has_mine(y * width + x, &mine_pos),
            });
        }
    }
    temp
}