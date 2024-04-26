use anyhow::Result;
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::stdout;
use tomatrix::*;

pub fn main() -> Result<()> {
    stdout().execute(Clear(ClearType::All))?;

    let mut window = Window::default();
    loop {
        window.draw_next()?;
        std::thread::sleep(std::time::Duration::new(0, rand::random::<u32>() % 1000000));
    }
}
