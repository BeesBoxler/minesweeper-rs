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
// #region Hell 
        for i in 0..height {
            for j in 0..width {
                if !grid[i][j].is_bomb() {
                    let mut count = 0;
                    if i > 0 && i < height - 1 && j > 0 && j < width - 1 {
                        for i_offset in -1 as i32..=1 {
                            for j_offset in -1 as i32..=1 {
                                if grid[(i as i32 + i_offset) as usize]
                                    [(j as i32 + j_offset) as usize]
                                    .is_bomb()
                                {
                                    count += 1
                                }
                            }
                        }
                    } else if i == 0 {
                        if j == 0 {
                            if grid[i][j+1].is_bomb() { count += 1}
                            if grid[i+1][j].is_bomb() { count += 1}
                            if grid[i+1][j+1].is_bomb() { count += 1}
                        } else if j == width - 1 {
                            if grid[i][j-1].is_bomb() { count += 1}
                            if grid[i+1][j-1].is_bomb() { count += 1}
                            if grid[i+1][j].is_bomb() { count += 1}
                        } else {
                            if grid[i][j-1].is_bomb() { count += 1}
                            if grid[i][j+1].is_bomb() { count += 1}
                            if grid[i+1][j-1].is_bomb() { count += 1}
                            if grid[i+1][j].is_bomb() { count += 1}
                            if grid[i+1][j+1].is_bomb() { count += 1}
                        }
                    }else if i == height - 1 {
                        if j == 0 {
                            if grid[i][j+1].is_bomb() { count += 1}
                            if grid[i-1][j].is_bomb() { count += 1}
                        } else if j == width - 1 {
                            if grid[i][j-1].is_bomb() { count += 1}
                            if grid[i-1][j-1].is_bomb() { count += 1}
                            if grid[i-1][j].is_bomb() { count += 1}
                        } else {
                            if grid[i][j-1].is_bomb() { count += 1}
                            if grid[i][j+1].is_bomb() { count += 1}
                            if grid[i-1][j-1].is_bomb() { count += 1}
                            if grid[i-1][j].is_bomb() { count += 1}
                            if grid[i-1][j+1].is_bomb() { count += 1}
                        }
                    }else if j == 0 {
                        if grid[i-1][j].is_bomb() { count += 1}
                        if grid[i-1][j+1].is_bomb() { count += 1}
                        if grid[i][j+1].is_bomb() { count += 1}
                        if grid[i+1][j].is_bomb() { count += 1}
                        if grid[i+1][j+1].is_bomb() { count += 1}
                    }else if j == width -1 {
                        if grid[i-1][j-1].is_bomb() { count += 1}
                        if grid[i-1][j].is_bomb() { count += 1}
                        if grid[i][j-1].is_bomb() { count += 1}
                        if grid[i+1][j-1].is_bomb() { count += 1}
                        if grid[i+1][j].is_bomb() { count += 1}
                    }
                    
                    grid[i][j].set_count(count);
                }
            }
        }
// #endregion Hell
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
