#[derive(Debug)]
pub(crate) enum Input {
    File(String),
    Folder(String),
}

impl Default for Input {
    fn default() -> Self {
        Input::File(String::new())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Arguments {
    pub(crate) input: Input,
}

impl Arguments {
    pub(crate) fn new() -> Self {
        Arguments {
            input: Input::default(),
        }
    }

    pub(crate) fn collect(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        if args.len() == 1 {
            return;
        }

        let mut input: String = String::new();

        for (index, arg) in args.iter().enumerate() {
            if index == 0 {
                continue;
            }

            match arg.as_str() {
                "-i" | "--input" => {
                    input = args[index + 1].clone();
                }
                _ => {}
            }
        }

        if input.is_empty() {
            println!("No input specified");
            return;
        }

        match std::fs::metadata(&input) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    self.input = Input::Folder(input);
                } else if metadata.is_file() {
                    self.input = Input::File(input);
                }
            }
            Err(_) => {}
        }
    }
}
