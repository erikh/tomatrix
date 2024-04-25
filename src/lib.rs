use anyhow::Result;
use crossterm::style::Color::{self, DarkGreen, DarkGrey, Green, White};
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetForegroundColor},
    ExecutableCommand,
};
use std::io::Read;

pub const MAX_SPEED: usize = 4;

lazy_static::lazy_static! {
    pub static ref COLORS: [Color; 4] = [DarkGreen, DarkGrey, Green, White];
}

#[derive(Debug, Clone)]
pub struct Data {
    position: Position,
    symbol: char,
    speed: usize,
    color: Color,
    volatility: f32,
}

impl Data {
    pub fn new(symbol: char, window: Window) -> Self {
        Self {
            position: Position(rand::random::<usize>() % window.width, window.height),
            symbol,
            speed: rand::random::<usize>() % MAX_SPEED,
            color: next_color(),
            volatility: rand::random::<f32>() % 1.0,
        }
    }

    pub fn iterate(&mut self) {
        if (rand::random::<f32>() % 1.0) > self.volatility {}
    }
}

pub type Corpus = Vec<char>;

#[derive(Debug, Clone)]
pub struct Position(usize, usize);

#[derive(Debug, Clone)]
pub struct Window {
    buffer: Vec<Data>,
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
        let data = self.next_data(corpus);
        let color = next_color();

        std::io::stdout()
            .execute(MoveTo(
                data.position.0.try_into()?,
                data.position.1.try_into()?,
            ))?
            .execute(SetForegroundColor(color))?
            .execute(Print(&format!("{}", data.symbol)))?;
        Ok(())
    }

    fn next_data(&self, corpus: Corpus) -> Data {
        Data::new(corpus[rand::random::<usize>() % corpus.len()], self.clone())
    }
}

fn next_color() -> Color {
    COLORS[rand::random::<usize>() % COLORS.len()]
}

fn get_corpus() -> Result<Corpus> {
    let mut v = Vec::new();
    std::io::stdin().read_to_end(&mut v)?;

    Ok(v.iter().map(|x| (*x).into()).collect::<Corpus>())
}
