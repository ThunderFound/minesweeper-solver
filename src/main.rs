const UNKNOWN: i32 = -1;
const MINE: i32 = -2;
const SAFE: i32 = -3;

struct Matrix {
    data: Box<[i32]>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: vec![0; rows * columns].into_boxed_slice(),
            rows: rows,
            columns: columns,
        }
    }

    fn from_array<const R: usize, const C: usize>(matrix: [[i32; C]; R]) -> Self {
        Self {
            data: matrix.into_iter().flatten().collect(),
            rows: R,
            columns: C,
        }
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        self.data[row * self.columns + col] = value;
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.columns + col]
    }
}

fn print_augmented_matrix(matrix: &Matrix) {
    for y in 0..matrix.rows {
        for x in 0..matrix.columns {
            let n = matrix.get(y, x);
            if x == matrix.columns - 1 {
                println!("| {n}");
            } else {
                print!(" {n} ");
            }
        }
    }
}

fn main() {
    let field = Matrix::from_array([
        [-4, -1,  2,  2, -1, -1, -1],
        [-1, -1, -1, -1,  2,  3, -1],
        [-1,  2, -1, -1, -1,  4, -1],
        [ 2, -1, -1,  2, -1, -1,  2],
        [-1,  2, -1, -1, -1, -1, -1],
        [-1,  2,  3, -1,  3, -1,  3],
        [-1, -1, -1,  2, -1, -1, -1],
    ]);

    let mut unknown_tiles: Vec<(usize, usize)> = Vec::new();
    let mut numbered_tiles: Vec<(usize, usize)> = Vec::new();
    for y in 0..field.rows {
        for x in 0..field.columns {
            let tile_value = field.get(y, x);
            'adj_check: for adj_y in y.saturating_sub(1)..=(y + 1).min(field.rows - 1) {
                for adj_x in x.saturating_sub(1)..=(x + 1).min(field.columns - 1) {
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
            for adj_x in x.saturating_sub(1)..=(x + 1).min(field.columns - 1) {
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

        matrix.set(numbered_index, matrix.columns - 1, tile_value);
    }
    print_augmented_matrix(&matrix);
}
