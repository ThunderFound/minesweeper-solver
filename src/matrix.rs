use std::collections::HashMap;

use crate::{UNKNOWN, MINE, SAFE};

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

pub struct Matrix {
    pub data: Vec<i32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0; rows * cols],
            rows: rows,
            cols: cols,
        }
    }

    pub fn from_array<const R: usize, const C: usize>(matrix: [[i32; C]; R]) -> Self {
        Self {
            data: matrix.into_iter().flatten().collect(),
            rows: R,
            cols: C,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: i32) {
        self.data[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.cols + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut i32 {
        &mut self.data[row * self.cols + col]
    }

    pub fn get_row(&self, row: usize) -> &[i32] {
        &self.data[row * self.cols..(row + 1) * self.cols]
    }

    pub fn get_row_mut(&mut self, row: usize) -> &mut [i32] {
        &mut self.data[row * self.cols..(row + 1) * self.cols]
    }

    pub fn get_row_coeffs_and_rhs(&self, row: usize) -> (&[i32], i32) {
        let (&rhs, coeffs) = self.get_row(row).split_last().unwrap();
        (coeffs, rhs)
    }

    pub fn divide_row_by_value(&mut self, row: usize, value: i32) {
        if value == 0 { return; }
        self.get_row_mut(row)
            .iter_mut()
            .for_each(|x| *x /= value);
    }

    pub fn multiply_row_by_value(&mut self, row: usize, value: i32) {
        self.get_row_mut(row)
            .iter_mut()
            .for_each(|x| *x *= value);
    }

    pub fn remove_col(&mut self, col: usize) {
        for row in (0..self.rows).rev() {
            self.data.remove(row * self.cols + col);
        }
        self.cols -= 1;
    }

    pub fn move_column_to_rhs(&mut self, col: usize, value: i32) {
        for row in (0..self.rows).rev() {
            let coeff = self.get(row, col);
            *self.get_mut(row, self.cols - 1) -= coeff * value;
            self.data.remove(row * self.cols + col);
        }
        self.cols -= 1;
    }

    pub fn remove_row(&mut self, row: usize) {
        self.data.drain(row * self.cols..(row + 1) * self.cols);
        self.rows -= 1;
    }

    pub fn swap_rows(&mut self, mut row_a: usize, mut row_b: usize) {
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

    pub fn normalize_row(&mut self, row: usize) {
        let row_slice = self.get_row(row);
        let row_gcd = gcd_array(row_slice);
        self.divide_row_by_value(row, row_gcd);
    }

    pub fn calculate_rhs_bounds_from_coeffs(&self, coeffs: &[i32]) -> (i32, i32) {
        (
            coeffs.iter().filter(|&&x| x < 0).sum(),
            coeffs.iter().filter(|&&x| x > 0).sum(),
        )
    }

    pub fn calculate_mines_from_row(&self, row: usize, unknown_tiles: &Vec<(usize, usize)>, resolved: &mut HashMap<(usize, usize), i32>) {
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

    pub fn convert_to_rref_and_calculate_mines(&mut self, unknown_tiles: &Vec<(usize, usize)>, resolved: &mut HashMap<(usize, usize), i32>) {
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

    pub fn print_augmented_matrix(&self) {
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

    pub fn print_field(&self) {
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