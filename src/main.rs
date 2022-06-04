mod search;
mod search_command_builder;

use search::{piped_search, in_file_search};
use r3bl_rs_utils::utils::{is_stdin_piped, with};
use std::env::args;
use std::error::Error;
use std::process::exit;
use r3bl_rs_utils::style_error;
use crate::search_command_builder::{SearchOptionsBuilder};

fn main() {
    let args = args().collect::<Vec<String>>();
    with(run(args), |it| match it {
        Ok(()) => exit(0),
        Err(err) => {
            eprintln!("{}: {}", style_error("Problem encountered"), err);
            exit(1);
        }
    });
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match is_stdin_piped() {
        // cat json.json | cargo run ...
        true => piped_search(SearchOptionsBuilder::parse_piped(args)?)?,
        // cargo run .. json.json
        false => {
            let (options, file_content) = SearchOptionsBuilder::parse_params(args)?;
            in_file_search(options, file_content)?
        },
    }
    Ok(())
}