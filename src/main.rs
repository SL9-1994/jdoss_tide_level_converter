use clap::Parser;
use converter::Converter;
use file_reader::FileReader;
use std::{fs, path::PathBuf};

mod converter;
mod file_reader;

#[derive(Parser)]
#[command(version, about = "CLI for converting hourly tide height data from Japan Oceanographic Data Center", long_about = None)]
struct Cli {
    /// Pass the path of the directory where the txt file to be converted is stored.
    #[arg(short, long, value_name = "CONVERT_DIRECTORY_PATH")]
    dir_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(dir_path) = cli.dir_path.as_deref() {
        let file_reader: FileReader = FileReader::new(dir_path.to_path_buf());
        let file_paths = file_reader
            .read_txt_files()
            .expect("Failed to read data from file");

        for file_path in file_paths {
            let txt_data = fs::read_to_string(&file_path).expect("Failed to read data from file");

            let converter = Converter::new();
            let c_data = converter.convert_missing_to_normal(txt_data);
            let result = file_reader.write_files(c_data, &file_path);
            if result.is_ok() {
                println!(
                    "Success: 「{}」 conversion completed successfully",
                    file_path
                );
            } else {
                eprintln!("Error: 「{}」 conversion failed.", file_path)
            }
        }
    }
}
