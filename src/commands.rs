use crate::pacman::Pacman;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

pub struct Dump {}

impl Dump {
    pub fn exec(file: Option<PathBuf>) -> Result<()> {
        let packages = Pacman::get_installed_packages()?;

        match file {
            Some(path) => {
                if path == PathBuf::from("-") {
                    let f = io::stdout().lock();
                    Self::write(f, "<stdout>", &packages)?;
                } else {
                    let path_str = path.to_string_lossy();
                    let f = File::create(&path)
                        .with_context(|| format!("Failed to open `{}`", path_str))?;
                    Self::write(f, &path_str, &packages)?;
                    Self::print_done_message(&path_str);
                }
            }
            None => {
                let path = "pacmanfile";
                let f = File::create(path).with_context(|| format!("Failed to open `{}`", path))?;
                Self::write(f, path, &packages)?;
                Self::print_done_message(path);
            }
        }

        Ok(())
    }

    fn write(file: impl Write, filepath: &str, packages: &[String]) -> Result<()> {
        let mut writer = BufWriter::new(file);

        for line in packages {
            writeln!(writer, "{}", line)
                .with_context(|| format!("Failed to write `{}`", filepath))?;
        }
        writer
            .flush()
            .with_context(|| format!("Failed to write `{}`", filepath))?;

        Ok(())
    }

    fn print_done_message(path: &str) {
        println!("Dumped to `{}`", path);
    }
}
