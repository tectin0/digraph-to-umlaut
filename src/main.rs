use std::error::Error;

use load_exclude::load_exclude;
use replace_digraphs::replace_digraphs;

mod arguments;
mod config;
mod constants;
mod load_dictionary;
mod load_exclude;
mod load_input;
mod replace_digraphs;
mod search_dictionary;
mod tree;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = arguments::Arguments::new();
    arguments.collect();

    let dictionary = load_dictionary::load_dictionary().expect("Failed to load dictionary");

    let input = load_input::load_input(arguments.input).expect("Failed to load input");

    let exclude = load_exclude().expect("Failed to load exclude");

    replace_digraphs(input, dictionary, exclude).expect("Failed to replace digraphs");

    Ok(())
}
