use logic::randomizer::Randomizer;
use std::error::Error;

mod assets;
mod data;
mod enums;
mod logic;
mod strings;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rando = Randomizer::new()?;

    rando.shuffle_world_order()?;
    rando.remove_specific_actors()?;

    rando.save()?;

    // check the rando can read what it wrote
    let _rando = Randomizer::new()?;

    Ok(())
}
