mod char_set;
mod direction;

use std::io;

use crossterm::{cursor, style};
use rand::distr::{Distribution as _, weighted};

use self::{char_set::CharSet, direction::Direction};

pub struct Pipe {
    col: u16,
    row: u16,
    half_row: bool,
    direction: Direction,
    direction_next: Direction,
    direction_change_distr: weighted::WeightedIndex<f64>,
    char_set: CharSet,
    color: style::Color,
}

impl Pipe {
    pub fn new(
        rng: &mut impl rand::Rng,
        cols: u16,
        rows: u16,
        straight_prob: f64,
        color: style::Color,
    ) -> Self {
        let direction = rng.random::<Direction>();
        let straight_prob = straight_prob.clamp(0.0, 1.0);
        let elbow_prob = (1.0 - straight_prob) / 2.0;
        let direction_change_distr =
            weighted::WeightedIndex::new([elbow_prob, elbow_prob, straight_prob])
                .expect("weights should always be valid");
        Self {
            col: rng.random_range(0..cols),
            row: rng.random_range(0..rows),
            half_row: false,
            direction,
            direction_next: direction,
            direction_change_distr,
            char_set: char_set::THICK,
            color,
        }
    }

    pub fn progress(&mut self, rng: &mut impl rand::Rng, cols: u16, rows: u16) {
        if self.half_row {
            self.half_row = false;
            return;
        }

        self.direction = self.direction_next;
        match self.direction_change_distr.sample(rng) {
            0 => self.direction_next.change_left(),
            1 => self.direction_next.change_right(),
            _ => (),
        }

        match self.direction {
            Direction::Up => {
                if self.row == 0 {
                    self.row = rows - 1;
                } else {
                    self.row -= 1;
                }
                if self.direction_next == Direction::Up {
                    self.half_row = true;
                }
            }
            Direction::Down => {
                if self.row >= rows - 1 {
                    self.row = 0;
                } else {
                    self.row += 1;
                }
                if self.direction_next == Direction::Down {
                    self.half_row = true;
                }
            }
            Direction::Left => {
                if self.col == 0 {
                    self.col = cols - 1;
                } else {
                    self.col -= 1;
                }
            }
            Direction::Right => {
                if self.col >= cols - 1 {
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            }
        }
    }

    pub fn draw(&self, w: &mut impl io::Write) -> io::Result<()> {
        let char = if self.half_row {
            if self.direction == Direction::Up {
                self.char_set.down
            } else {
                self.char_set.up
            }
        } else {
            match (self.direction, self.direction_next) {
                (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                    self.char_set.vertical
                }
                (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
                    self.char_set.horizontal
                }
                (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                    self.char_set.up_left
                }
                (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                    self.char_set.up_right
                }
                (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                    self.char_set.down_left
                }
                (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                    self.char_set.down_right
                }
                _ => unreachable!("current and next direction can never be opposites"),
            }
        };

        crossterm::queue!(
            w,
            cursor::MoveTo(self.col, self.row),
            style::SetForegroundColor(self.color),
            style::Print(char),
        )?;

        Ok(())
    }
}
