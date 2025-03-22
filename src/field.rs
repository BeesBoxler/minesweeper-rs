use super::tile::Tile;
use rand::Rng;

pub struct Field {
    pub grid: Vec<Vec<Tile>>,

    width: usize,
    height: usize,
    bombs: usize,
}

impl Field {
    pub fn new(width: usize, height: usize, bombs: usize) -> Self {
        let mut grid: Vec<Vec<Tile>> = (0..height)
            .map(|_| (0..width).map(|_| Tile::default()).collect())
            .collect();

        let mut rng = rand::rng();
        let mut remaining_bombs = bombs;
        while remaining_bombs > 0 {
            let index = rng.random_range(0..width * height);
            let cell = &mut grid[index / height][index % height];
            if !cell.is_bomb() {
                cell.make_bomb();
                remaining_bombs -= 1;
            }
        }

        for i in 0..height {
            for j in 0..width {
                if !grid[i][j].is_bomb() {
                    let mut count = 0;

                    'row_loop: for i_offset in -1 as i32..=1 {
                        'column_loop: for j_offset in -1 as i32..=1 {
                            if i == 0 && i_offset == -1 || i == height - 1 && i_offset == 1 {
                                continue 'row_loop;
                            }
                            if j == 0 && j_offset == -1 || j == width - 1 && j_offset == 1 {
                                continue 'column_loop;
                            }
                            let i = (i as i32 + i_offset) as usize;
                            let j = (j as i32 + j_offset) as usize;

                            if grid[i][j].is_bomb() {
                                count += 1
                            }
                        }
                    }

                    grid[i][j].set_count(count);
                }
            }
        }

        Field {
            width,
            height,
            bombs,
            grid,
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\u{2597}{}\u{2596}",
            (0..self.width).map(|_| '\u{2581}').collect::<String>()
        )?;

        self.grid.iter().for_each(|line| {
            write!(f, "\u{2595}").unwrap();
            line.iter().for_each(|tile| write!(f, "{tile}").unwrap());
            writeln!(f, "\u{258F}").unwrap();
        });

        writeln!(
            f,
            "\u{259D}{}\u{2598}",
            (0..self.width).map(|_| '\u{2594}').collect::<String>()
        )
    }
}
