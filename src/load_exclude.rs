use std::{error::Error, io::BufRead, path::PathBuf};

use crate::config::get_config_value;

pub(crate) fn load_exclude() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut exclude: Vec<Vec<u8>> = Vec::new();

    let data_path = get_config_value("data_path")?;
    let data_path = data_path.as_str().expect("Failed to get data path");

    let exclude_path = PathBuf::from(data_path).join("exclude.dic");

    let exclude_file = std::fs::File::open(exclude_path)?;

    for line in std::io::BufReader::new(exclude_file).lines() {
        let line = line?;
        let line = line.trim();
        let line = line.as_bytes().to_vec();
        exclude.push(line);
    }

    Ok(exclude)
}
