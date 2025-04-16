pub fn validate_no_commas(template_name: &str) -> Result<String, String> {
    if template_name.contains(',') {
        Err(String::from("Commas are not allowed in file names."))
    } else {
        Ok(template_name.to_string())
    }
}
