use std::io::Write;
use std::time::{Duration, Instant};

use minesweeper_solver::field::Field;
use minesweeper_solver::{UNKNOWN, MINE, SAFE};

fn main() {
    let total_attempts = 1000;
    let mut successful_attempts = 0;
    let mut total_time_for_successful_attempts = Duration::new(0, 0);

    let start = Instant::now();
    for att in 0..total_attempts {
        let mut field = Field::new(30, 16, 99);

        let starting_tile = field.get_starting_tile();
        field.reveal(starting_tile.0, starting_tile.1);
        // field.print_field();
        
        let start = Instant::now();
        loop {
            // let start = Instant::now();
            let (solved, resolved) = field.solve();

            if resolved.len() == 0 {
                let tile = field.get_random_tile();
                let success = field.reveal(tile.0, tile.1);
                if !success {
                    println!("      revealed a mine while randomly opening");
                    break;
                }
            }

            // println!("solving took {:?}", start.elapsed());
            for (tile, &value) in resolved.iter() {
                field.field.set(tile.0, tile.1, match value {
                    1 => MINE,
                    0 => SAFE,
                    _ => UNKNOWN,
                });
            }
            
            // field.print_field();

            for (tile, &value) in resolved.iter() {
                if value == 0 {
                    let success = field.reveal(tile.0, tile.1);
                    if !success {
                        println!("revealed a mine from a solution");
                    }
                }
            }
            // println!();
            // field.print_field();

            if solved {
                // println!("solved");
                total_time_for_successful_attempts += start.elapsed();
                successful_attempts += 1;
                break;
            }
        }
        print!("{att}\r");
        std::io::stdout().flush();
    }
    println!("\ntook {:?} ({:?} per att)", start.elapsed(), start.elapsed() / total_attempts);
    println!("{:?} per successful att", total_time_for_successful_attempts / successful_attempts);
    println!("total attempts: {} | solved fields: {} | success rate: {:.5}%", total_attempts, successful_attempts, successful_attempts as f64 / total_attempts as f64 * 100.0)
}