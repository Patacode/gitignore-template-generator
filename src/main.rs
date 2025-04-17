use clap::{CommandFactory, Parser};

use gitignore_template_generator::{
    Args, get_call_to_gitignore_template_service,
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
    let body = get_call_to_gitignore_template_service(&args);

    print!("{body}");
}
