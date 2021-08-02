use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

pub fn load_configuration<T: serde::de::DeserializeOwned>(file_path: PathBuf) -> Result<T, Box<dyn Error>> {
    let file_data = read_file(file_path)?;

    let result = serde_json::from_str::<T>(&file_data)?;
    Ok(result)
}

fn read_file(file_path: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}