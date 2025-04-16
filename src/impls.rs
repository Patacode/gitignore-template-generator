use std::process::exit;

pub fn get_call_to_gitignore_template_service(values: &String) -> String {
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
