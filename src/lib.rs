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
    height: usize,
    position: Position,
    symbol: char,
    speed: usize,
    color: Color,
    volatility: f32,
}

fn pick_speed() -> usize {
    rand::random::<usize>() % MAX_SPEED
}

fn pick_color() -> Color {
    COLORS[rand::random::<usize>() % COLORS.len()]
}

fn pick_volatility() -> f32 {
    rand::random::<f32>() % 1.0
}

fn pick_symbol(corpus: &Corpus) -> char {
    let c = corpus[rand::random::<usize>() % corpus.len()];
    if rand::random() {
        return ' ';
    }

    c
}

fn get_corpus() -> Result<Corpus> {
    let mut v = Vec::new();
    std::io::stdin().read_to_end(&mut v)?;

    Ok(v.iter().map(|x| (*x).into()).collect::<Corpus>())
}

impl Data {
    pub fn new(window: &Window) -> Self {
        Self {
            height: window.height,
            position: Position(rand::random::<usize>() % window.width, 0),
            symbol: pick_symbol(&window.corpus),
            speed: pick_speed(),
            color: pick_color(),
            volatility: pick_volatility(),
        }
    }

    pub fn iterate(&mut self) -> bool {
        if (rand::random::<f32>() % 1.0) > self.volatility {
            if rand::random::<bool>() {
                self.position.1 += self.speed;
                if self.position.1 >= (self.height - (rand::random::<usize>() % self.height)) * 2 {
                    return false;
                }
            }
            self.speed = pick_speed();
            self.color = pick_color();
            self.volatility = pick_volatility();
        }

        return true;
    }
}

pub type Corpus = Vec<char>;

#[derive(Debug, Clone)]
pub struct Position(usize, usize);

#[derive(Debug, Clone)]
pub struct Window {
    data: Vec<Data>,
    corpus: Corpus,
    height: usize,
    width: usize,
    last_data: std::time::Instant,
}

impl Default for Window {
    fn default() -> Self {
        Window::from_terminal().expect("Could not get terminal settings")
    }
}

impl Window {
    pub fn new(width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            data: Vec::new(),
            corpus: get_corpus()?,
            height,
            width,
            last_data: std::time::Instant::now(),
        })
    }

    pub fn from_terminal() -> Result<Self> {
        let ws = crossterm::terminal::size()?;
        Self::new((ws.0 - 1).into(), (ws.1 - 1).into())
    }

    pub fn draw_next(&mut self) -> Result<()> {
        let mut newdata = Vec::new();

        for item in &mut self.data {
            std::io::stdout()
                .execute(MoveTo(
                    item.position.0.try_into()?,
                    item.position.1.try_into()?,
                ))?
                .execute(SetForegroundColor(item.color))?
                .execute(Print(&format!("{}", item.symbol)))?;

            if rand::random() {
                if item.iterate() {
                    newdata.push(item.clone())
                }
            } else {
                newdata.push(item.clone())
            }
        }

        self.data = newdata;

        if std::time::Instant::now() - self.last_data > std::time::Duration::new(0, 2500000) {
            self.next_data(rand::random::<usize>() % 5);
            self.last_data = std::time::Instant::now();
        }

        Ok(())
    }

    fn next_data(&mut self, count: usize) {
        for _ in 0..count {
            let data = Data::new(self);
            self.data.push(data.clone());
        }
    }
}
