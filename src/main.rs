struct Matrix {
    data: Box<[i32]>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn from_array<const R: usize, const C: usize>(matrix: [[i32; C]; R]) -> Self {
        Self {
            data: matrix.into_iter().flatten().collect(),
            rows: R,
            columns: C,
        }
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row * self.columns + col]
    }
}

fn main() {
    let field = Matrix::from_array([
        [-2, -1,  2,  2, -1, -1, -1],
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
            'adj_check: for adj_y in y.saturating_sub(1)..=y.saturating_add(1).min(field.rows) {
                for adj_x in x.saturating_sub(1)..=x.saturating_add(1).min(field.columns) {
                    if adj_y == y && adj_x == x { continue; }
                    
                    let adj_tile_value = field.get(adj_y, adj_x);
                    if (0 <= tile_value && tile_value <= 8) && (adj_tile_value == -1) {
                        numbered_tiles.push((y, x));
                        break 'adj_check;
                    } else if (tile_value == -1) && (0 <= adj_tile_value && adj_tile_value <= 8) {
                        unknown_tiles.push((y, x));
                        break 'adj_check;
                    }
                }
            }
        }
    }
    println!("({}) {:?}", unknown_tiles.len(), unknown_tiles);
    println!("({}) {:?}", numbered_tiles.len(), numbered_tiles);
}
