use clap::Parser;

use crate::validator::validate_no_commas;

#[derive(Parser, Debug)]
#[command(version, author, long_about = None)]
#[command(about = "Generate templates for .gitignore files")]
#[command(help_template = "\
{before-help}
{usage-heading} {usage}

{about-with-newline}
{all-args}{after-help}

Version: {version}
Author: {author}
")]
pub struct Args {
    #[arg(
        required = true,
        value_parser = validate_no_commas,
        help = "A non-empty list of existing gitignore template names"
    )]
    pub template_names: Vec<String>,

    #[arg(
        short = 'a',
        long = "author",
        action = clap::ArgAction::SetTrue,
        help = "Print author"
    )]
    pub show_author: bool,
}