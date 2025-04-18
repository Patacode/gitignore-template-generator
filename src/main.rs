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
    let body = GitignoreTemplateGenerator::generate(&args);

    print!("{body}");
}
