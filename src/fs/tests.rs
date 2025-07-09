use std::io::ErrorKind;

use crate::{
    fs::{FileSystemHandler, impls::DirectoryHandler},
    test_helper::{DefaultTestUtils, TestUtils},
};

mod directory_handler {
    use super::*;

    mod fetch_content {
        use super::*;

        mod success {
            use super::*;
            use crate::test_helper::DefaultTestUtils;

            #[test]
            fn it_fetches_file_content() {
                let template_dir = DefaultTestUtils::get_resource_file_path("templates");
                let file_name = "rust.txt";
                let directory_handler = DirectoryHandler::new(&template_dir);

                let expected_content = DefaultTestUtils::load_resource_file("templates/rust.txt");
                let actual_content = directory_handler.fetch_content(file_name);

                assert!(actual_content.is_ok());

                let actual_content = actual_content.unwrap();
                assert_eq!(actual_content, expected_content);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_fails_if_any_file_system_error() {
                let template_dir = DefaultTestUtils::get_resource_file_path("templates");
                let directory_handler = DirectoryHandler::new(&template_dir);

                let expected_error_kind = ErrorKind::IsADirectory;
                let actual_error = directory_handler.fetch_content("dummy");

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error.kind(), expected_error_kind);
            }
        }
    }

    mod list_files {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn it_list_directory_files() {
                let template_dir = DefaultTestUtils::get_resource_file_path("templates");
                let directory_handler = DirectoryHandler::new(&template_dir);

                let expected_list = DefaultTestUtils::to_string_list("python rust");
                let actual_list = directory_handler.list_files();

                assert!(actual_list.is_ok());

                let actual_list = actual_list.unwrap();
                assert_eq!(actual_list, expected_list);
            }
        }

        mod failure {
            use super::*;

            #[test]
            fn it_fails_if_any_file_system_error() {
                let template_dir = DefaultTestUtils::get_resource_file_path("inexistent");
                let directory_handler = DirectoryHandler::new(&template_dir);

                let expected_error_kind = ErrorKind::NotFound;
                let actual_error = directory_handler.list_files();

                assert!(actual_error.is_err());

                let actual_error = actual_error.unwrap_err();
                assert_eq!(actual_error.kind(), expected_error_kind);
            }
        }
    }
}
