use std::{ffi::OsString, process::exit};

use clap::{CommandFactory, Parser};

use crate::{
    config::{Args, ArgsParser},
    constant,
    http_client::ProgramError,
};

pub struct DefaultArgsParser;

impl DefaultArgsParser {
    fn parse_author_option(args: &Args) -> Option<ProgramError> {
        if args.show_author {
            let cmd = Args::command();
            let message = String::from(match cmd.get_author() {
                Some(author) => author,
                None => constant::error_messages::AUTHOR_INFOS_NOT_AVAILABLE,
            });

            Some(ProgramError {
                message,
                exit_status: constant::exit_status::SUCCESS,
            })
        } else {
            None
        }
    }
}

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

    fn try_parse(
        args: impl IntoIterator<Item = OsString>,
    ) -> Result<Args, ProgramError> {
        match Args::try_parse_from(args) {
            Ok(parsed_args) => match Self::parse_author_option(&parsed_args) {
                Some(error) => Err(error),
                None => Ok(parsed_args),
            },
            Err(error) => Err(ProgramError {
                message: error.render().to_string(),
                exit_status: error.exit_code(),
            }),
        }
    }
}
