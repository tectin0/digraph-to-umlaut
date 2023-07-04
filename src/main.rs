use std::io::Write;

use replace_digraphs::replace_digraphs;

mod arguments;
mod config;
mod load_dictionary;
mod load_input;
mod replace_digraphs;
mod search_dictionary;

fn main() -> std::io::Result<()> {
    let mut arguments = arguments::Arguments::new();
    arguments.collect();

    let dictionary = load_dictionary::load_dictionary();

    let input = load_input::load_input(arguments.input)?;

    replace_digraphs(input, dictionary)?;

    Ok(())
}
