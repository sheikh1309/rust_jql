use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchOptions {
    pub query: String
}

const REQUIRED_ARGS_COUNT: usize = 1;

pub struct SearchOptionsBuilder;

impl SearchOptionsBuilder {
    pub fn parse_piped(args: Vec<String>) -> Result<SearchOptions, String> {
        if args.len() < REQUIRED_ARGS_COUNT {
            return Err(format!(
                "Expected at least {} arguments, got {}.",
                REQUIRED_ARGS_COUNT,
                args.len()
            ));
        }

        let mut args = args.iter();
        args.next(); // Skip the first argument.

        let options = SearchOptions {
            query: match args.next() {
                Some(arg) => arg.clone(),
                None => String::new(),
            }
        };

        Ok(options)
    }

    pub fn parse_params(args: Vec<String>) -> Result<(SearchOptions, String), String> {
        if args.len() < REQUIRED_ARGS_COUNT {
            return Err(format!(
                "Expected at least {} arguments, got {}.",
                REQUIRED_ARGS_COUNT,
                args.len()
            ));
        }

        let mut args = args.iter();
        args.next(); // Skip the first argument.

        let query = match args.next() {
            Some(arg) => arg.clone(),
            None => String::new(),
        };

        let file_path = match args.next() {
            Some(arg) => arg.clone(),
            None => String::new(),
        };

        let file_content = fs::read_to_string(file_path).expect("Something went wrong reading the file");

        let options = SearchOptions { query };

        Ok((options, file_content))
    }
}
