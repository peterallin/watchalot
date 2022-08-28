use anyhow::Result;
use clap::Parser;
use notify::Watcher;
use std::{
    collections::HashMap,
    io::{BufRead, Seek},
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
};

#[derive(Debug, clap::Parser)]
struct Options {
    /// The files to watch
    files: Vec<PathBuf>,
}

struct FileState {
    size: u64,
    file: std::fs::File,
}

fn handle_file(file: &Path, state: &mut HashMap<PathBuf, FileState>) -> Result<()> {
    let file_state = state.get_mut(file).unwrap();
    let new_size = file_state.file.metadata()?.size();

    let mut buf_reader = std::io::BufReader::new(file_state.file.try_clone()?);
    buf_reader.seek(std::io::SeekFrom::Start(file_state.size))?;
    file_state.size = new_size;
    let mut line = String::new();
    loop {
        let read_bytes = buf_reader.read_line(&mut line)?;
        if read_bytes == 0 {
            break;
        }
        print!("{}: {}", file.display(), line);
    }

    Ok(())
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, std::time::Duration::from_millis(10))?;
    for file in &options.files {
        watcher.watch(file, notify::RecursiveMode::NonRecursive)?;
    }

    let mut state = HashMap::new();
    for f in options.files {
        let file = std::fs::File::open(&f).unwrap();
        let size = file.metadata()?.size();
        state.insert(f, FileState { file, size });
    }

    loop {
        match rx.recv()? {
            notify::DebouncedEvent::Write(path) => handle_file(&path, &mut state)?,
            _ => {}
        }
    }
}
