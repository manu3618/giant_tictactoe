use itertools::iproduct;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq)]
enum Space {
    #[default]
    None,
    Cross,
    Circle,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match &self {
            Space::None => ' ',
            Space::Cross => 'X',
            Space::Circle => 'O',
        };
        write!(f, "{}", d)
    }
}

impl From<Space> for char {
    fn from(s: Space) -> Self {
        format!("{}", s).chars().next().unwrap()
    }
}

#[derive(Debug, Default, Clone)]
struct TicTacToe {
    grid: [[Space; 3]; 3],
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line_nb, line) in self.grid.clone().into_iter().enumerate() {
            writeln!(f, "{} | {} | {}", line[0], line[1], line[2])?;
            if line_nb < 2 {
                writeln!(f, "--+---+--")?
            }
        }
        Ok(())
    }
}

impl TicTacToe {
    /// Place mark of player in place
    ///
    /// place is numbered as follow:
    ///
    ///     1 | 2 | 3
    ///    ---+---+---
    ///     4 | 5 | 6
    ///    ---+---+---
    ///     7 | 8 | 9
    ///
    fn play(&mut self, player: Space, place: usize) -> Result<(), String> {
        if place == 0 || place > 9 {
            // TODO: create error type
            return Err("Invalid place".into());
        }
        let place = place - 1;
        let coords = (place / 3, place % 3);
        if self.grid[coords.0][coords.1] != Space::None {
            // TODO: create error type
            return Err("Already occupied".into());
        }
        *&mut self.grid[coords.0][coords.1] = player.clone();
        Ok(())
    }

    fn victory(&self) -> Space {
        for p in [Space::Cross, Space::Circle] {
            for c in 0..3 {
                if (0..3).map(|u| &self.grid[c][u]).cloned().all(|x| x == p) {
                    // line
                    return p;
                }
                if (0..3).map(|u| &self.grid[u][c]).cloned().all(|x| x == p) {
                    // column
                    return p;
                }
            }
            if (0..3).map(|u| &self.grid[u][u]).cloned().all(|x| x == p) {
                // first diag
                return p;
            }
            if (0..3)
                .map(|u| &self.grid[u][2 - u])
                .cloned()
                .all(|x| x == p)
            {
                // second diag
                return p;
            }
        }
        Space::None
    }
}

#[derive(Debug, Default)]
struct GiantTicTacToe {
    grid: [[TicTacToe; 3]; 3],
}

impl fmt::Display for GiantTicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_grid(None).unwrap())
    }
}

impl GiantTicTacToe {
    /// Returns the grid to display
    ///
    /// playable: if provided, display possibilities on grid, numbered as follow:
    ///
    ///     1 | 2 | 3
    ///    ---+---+---
    ///     4 | 5 | 6
    ///    ---+---+---
    ///     7 | 8 | 9
    ///
    fn to_grid(&self, playable: Option<usize>) -> Result<String, String> {
        // grids
        let mut grid = empty_giant_grid();

        // data
        for (l, x) in self.grid.iter().enumerate() {
            for (c, g) in x.iter().enumerate() {
                let grid_offset = (l * 12 + 1, c * 12 + 1);
                for (line, column) in iproduct!(0..3, 0..3) {
                    let coords = (4 * line + grid_offset.0, 4 * column + grid_offset.1);
                    grid[coords.0][coords.1] = g.grid[line][column].clone().into();
                }
            }
        }

        // numbers
        if let Some(g) = playable {
            if g < 10 && g > 0 {
                let g = g - 1;
                let offset = (g / 3 * 12, g % 3 * 12);
                for n in 0..9 {
                    let c = ((n / 3) * 4, (n % 3) * 4);
                    grid[offset.0 + c.0][offset.1 + c.1] =
                        format!("{}", n + 1).chars().next().unwrap();
                }
            }
        }

        // collect
        let mut lines: Vec<String> = Vec::new();
        for line in grid.iter() {
            lines.push(line.iter().collect())
        }
        let result = lines.join("\n");
        Ok(result)
    }

    fn victories(&self) -> TicTacToe {
        let mut t = TicTacToe::default();
        for (a, b) in iproduct!(0..3, 0..3) {
            t.grid[a][b] = self.grid[a][b].victory();
        }
        t
    }

    fn victory(&self) -> Space {
        self.victories().victory()
    }
}

fn empty_giant_grid() -> [[char; 3 * 12]; 3 * 12] {
    let mut grid = [[' '; 3 * 12]; 3 * 12];
    for (x, y) in iproduct!(0..3, 0..3) {
        let offset = (x * 12, y * 12);

        // lines
        for l in 0..2 {
            for c in 0..11 {
                grid[3 + l * 4 + offset.0][c + offset.1] = '-'
            }
        }

        // columns
        for c in 0..2 {
            for l in 0..11 {
                grid[l + offset.0][3 + c * 4 + offset.1] = '|'
            }
        }

        // crosses
        grid[offset.0 + 3][offset.1 + 3] = '+';
        grid[offset.0 + 7][offset.1 + 3] = '+';
        grid[offset.0 + 3][offset.1 + 7] = '+';
        grid[offset.0 + 7][offset.1 + 7] = '+';
    }
    grid
}

fn main() {
    println!("Hello, world!");
    let mut game = TicTacToe::default();
    println!("{}", &game);
    let _ = game.play(Space::Cross, 4);
    let _ = game.play(Space::Cross, 5);
    let _ = game.play(Space::Circle, 1);
    let _ = game.play(Space::Circle, 9);
    println!("{}", &game);

    println!("giant");
    let mut giant = GiantTicTacToe::default();
    giant.grid[0][0] = game.clone();
    // println!("{}", &giant);
    let choice = 6;
    println!("{choice}");
    println!("{}", &giant.to_grid(Some(choice)).unwrap());
}
