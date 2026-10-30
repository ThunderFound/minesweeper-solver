use minesweeper_solver::matrix::Matrix;
use minesweeper_solver::solver::solve;
use minesweeper_solver::{UNKNOWN, MINE, SAFE};

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

    let resolved = solve(&field);
    for (tile, value) in resolved.iter() {
        field.set(tile.0, tile.1, match value {
            1 => MINE,
            0 => SAFE,
            _ => UNKNOWN,
        });
    }
    field.print_field();
}