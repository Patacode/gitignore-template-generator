use clap::CommandFactory;
use clap::Parser;

use gitignore_template_generator::get_call_to_gitignore_template_service;
use gitignore_template_generator::validator::validate_no_commas;

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
        required = true,
        value_parser = validate_no_commas,
        help = "A non-empty list of existing gitignore template names"
    )]
    template_names: Vec<String>,

    #[arg(
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue,
        help = "Print author"
    )]
    show_author: bool,
}

fn main() {
    let args = Args::parse();

    if args.show_author {
        let cmd = Args::command();
        if let Some(author) = cmd.get_author() {
            println!("{author}");
        } else {
            println!("Author information not available.");
        }
        return;
    }

    let args = args.template_names.join(",");
    let body = get_call_to_gitignore_template_service(&args);

    print!("{body}");
}
