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
        let coords = (place / 3, place % 3 );
        if self.grid[coords.0][coords.1] != Space::None {
            // TODO: create error type
            return Err("Already occupied".into());
        }
        *&mut self.grid[coords.0][coords.1] = player.clone();
        Ok(())
    }
}

#[derive(Debug, Default)]
struct GiantTicTacToe {
    grid: [[TicTacToe; 3]; 3],
}

impl fmt::Display for GiantTicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (line_nb, line) in self.grid.clone().into_iter().enumerate() {
            writeln!(f, "{}   {}  {}", line[0], line[1], line[2])?;
            if line_nb < 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
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
}
