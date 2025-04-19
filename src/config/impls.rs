use std::process::exit;

use clap::{CommandFactory, Parser};

use crate::{
    config::{Args, ArgsParser},
    constant,
};

pub struct DefaultArgsParser;

impl ArgsParser for DefaultArgsParser {
    fn parse() -> Args {
        let args = Args::parse();

        if args.show_author {
            let cmd = Args::command();
            if let Some(author) = cmd.get_author() {
                println!("{author}");
            } else {
                println!(
                    "{}",
                    constant::error_messages::AUTHOR_INFOS_NOT_AVAILABLE
                );
            }

            exit(constant::exit_status::SUCCESS);
        }

        args
    }
}
