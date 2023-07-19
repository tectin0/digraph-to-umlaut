use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const COPY_DATA: &'static str = "data";
const COPY_CONFIG: &'static str = "config.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let out = env::var("PROFILE")?;
    let out_data = PathBuf::from(format!("target/{}/{}", out, COPY_DATA));
    let out_file = PathBuf::from(format!("target/{}", out));

    if out_data.exists() {
        fs::remove_dir_all(&out_data).expect("Failed to remove old build");
    }

    fs::create_dir_all(&out_data).expect("Failed to create build directory");

    copy_dir(COPY_DATA, &out_data).expect("Failed to copy data directory");
    copy_file(COPY_CONFIG, &out_file).expect("Failed to copy config file");

    Ok(())
}

fn copy_dir<P, Q>(from: P, to: Q) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let mut to = to.as_ref().to_path_buf();
        to.push(file_name);
        if path.is_dir() {
            fs::create_dir_all(&to)?;
            copy_dir(&path, &to)?;
        } else {
            fs::copy(&path, &to)?;
        }
    }
    Ok(())
}

fn copy_file<P, Q>(from: P, to: Q) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut to = to.as_ref().to_path_buf();
    to.push(from.as_ref().file_name().unwrap());
    fs::copy(from, to)?;

    Ok(())
}
