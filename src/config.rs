use std::env;

const DEFAULT_MESSAGE: &str = "Earthquake!!! This is an emergency commit";

/// Structure holding the configuration
pub struct Config {
    /// The message used for the commit
    pub commit_message: String
}

impl Config {
    /// Creates a new Config using the arguments
    pub fn new(mut args: env::Args) -> Self {
        // ignoring the first parameter (always the program's full path)
        args.next();

        let commit_message = match args.next() {
            Some(s) => s,
            None => DEFAULT_MESSAGE.to_owned()
        };

        Self { commit_message }
    }
}