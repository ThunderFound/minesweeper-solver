use std::collections::HashMap;

const UNKNOWN: i32 = -1;
const MINE: i32 = -2;
const SAFE: i32 = -3;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b)
    };
    a.abs()
}

fn gcd_array(nums: &[i32]) -> i32 {
    nums.iter().skip(1).fold(nums[0], |acc, &x| gcd(acc, x))
}

fn lcm_array(nums: &[i32]) -> i32 {
    nums.iter().product::<i32>() / gcd_array(nums)
}

struct Matrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0; rows * cols],
            rows: rows,
            cols: cols,
        }
    }

    fn from_array<const R: usize, const C: usize>(matrix: [[i32; C]; R]) -> Self {
        Self {
            data: matrix.into_iter().flatten().collect(),
            rows: R,
            cols: C,
        }
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        self.data[row * self.cols + col] = value;
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.cols + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut i32 {
        &mut self.data[row * self.cols + col]
    }

    fn get_row(&self, row: usize) -> &[i32] {
        &self.data[row * self.cols..(row + 1) * self.cols]
    }

    fn get_row_mut(&mut self, row: usize) -> &mut [i32] {
        &mut self.data[row * self.cols..(row + 1) * self.cols]
    }

    fn get_row_coeffs_and_rhs(&self, row: usize) -> (&[i32], i32) {
        let (&rhs, coeffs) = self.get_row(row).split_last().unwrap();
        (coeffs, rhs)
    }

    fn divide_row_by_value(&mut self, row: usize, value: i32) {
        if value == 0 { return; }
        self.get_row_mut(row)
            .iter_mut()
            .for_each(|x| *x /= value);
    }

    fn multiply_row_by_value(&mut self, row: usize, value: i32) {
        self.get_row_mut(row)
            .iter_mut()
            .for_each(|x| *x *= value);
    }

    fn remove_col(&mut self, col: usize) {
        for row in (0..self.rows).rev() {
            self.data.remove(row * self.cols + col);
        }
        self.cols -= 1;
    }

    fn move_column_to_rhs(&mut self, col: usize, value: i32) {
        for row in (0..self.rows).rev() {
            let coeff = self.get(row, col);
            *self.get_mut(row, self.cols - 1) -= coeff * value;
            self.data.remove(row * self.cols + col);
        }
        self.cols -= 1;
    }

    fn remove_row(&mut self, row: usize) {
        self.data.drain(row * self.cols..(row + 1) * self.cols);
        self.rows -= 1;
    }

    fn swap_rows(&mut self, mut row_a: usize, mut row_b: usize) {
        if row_a == row_b { return; }
        let rows = self.rows;
        let cols = self.cols;
        if row_a >= rows || row_b >= rows { return; }

        if row_a > row_b { (row_a, row_b) = (row_b, row_a); }

        let start_a = row_a * cols;
        let start_b = row_b * cols;
        let (left, right) = self.data.split_at_mut(start_b);
        left[start_a..start_a + cols].swap_with_slice(&mut right[0..cols]);
    }

    fn normalize_row(&mut self, row: usize) {
        let row_slice = self.get_row(row);
        let row_gcd = gcd_array(row_slice);
        self.divide_row_by_value(row, row_gcd);
    }

    fn calculate_rhs_bounds_from_coeffs(&self, coeffs: &[i32]) -> (i32, i32) {
        (
            coeffs.iter().filter(|&&x| x < 0).sum(),
            coeffs.iter().filter(|&&x| x > 0).sum(),
        )
    }

    fn calculate_mines_from_row(&self, row: usize, unknown_tiles: &Vec<(usize, usize)>, resolved: &mut HashMap<(usize, usize), i32>) {
        let (coeffs, rhs) = self.get_row_coeffs_and_rhs(row);
        let (min_bound, max_bound) = self.calculate_rhs_bounds_from_coeffs(coeffs);
        
        let mine_sign = if rhs == min_bound {
            -1
        } else if rhs == max_bound {
            1
        } else {
            return;
        };

        for (&tile, &coeff) in unknown_tiles.iter().zip(coeffs) {
            if coeff == 0 { continue; }
            if coeff.signum() == mine_sign {
                resolved.insert(tile, 1);
            } else {
                resolved.insert(tile, 0);
            }
        }
    }

    fn convert_to_rref_and_calculate_mines(&mut self, unknown_tiles: &Vec<(usize, usize)>, resolved: &mut HashMap<(usize, usize), i32>) {
        for row in 0..self.rows {
            self.calculate_mines_from_row(row, unknown_tiles, resolved);
        }

        let mut lead = 0;
        for row in 0..self.rows {
            if self.cols < lead {
                return;
            }

            let mut i = row;
            while self.get(i, lead) == 0 {
                i += 1;
                if self.rows == i {
                    i = row;
                    lead += 1;
                    if self.cols == lead {
                        return;
                    }
                }
            }

            self.swap_rows(i, row);

            if self.get(row, lead) < 0 {
                self.multiply_row_by_value(row, -1);
            }

            let a = self.get(row, lead);
            for i in 0..self.rows {
                if i != row {
                    let b = self.get(i, lead);
                    if b == 0 { continue; }

                    for j in 0..self.cols {
                        let v = a * self.get(i, j) - b * self.get(row, j);
                        self.set(i, j, v);
                    }
                    self.normalize_row(i);
                    self.calculate_mines_from_row(i, unknown_tiles, resolved);
                }
            }
            lead += 1;
        }
    }

    fn print_augmented_matrix(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let n = self.get(y, x);
                if x == self.cols - 1 {
                    println!("| {n}");
                } else {
                    print!(" {n} ");
                }
            }
        }
    }

    fn print_field(&self) {
        for i in 0..self.data.len() {
            let tile_value = self.data[i];
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
            if i % self.cols == self.cols - 1 {
                print!("{}\x1b[0m\n", text);
            } else {
                print!("{}\x1b[0m", text);
            }
        }
    }
}

fn main() {
    // let mut field = Matrix::from_array([
    //     [-4, -1,  2,  2, -1, -1, -1],
    //     [-1, -1, -1, -1,  2,  3, -1],
    //     [-1,  2, -1, -1, -1,  4, -1],
    //     [ 2, -1, -1,  2, -1, -1,  2],
    //     [-1,  2, -1, -1, -1, -1, -1],
    //     [-1,  2,  3, -1,  3, -1,  3],
    //     [-1, -1, -1,  2, -1, -1, -1],
    // ]);
    let mut field = Matrix::from_array([
        [-1,  1, -1, -1,  1, -1,  2, -1, -1],
        [-1,  2, -1, -1, -1, -1, -1,  3,  2],
        [ 2,  3, -1, -1,  2, -1, -1, -1, -1],
        [-1, -1,  2, -1,  1,  3,  4, -1,  2],
        [ 1, -1, -1,  1, -1,  1, -1, -1,  2],
        [-1,  2,  1, -1, -1, -1,  1, -1, -1],
        [-1, -1,  2, -1, -1, -1,  2, -1,  3],
        [ 3, -1,  2, -1,  1, -1, -1, -1, -1],
        [-1, -1, -1, -1, -1,  1, -1, -1,  2],
    ]);
    field.print_field();

    let mut unknown_tiles: Vec<(usize, usize)> = Vec::new();
    let mut numbered_tiles: Vec<(usize, usize)> = Vec::new();
    for y in 0..field.rows {
        for x in 0..field.cols {
            let tile_value = field.get(y, x);
            'adj_check: for adj_y in y.saturating_sub(1)..=(y + 1).min(field.rows - 1) {
                for adj_x in x.saturating_sub(1)..=(x + 1).min(field.cols - 1) {
                    if adj_y == y && adj_x == x { continue; }
                    
                    let adj_tile_value = field.get(adj_y, adj_x);
                    if (0 <= tile_value && tile_value <= 8) && (adj_tile_value == UNKNOWN) {
                        numbered_tiles.push((y, x));
                        break 'adj_check;
                    } else if (tile_value == UNKNOWN) && (0 <= adj_tile_value && adj_tile_value <= 8) {
                        unknown_tiles.push((y, x));
                        break 'adj_check;
                    }
                }
            }
        }
    }
    println!("({}) {:?}", unknown_tiles.len(), unknown_tiles);
    println!("({}) {:?}", numbered_tiles.len(), numbered_tiles);

    let mut matrix = Matrix::new(numbered_tiles.len(), unknown_tiles.len() + 1);
    for (numbered_index, &(y, x)) in numbered_tiles.iter().enumerate() {
        let mut tile_value = field.get(y, x);

        for adj_y in y.saturating_sub(1)..=(y + 1).min(field.rows - 1) {
            for adj_x in x.saturating_sub(1)..=(x + 1).min(field.cols - 1) {
                if adj_y == y && adj_x == x { continue; }
                
                let adj_tile_value = field.get(adj_y, adj_x);
                match adj_tile_value {
                    MINE => { tile_value -= 1; },
                    UNKNOWN => {
                        let unknown_index = unknown_tiles.iter().position(|tile| tile == &(adj_y, adj_x)).unwrap();
                        matrix.set(numbered_index, unknown_index, 1);
                    }
                    _ => (),
                }
            }
        }

        matrix.set(numbered_index, matrix.cols - 1, tile_value);
    }
    matrix.print_augmented_matrix();

    let mut resolved: HashMap<(usize, usize), i32> = HashMap::new();
    while unknown_tiles.len() > 0 {
        let old_resolved_len = resolved.len();
        while unknown_tiles.len() > 0 {
            let old_resolved_len = resolved.len();

            matrix.convert_to_rref_and_calculate_mines(&unknown_tiles, &mut resolved);
            println!();
            matrix.print_augmented_matrix();

            for i in (0..unknown_tiles.len()).rev() {
                let tile = unknown_tiles.get(i).unwrap();
                if let Some(&tile_value) = resolved.get(tile) {
                    matrix.move_column_to_rhs(i, tile_value);
                    unknown_tiles.remove(i);
                    println!("reduced col");
                }
            }

            for row in (0..matrix.rows).rev() {
                if matrix.get_row(row).iter().rev().skip(1).all(|x| x == &0) {
                    matrix.remove_row(row);
                }
            }

            if resolved.len() == old_resolved_len {
                break;
            }
        }

        let mut assumed: HashMap<(usize, usize), i32> = HashMap::new();
        for check_index in (0..unknown_tiles.len()).rev() {
            let check_tile = unknown_tiles[check_index];
            let mut contradicted = [false, false];
            for check_tile_value in 0..=1 {
                assumed.insert(check_tile, check_tile_value as i32);
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
                            println!("{:?} cannot hold value {}", check_tile, check_tile_value);
                            contradicted[check_tile_value] = true;
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
                assumed.clear();
            }
            println!("{:?}", contradicted);
            match contradicted {
                [true, false] => {
                    resolved.insert(check_tile, 1);
                    matrix.move_column_to_rhs(check_index, 1);
                    unknown_tiles.remove(check_index);
                },
                [false, true] => {
                    resolved.insert(check_tile, 0);
                    matrix.move_column_to_rhs(check_index, 0);
                    unknown_tiles.remove(check_index);
                },
                [true, true] => { println!("invalid board"); },
                [false, false] => (),
            }
        }

        if resolved.len() == old_resolved_len {
            break;
        }
    }

    // debug
    for (tile, value) in resolved.iter() {
        field.set(tile.0, tile.1, match value {
            1 => MINE,
            0 => SAFE,
            _ => UNKNOWN,
        });
    }
    field.print_field();
}
