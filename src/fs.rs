use std::io::Error;

pub use crate::fs::impls::DirectoryHandler;

mod impls;

#[cfg(test)]
mod tests;

/// File system handler trait to handle common file system operations.
pub trait FileSystemHandler {
    /// Fetches the file content.
    ///
    /// File location is not taken into consideration here. It is up to
    /// the struct implementing this trait to take that decision.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to be fetched
    ///
    /// # Returns
    ///
    /// A result containing the fetched file content, or a
    /// [`std::io::Error`] on error (e.g. file system failure, insufficient
    /// privilege...).
    fn fetch_content(&self, file_name: &str) -> Result<String, Error>;

    /// List files.
    ///
    /// File location is not taken into consideration here. It is up to
    /// the struct implementing this trait to take that decision.
    ///
    /// # Returns
    ///
    /// A result containing the list of files, or a
    /// [`std::io::Error`] on error (e.g. file system failure, insufficient
    /// privilege...).
    fn list_files(&self) -> Result<Vec<String>, Error>;
}
