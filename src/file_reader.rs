use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct FileReader {
    dir_path: PathBuf,
}

impl FileReader {
    /// Creates a new `FileReader` instance with the given directory path.
    ///
    /// # Arguments
    ///
    /// * `dir_path` - The directory path where the files are located.
    ///
    /// # Returns
    ///
    /// * A new `FileReader` instance.
    pub fn new(dir_path: PathBuf) -> Self {
        Self { dir_path }
    }

    /// Reads all the text files in the directory and returns their file names.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of file names as strings, or an error if reading the directory fails.
    pub fn read_txt_files(&self) -> Result<Vec<String>, Box<dyn Error>> {
        fs::read_dir(&self.dir_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.path().extension() == Some(OsStr::new("txt")) {
                    Some(Ok(entry.path().to_string_lossy().into_owned()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Writes the given text data to a file at the specified file path.
    ///
    /// ## Arguments
    ///
    /// * `txt_data` - The text data to write to the file.
    /// * `file_path` - The file path where the file should be created.
    ///
    /// ## Returns
    ///
    /// * A `Result` indicating whether the file was successfully written or not.
    pub fn write_files(&self, txt_data: String, file_path: &String) -> std::io::Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(txt_data.as_bytes())?;

        Ok(())
    }
}
// 48
