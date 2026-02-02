use logic::randomizer::Randomizer;
use std::error::Error;

mod assets;
mod data;
mod enums;
mod logic;
mod strings;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let rando = Randomizer::new()?;
    rando.save()?;

    Ok(())
}
