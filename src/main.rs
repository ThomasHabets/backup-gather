use anyhow::Result;

const NO_BACKUP: &str = ".no-backup";
const CARGO_TOML: &str = "Cargo.toml";
const TARGET: &str = "target";

#[derive(Default)]
struct Directory {
    rust: bool,
    entries: Vec<(std::fs::DirEntry, std::fs::Metadata)>,
}

fn read_dir(dir: &std::path::Path) -> Result<Directory> {
    let l = std::fs::read_dir(&dir)?;
    let mut entries = Vec::new();
    let mut rust = false;
    for entry in l.into_iter() {
        let entry = entry?;
        let meta = entry.metadata()?;
        match entry.path().file_name() {
            Some(x) if x == NO_BACKUP => return Ok(Directory::default()),
            Some(x) if x == CARGO_TOML => rust = true,
            Some(_) => {},
            None => panic!("Huh, no file name?"),
        }
        entries.push((entry, meta));
    }
    Ok(Directory {
        rust: rust,
        entries: entries,
    })
}
fn main() -> Result<()> {
    let mut dirs: Vec<std::path::PathBuf> = [".".into()].into();
    while let Some(dir) = dirs.pop() {
        let read = read_dir(&dir)?;
        for (e, meta) in read.entries.iter() {
            println!("{:?} {}", e.file_name(), meta.is_dir());
            if meta.is_dir() {
                if !(read.rust && e.file_name() == TARGET) {
                    dirs.push(e.path());
                }
            }
        }
    }
    Ok(())
}
