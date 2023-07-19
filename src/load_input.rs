use std::collections::HashMap;

use crate::arguments::Input;

pub(crate) fn load_input(input: Input) -> Result<HashMap<String, Vec<Vec<u8>>>, std::io::Error> {
    match input {
        Input::File(path) => {
            let path = std::path::Path::new(&path);
            let file = std::fs::read(path)?;
            let is_crlf = file.iter().any(|&x| x == b'\r');
            let lines: Vec<Vec<u8>> = file
                .split(|&x| x == b'\n')
                .map(|x| {
                    let mut line = x.to_vec();
                    if is_crlf {
                        line.pop();
                    }
                    line
                })
                .collect();

            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

            let mut map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
            map.insert(file_name, lines);
            return Ok(map);
        }
        Input::Folder(path) => {
            let mut map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
            let paths = std::fs::read_dir(path)?;
            for path in paths {
                let path = path?;
                let file = std::fs::read(path.path())?;
                let is_crlf = file.iter().any(|&x| x == b'\r');
                let lines: Vec<Vec<u8>> = file
                    .split(|&x| x == b'\n')
                    .map(|x| {
                        let mut line = x.to_vec();
                        if is_crlf {
                            line.pop();
                        }
                        line
                    })
                    .collect();

                let file_name = path.file_name().to_str().unwrap().to_string();

                map.insert(file_name, lines);
            }

            return Ok(map);
        }
    }
}
