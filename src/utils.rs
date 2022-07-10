use std::collections::HashSet;

use rand::Rng;

use crate::{widgets::{Cell, Widget}, types::CellState};

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
                gx: x,
                gy: y,
                mine: mine_pos.contains(&(y * width + x)),
                state: CellState::Hidden(0),
            });
        }
    }

    let cells = temp.clone();

    for y in 0..(height as isize) {
        for x in 0..(width as isize) {
            if let Some(s) = temp.get_mut((y * width as isize + x) as usize) {
                let mut mines = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        if let Some(s) = cells.get(((y + i) * width as isize + x + j) as usize) {
                            if s.mine {
                                mines += 1;
                            }
                        }
                    }
                }

                s.state = CellState::Hidden(mines);
            }
        }
    }

    temp
}

pub fn is_hovered(widget: &dyn Widget, x: i32, y: i32) -> bool {
    x >= widget.get_pos().0 &&
    y >= widget.get_pos().1 &&
    x <= widget.get_pos().0 + widget.get_size().0 as i32 &&
    y <= widget.get_pos().1 + widget.get_size().1 as i32
}

pub fn change_state(cells: &mut Vec<Cell>, x: usize, y: usize, width: usize, height: usize, clicked: bool) {
    if let Some(s) = cells.get_mut(y * width + x) {
        if !s.mine {
            match s.state {
                CellState::Hidden(0) => {
                    s.state = CellState::Revealed(0);
                    if x > 0 {
                        change_state(cells, x - 1, y, width, height, false);
                    }
                    if x < width {
                        change_state(cells, x + 1, y, width, height, false);
                    }
                    if y > 0 {
                        change_state(cells, x, y - 1, width, height, false);
                    }
                    if y < height {
                        change_state(cells, x, y + 1, width, height, false);
                    }
                }
                CellState::Hidden(i) => s.state = CellState::Revealed(i),
                _ => {}
            }
        } else {
            if clicked {
                s.state = CellState::Revealed(10);
            }
        }
    }
}