use anyhow::Result;
use crossterm::style::Color::{self, DarkGreen, DarkGrey, Green, White};
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetForegroundColor},
    ExecutableCommand,
};
use std::io::Read;

lazy_static::lazy_static! {
    pub static ref COLORS: [Color; 4] = [DarkGreen, DarkGrey, Green, White];
}

// stdout()
//     .execute(MoveTo(x as u16, y as u16))?
//     .execute(SetForegroundColor(self.grid[y][x].color()))?
//     .execute(Print(&format!("{}", self.grid[y][x].character())))?;
//
pub type Corpus = Vec<char>;

pub struct Position(usize, usize);

pub struct Window {
    buffer: Vec<char>,
    height: usize,
    width: usize,
}

impl Window {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            buffer: Vec::with_capacity(height * width),
        }
    }

    pub fn from_terminal() -> Result<Self> {
        let ws = crossterm::terminal::window_size()?;
        Ok(Self::new(ws.height.into(), ws.width.into()))
    }

    pub fn draw_next(&self, corpus: Corpus) -> Result<()> {
        let ch = next_char(corpus);
        let pos = self.next_position();
        let color = next_color();

        std::io::stdout()
            .execute(MoveTo(pos.0.try_into()?, pos.1.try_into()?))?
            .execute(SetForegroundColor(color))?
            .execute(Print(&format!("{}", ch)))?;
        Ok(())
    }

    fn next_position(&self) -> Position {
        Position(
            rand::random::<usize>() % self.width,
            rand::random::<usize>() % self.height,
        )
    }
}

fn next_char(corpus: Corpus) -> char {
    corpus[rand::random::<usize>() % corpus.len()]
}

fn next_color() -> Color {
    COLORS[rand::random::<usize>() % COLORS.len()]
}

fn get_corpus() -> Result<Corpus> {
    let mut v = Vec::new();
    std::io::stdin().read_to_end(&mut v)?;

    Ok(v.iter().map(|x| (*x).into()).collect::<Corpus>())
}
