use anyhow::{Context, Result};
use clap::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
struct Options {
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let files: Result<Vec<File>> = options
        .files
        .iter()
        .map(|f| File::create(f).with_context(|| format!("Failed to create file: {}", f.display())))
        .collect();
    let files = files?;
    let mut rng = rand::thread_rng();
    let mut counter = 0;

    let messages = [
        "Hi!",
        "Frobnicating the foobars",
        "Printer on fire",
        "Self destruct sequence initiated",
        "Relaxen und watschen der blinkenlichten",
    ];
    loop {
        if let Some(mut f) = files.choose(&mut rng) {
            counter += 1;
            let message = messages.choose(&mut rng).unwrap();
            let line = format!("{message} {counter}\n");
            f.write_all(line.as_bytes())
                .context("Failed writing to file")?;
        }

        std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..2000)))
    }
}
