use std::process::exit;

use clap::{CommandFactory, Parser};

use gitignore_template_generator::{
    Args, TemplateGenerator, GitignoreTemplateGenerator
};

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
    let result = GitignoreTemplateGenerator::generate(&args);

    match result {
        Ok(body) => print!("{body}"),
        Err(error) => {
            eprintln!("{}", error.message);
            exit(error.exit_status);
        }
    }
}
