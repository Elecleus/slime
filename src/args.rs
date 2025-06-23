use crate::process::Direction;
use anyhow::{Result, bail};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Direction in which given images should be splice together: 'd', 'u', 'l' or 'r'
    #[arg(short, long, default_value_t = 'd')]
    direction: char,

    /// Input images, keep blank for pipe in
    #[arg(required = false)]
    images: Vec<String>,
}

pub struct MyArgs {
    pub direction: Direction,
    pub images: Option<Vec<String>>,
}

pub fn get_args() -> Result<MyArgs> {
    let args = Args::parse();
    let mut my_args = MyArgs {
        direction: Direction::Down,
        images: None,
    };

    my_args.direction = match args.direction {
        'd' => Direction::Down,
        'u' => Direction::Up,
        'l' => Direction::Left,
        'r' => Direction::Right,
        _ => bail!("Direction must be one of 'd', 'u', 'l' and 'r'!"),
    };

    my_args.images = match args.images.len() {
        0 => None,
        _ => Some(args.images),
    };

    return Ok(my_args);
}
