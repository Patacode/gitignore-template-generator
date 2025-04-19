use std::process::exit;

use clap::{CommandFactory, Parser};

use crate::config::{Args, ArgsParser};

pub struct DefaultArgsParser;

impl ArgsParser for DefaultArgsParser {
    fn parse() -> Args {
        let args = Args::parse();

        if args.show_author {
            let cmd = Args::command();
            if let Some(author) = cmd.get_author() {
                println!("{author}");
            } else {
                println!("Author information not available.");
            }

            exit(0);
        }

        args
    }
}
