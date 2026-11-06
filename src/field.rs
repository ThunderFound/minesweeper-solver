use std::time::Instant;
use std::collections::{HashMap, HashSet};
use rand::RngExt;

use crate::matrix::Matrix;
use crate::{UNKNOWN, MINE, SAFE};

pub struct Field {
    pub field: Matrix,
    pub mine_tiles: HashSet<(usize, usize)>,
    pub all_unknown_tiles: HashSet<(usize, usize)>,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Field {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let mut rng = rand::rng();
        
        let field = Matrix::new(height, width, UNKNOWN);
        let mut mine_tiles = HashSet::new();

        let mut placed_mines = 0;
        while placed_mines < mines {
            let row = rng.random_range(0..height);
            let col = rng.random_range(0..width);
            if !mine_tiles.contains(&(row, col)) {
                mine_tiles.insert((row, col));
                placed_mines += 1;
            }
        }

        let all_unknown_tiles = HashSet::from_iter((0..field.rows)
            .flat_map(|row| {
                (0..field.cols).map(move |col| (row, col))
            })
        );

        Self {
            field: field,
            mine_tiles: mine_tiles,
            all_unknown_tiles: all_unknown_tiles,
            width: width,
            height: height,
            mines: mines,
        }
    }

    pub fn from_matrix(matrix: Matrix) -> Self {
        let all_unknown_tiles: HashSet<(usize, usize)> = HashSet::from_iter(
            (0..matrix.rows)
                .map(|row| matrix.get_row(row))
                .enumerate()
                .flat_map(|(row, row_slice)| {
                    row_slice
                        .iter()
                        .enumerate()
                        .filter(|&(_, tile_value)| *tile_value == UNKNOWN)
                        .map(move |(col, _)| (row, col))
                })
        );
        let width = matrix.cols;
        let height = matrix.rows;
        Self {
            field: matrix,
            mine_tiles: HashSet::new(),
            all_unknown_tiles: all_unknown_tiles,
            width: width,
            height: height,
            mines: 0,
        }
    }

    pub fn get_neighboring_tiles(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        (row.saturating_sub(1)..=(row + 1).min(self.field.rows - 1)).flat_map(move |adj_row| {
            (col.saturating_sub(1)..=(col + 1).min(self.field.cols - 1))
                .map(move |adj_col| (adj_row, adj_col))
        })
        .filter(move |&(adj_row, adj_col)| adj_row != row || adj_col != col )
    }

    pub fn solve(&mut self) -> (bool, HashMap<(usize, usize), i32>) {
        let start = Instant::now();
        let mut unknown_tiles: Vec<(usize, usize)> = Vec::new();
        let mut numbered_tiles: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.field.rows {
            for col in 0..self.field.cols {
                let tile_value = self.field.get(row, col);
                'adj_check: for adj_row in row.saturating_sub(1)..=(row + 1).min(self.field.rows - 1) {
                    for adj_col in col.saturating_sub(1)..=(col + 1).min(self.field.cols - 1) {
                        if adj_row == row && adj_col == col { continue; }
                        
                        let adj_tile_value = self.field.get(adj_row, adj_col);
                        if (0 <= tile_value && tile_value <= 8) && (adj_tile_value == UNKNOWN) {
                            numbered_tiles.push((row, col));
                            break 'adj_check;
                        } else if (tile_value == UNKNOWN) && (0 <= adj_tile_value && adj_tile_value <= 8) {
                            unknown_tiles.push((row, col));
                            break 'adj_check;
                        }
                    }
                }
            }
        }
        let mut unknown_tiles_count = unknown_tiles.len();
        println!("getting unknown tiles and numbered tiles took {:?} | unknowns: {}", start.elapsed(), unknown_tiles.len());

        // let start = Instant::now();
        let mut groups: usize = 0;
        let mut unknown_groups: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        let mut numbered_groups: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        
        unknown_groups.insert(0, unknown_tiles);

        numbered_groups.insert(0, numbered_tiles);
        groups += 1;
        // println!("grouping unknown tiles and numbered tiles took {:?} | unknowns: {}", start.elapsed(), unknown_tiles.len());

        let mut resolved: HashMap<(usize, usize), i32> = HashMap::new();

        for group in 0..groups {
            let unknown_tiles = unknown_groups.get_mut(&group).unwrap();
            let numbered_tiles = numbered_groups.get_mut(&group).unwrap();

            let start = Instant::now();
            let mut matrix = Matrix::new(numbered_tiles.len(), unknown_tiles.len() + 1, 0);
            for (numbered_index, &(row, col)) in numbered_tiles.iter().enumerate() {
                let mut tile_value = self.field.get(row, col);
                self.get_neighboring_tiles(row, col).for_each(|(adj_row, adj_col)| {
                    let adj_tile_value = self.field.get(adj_row, adj_col);
                    match adj_tile_value {
                        MINE => { tile_value -= 1; },
                        UNKNOWN => {
                            let unknown_index = unknown_tiles.iter().position(|tile| tile == &(adj_row, adj_col)).unwrap();
                            matrix.set(numbered_index, unknown_index, 1);
                        }
                        _ => (),
                    }
                });

                matrix.set(numbered_index, matrix.cols - 1, tile_value);
            }
            println!("building matrix took {:?}", start.elapsed());

            while unknown_tiles.len() > 0 {
                println!("\n--- iter start");
                while unknown_tiles.len() > 0 {
                    let start = Instant::now();
                    let old_resolved_len = resolved.len();

                    matrix.convert_to_rref_and_calculate_mines(&unknown_tiles, &mut resolved);

                    for i in (0..unknown_tiles.len()).rev() {
                        let tile = unknown_tiles.get(i).unwrap();
                        if let Some(&tile_value) = resolved.get(tile) {
                            matrix.move_column_to_rhs(i, tile_value);
                            unknown_tiles.remove(i);
                            unknown_tiles_count -= 1;
                        }
                    }

                    for row in (0..matrix.rows).rev() {
                        if matrix.get_row(row).iter().rev().skip(1).all(|x| x == &0) {
                            matrix.remove_row(row);
                        }
                    }

                    println!("rref took {:?} | left unknown: {}", start.elapsed(), unknown_tiles.len());
                    if resolved.len() == old_resolved_len {
                        break;
                    }
                }

                if unknown_tiles.len() == 0 {
                    println!("--- iter end");
                    break;
                }

                let old_resolved_len = resolved.len();
                println!("\nsubstitution started | resolved: {} | left unknown: {}", old_resolved_len, unknown_tiles.len());

                let start = Instant::now();
                let mut assumed: HashMap<(usize, usize), i32> = HashMap::new();
                for check_index in (0..unknown_tiles.len()).rev() {
                    let check_tile = unknown_tiles[check_index];
                    let mut already_contradicted = false;
                    for check_tile_value in 0..=1 {
                        assumed.insert(check_tile, check_tile_value as i32);
                        let start = Instant::now();
                        let mut should_continue = true;
                        'check: while should_continue {
                            should_continue = false;
                            for row in 0..matrix.rows {
                                let row_slice = matrix.get_row(row);
                                let rhs = row_slice[matrix.cols - 1];
                                let (min_bound, max_bound, rhs): (i32, i32, i32) = row_slice.iter().enumerate().rev().skip(1).fold((0, 0, rhs), |mut acc, (i, &coeff)| {
                                    if let Some(&tile_value) = resolved.get(unknown_tiles.get(i).unwrap()) {
                                        acc.2 -= coeff * tile_value;
                                    } else if let Some(&tile_value) = assumed.get(unknown_tiles.get(i).unwrap()) {
                                        acc.2 -= coeff * tile_value;
                                    } else {
                                        if coeff < 0 {
                                            acc.0 += coeff;
                                        } else if coeff > 0 {
                                            acc.1 += coeff;
                                        }
                                    }
                                    acc
                                });

                                if rhs < min_bound || max_bound < rhs {
                                    already_contradicted = true;
                                    resolved.insert(check_tile, 1 - check_tile_value);
                                    matrix.move_column_to_rhs(check_index, 1 - check_tile_value);
                                    unknown_tiles.remove(check_index);
                                    unknown_tiles_count -= 1;
                                    break 'check;
                                }

                                let mine_sign = if rhs == min_bound {
                                    -1
                                } else if rhs == max_bound {
                                    1
                                } else {
                                    continue;
                                };

                                for (tile, &coeff) in unknown_tiles.iter().zip(row_slice) {
                                    if coeff == 0 || resolved.contains_key(tile) || assumed.contains_key(tile) { continue; }
                                    let new_value = if coeff.signum() == mine_sign { 1 } else { 0 };
                                    assumed.insert(*tile, new_value);
                                    should_continue = true;
                                }
                            }
                        }
                        println!("substitution iter for {:?} = {} took {:?} | resolved: {}", check_tile, check_tile_value, start.elapsed(), resolved.len());
                        assumed.clear();
                        if already_contradicted {
                            break;
                        }
                    }
                }
                println!("substitution took {:?} | left unknown: {}", start.elapsed(), unknown_tiles.len());
                println!("--- iter end");

                if resolved.len() == old_resolved_len {
                    break;
                }
            }
        }

        resolved
            .iter()
            .for_each(|(tile, _)| {
                self.all_unknown_tiles.remove(tile);
            });
        println!("{} {}", unknown_tiles_count, unknown_groups.values().map(|unknown_tiles| unknown_tiles.len()).sum::<usize>());
        (unknown_tiles_count == 0, resolved)
    }

    pub fn print_field(&self) {
        for i in 0..self.field.data.len() {
            let tile_value = self.field.data[i];
            let text = match tile_value {
                MINE => "\x1b[48;2;255;0;0m  ",
                SAFE => "\x1b[48;2;0;255;0m  ",
                UNKNOWN => "\x1b[48;2;128;128;64m  ",
                0 => "\x1b[48;2;192;192;128m  ",
                1 => "\x1b[48;2;192;192;128m\x1b[38;2;0;0;255m1 ",
                2 => "\x1b[48;2;192;192;128m\x1b[38;2;0;128;0m2 ",
                3 => "\x1b[48;2;192;192;128m\x1b[38;2;255;0;0m3 ",
                4 => "\x1b[48;2;192;192;128m\x1b[38;2;0;0;128m4 ",
                5 => "\x1b[48;2;192;192;128m\x1b[38;2;128;0;0m5 ",
                6 => "\x1b[48;2;192;192;128m\x1b[38;2;0;128;128m6 ",
                7 => "\x1b[48;2;192;192;128m\x1b[38;2;0;0;0m7 ",
                8 => "\x1b[48;2;192;192;128m\x1b[38;2;128;128;128m8 ",
                _ => "  ",
            };
            if i % self.field.cols == self.field.cols - 1 {
                print!("{}\x1b[0m\n", text);
            } else {
                print!("{}\x1b[0m", text);
            }
        }
    }

    pub fn reveal(&mut self, row: usize, col: usize) -> bool {
        let tile = (row, col);
        if self.mine_tiles.contains(&tile) {
            // println!("tried to reveal a mine");
            return false;
        }

        self.all_unknown_tiles.remove(&tile);

        let mut mine_count = 0;
        for adj_row in row.saturating_sub(1)..=(row + 1).min(self.field.rows - 1) {
            for adj_col in col.saturating_sub(1)..=(col + 1).min(self.field.cols - 1) {
                if adj_row == row && adj_col == col { continue; }
                
                if self.mine_tiles.contains(&(adj_row, adj_col)) {
                    mine_count += 1;
                }
            }
        }

        self.field.set(row, col, mine_count);

        if mine_count == 0 {
            for adj_row in row.saturating_sub(1)..=(row + 1).min(self.field.rows - 1) {
                for adj_col in col.saturating_sub(1)..=(col + 1).min(self.field.cols - 1) {
                    if adj_row == row && adj_col == col { continue; }
                    let adj_tile_value = self.field.get(adj_row, adj_col);
                    if adj_tile_value == UNKNOWN {
                        self.reveal(adj_row, adj_col);
                    } 
                }
            }
        }
        true
    }

    pub fn get_starting_tile(&self) -> (usize, usize) {
        let mut rng = rand::rng();

        loop {
            // this is so stupid im sorry
            let row = rng.random_range(0..self.height);
            let col = rng.random_range(0..self.width);
            if !self.mine_tiles.contains(&(row, col)) {
                let mut mine_count = 0;
                for adj_row in row.saturating_sub(1)..=(row + 1).min(self.field.rows - 1) {
                    for adj_col in col.saturating_sub(1)..=(col + 1).min(self.field.cols - 1) {
                        if adj_row == row && adj_col == col { continue; }
                        if self.mine_tiles.contains(&(adj_row, adj_col)) {
                            mine_count += 1;
                        }
                    }
                }
                if mine_count == 0 {
                    return (row, col);
                }
            }
        }
    }

    pub fn get_random_tile(&self) -> (usize, usize) {
        let mut rng = rand::rng();

        // loop {
        //     let row = rng.random_range(0..self.height);
        //     let col = rng.random_range(0..self.width);
        //     if self.field.get(row, col) == UNKNOWN {
        //         return (row, col);
        //     }
        // }
        self.all_unknown_tiles
            .iter()
            .nth(rng.random_range(0..self.all_unknown_tiles.len()))
            .unwrap()
            .clone()
    }
}