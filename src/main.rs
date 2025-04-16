use clap::Parser;

use std::{env, process::exit};

#[derive(Parser, Debug)]
#[command(version, author, long_about = None)]
#[command(about = "A binary crate to generate templates for .gitignore files")]
#[command(help_template = "\
{before-help}
{usage-heading} {usage}

{about-with-newline}
{all-args}{after-help}

Version: {version}
Author: {author}
")]
struct Args {
    #[arg(
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue,
        help = "Print author"
    )]
    show_author: bool,
}

fn get_call_to_gitignore_template_service(values: &String) -> String {
    let url =
        format!("https://www.toptal.com/developers/gitignore/api/{values}");

    ureq::get(url)
        .call()
        .unwrap_or_else(|error| {
            eprintln!("An error occurred during the API call: {error}");
            exit(2);
        })
        .body_mut()
        .read_to_string()
        .unwrap_or_else(|_| {
            eprintln!("An error occurred during body parsing");
            exit(3);
        })
}

fn parse_vec_to_comma_separated_list(values: &[String]) -> String {
    values.join(",")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "You must give at least 1 CLI arg but you gave {}",
            args.len() - 1
        );
        exit(1);
    }

    let args = parse_vec_to_comma_separated_list(&args[1..]);
    let body = get_call_to_gitignore_template_service(&args);

    print!("{body}");
}
