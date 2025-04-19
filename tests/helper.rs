use std::fs;

use gitignore_template_generator::constant;

pub fn load_expectation_file_as_string(expectation_file_name: &str) -> String {
    let expectation_file_path = format!(
        "{}/{expectation_file_name}.txt",
        constant::path::TEST_EXPECTATIONS
    );

    fs::read_to_string(expectation_file_path)
        .expect(constant::error_messages::FILE_READ_TO_STRING_FAILURE)
}
