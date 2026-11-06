use std::io::Write;
use std::time::{Duration, Instant};

use minesweeper_solver::field::Field;
use minesweeper_solver::matrix::Matrix;
use minesweeper_solver::{UNKNOWN, MINE, SAFE};

// fn get_median(array: &mut [u128]) -> u128 {
//     array.sort_unstable();
//     if array.len() % 2 == 0 {
//         (array[array.len() / 2] + array[array.len() / 2 + 1]) / 2
//     } else {
//         array[array.len() / 2 + 1]
//     }
// }

// fn get_mean(array: &[u128]) -> u128 {
//     array.iter().sum::<u128>() / array.len() as u128
// }

// fn micros_to_millis(micros: u128) -> f64 {
//     micros as f64 / 1000.0
// }

// fn main() {
//     const TOTAL_ATTEMPTS: usize = 1000;
    
//     let mut successful_attempts = 0;
//     let mut attempt_times = [0; TOTAL_ATTEMPTS];
//     let mut successful_attempt_times: Vec<u128> = Vec::new();

//     let start = Instant::now();
//     for att in 0..TOTAL_ATTEMPTS {
//         // print!("{att}\r");
//         // std::io::stdout().flush();
//         let mut field = Field::new(30, 16, 99);

//         let starting_tile = field.get_starting_tile();
//         field.reveal(starting_tile.0, starting_tile.1);
//         // field.print_field();
        
//         let start = Instant::now();
//         let mut is_solved = false;
//         let mut solving_iters = 0;
//         let mut random_tiles = 0;
//         let mut total_solve_time = Duration::new(0, 0);
//         loop {
//             if total_solve_time.as_millis() > 100 {
//                 field.print_field();
//                 field.field.print_augmented_matrix();
//             }
//             let start = Instant::now();
//             let (solved, resolved) = field.solve();
//             total_solve_time += start.elapsed();
//             if total_solve_time.as_millis() > 100 {
//                 println!("solving took {:?}", start.elapsed());
//             }

//             if resolved.len() == 0 {
//                 random_tiles += 1;
//                 let start = Instant::now();
//                 let tile = field.get_random_tile();
//                 if total_solve_time.as_millis() > 100 {
//                     println!("getting random tile took {:?}", start.elapsed());
//                 }
//                 let success = field.reveal(tile.0, tile.1);
//                 if !success {
//                     // println!("      revealed a mine while randomly opening");
//                     break;
//                 }
//             }

//             for (tile, &value) in resolved.iter() {
//                 field.field.set(tile.0, tile.1, match value {
//                     1 => MINE,
//                     0 => SAFE,
//                     _ => UNKNOWN,
//                 });
//             }

//             for (tile, &value) in resolved.iter() {
//                 if value == 0 {
//                     let success = field.reveal(tile.0, tile.1);
//                     if !success {
//                         println!("revealed a mine from a solution");
//                     }
//                 }
//             }

//             solving_iters += 1;
//             if solved {
//                 successful_attempts += 1;
//                 is_solved = true;
//                 break;
//             }
//         }
//         let micros = start.elapsed().as_micros();
//         if total_solve_time.as_millis() > 100 {
//             println!("solved: {} | iters: {} | random: {} | took {}ms ({:?} - solving)\n", is_solved, solving_iters, random_tiles, micros as f64 / 1000.0, total_solve_time);
//         }
//         attempt_times[att] = micros;
//         if is_solved {
//             successful_attempt_times.push(micros);
//         }
//     }
//     println!("\ntook {:?}", start.elapsed());

//     println!("total attempts: {}", TOTAL_ATTEMPTS);
//     println!("successful attempts: {}", successful_attempts);
//     println!("success rate: {:.5}%", successful_attempts as f64 / TOTAL_ATTEMPTS as f64 * 100.0);

//     println!("mean attempt: {}ms", micros_to_millis(get_mean(&attempt_times)));
//     println!("median attempt: {}ms", micros_to_millis(get_median(&mut attempt_times)));

//     println!("mean successful attempt: {}ms", micros_to_millis(get_mean(&successful_attempt_times)));
//     println!("median successful attempt: {}ms", micros_to_millis(get_median(&mut successful_attempt_times)));

//     println!("{:?}", attempt_times);
//     println!("{:?}", successful_attempt_times);
// }

fn main() {
    let mut field = Field::from_matrix(
        Matrix::from_array([
[-1,-1,-1,-1,-1,-1,3,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,2,-1,-1,-1,-1,-1],
[ -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -1,1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -1,-1,3,-2,-2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ 1,2,-2,4,-2,5,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ 0,1,1,2,3,-2,-1,-1,-1,2,2,2,2,2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ 1,1,1,1,3,-2,-1,-1,-1,1,-1,-1,-1,1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -1,-1,1,-1,-1,4,-1,-1,-1,2,1,1,1,1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ -2,3,2,3,-2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
[ 2,-2,2,3,-2,4,-2,2,1,2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,1,1,3,-1,-1],
[ 2,3,-2,2,1,2,1,1,1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-2,-1,-1,1,-1,-1,-1],
[ 2,-2,2,1,0,0,0,1,3,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,4,-2,3,1,2,4,-1,-1],
[ -2,2,1,0,0,0,0,1,-2,-2,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,2,1,0,1,-2,-1,1],
[ 1,1,0,0,0,0,0,1,2,3,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,1,0,0,1,2,-1,-1],
        ])
    );
//     let mut field = Field::from_matrix(
//         Matrix::from_array([
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1,  1, -1, -1,  1, -1, -1,  1, -1, -1,  1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1,  1, -1, -1,  1, -1, -1,  1, -1, -1,  1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1,  1, -1, -1,  1, -1, -1,  1, -1, -1,  1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
// [-1,  1, -1, -1,  1, -1, -1,  1, -1, -1,  1, -1],
// [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
//         ])
//     );
    field.print_field();

    let start = Instant::now();
    
    let (_solved, resolved) = field.solve();

    println!("solving took {:?}", start.elapsed());

    for (tile, &value) in resolved.iter() {
        field.field.set(tile.0, tile.1, match value {
            1 => MINE,
            0 => SAFE,
            _ => UNKNOWN,
        });
    }

    field.print_field();
}