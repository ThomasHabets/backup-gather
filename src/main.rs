//! cargo build &&  ./target/debug/backup-gather . | tar cf - --no-recursion --null --files-from=-  | tar tvf -
use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;

const NO_BACKUP: &str = ".no-backup";
const CARGO_TOML: &str = "Cargo.toml";
const TARGET: &str = "target";

#[derive(Parser)]
#[command(version)]
struct Opt {
    dirs: Vec<std::path::PathBuf>,

    /// No output.
    #[arg(short)]
    n: bool,

    /// Separate output files with newline instead of null.
    #[arg(long)]
    nl: bool,
}

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
        match entry.path().file_name() {
            Some(x) if x == NO_BACKUP => return Ok(Directory::default()),
            Some(x) if x == CARGO_TOML => rust = true,
            Some(_) => {}
            None => panic!("Huh, no file name?"),
        }
        let meta = entry.metadata()?;
        entries.push((entry, meta));
    }
    Ok(Directory {
        rust: rust,
        entries: entries,
    })
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(3)
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    let mut backlog = Vec::new();
    let mut dirs = opt.dirs.clone();

    let mut size = 0u64;
    while let Some(dir) = dirs.pop() {
        let read = read_dir(&dir)?;
        for (e, meta) in read.entries.iter() {
            if meta.is_dir() {
                if !(read.rust && e.file_name() == TARGET) {
                    dirs.push(e.path());
                }
                continue;
            } else {
                size += meta.len();
            }
            backlog.push(e.path());
        }
    }

    {
        use num_format::ToFormattedString;
        let locale = num_format::SystemLocale::default()?;

        let bs = bytesize::ByteSize(size);
        log::info!(
            "Total: {} ({}) in {} entries",
            bs.display().si(),
            bs,
            backlog.len().to_formatted_string(&locale)
        );
    }

    if !opt.n {
        let mut o = std::io::stdout();
        for path in backlog.into_iter() {
            o.write_all(path.as_os_str().as_bytes())?;
            if opt.nl {
                o.write_all(b"\n")?;
            } else {
                o.write_all(b"\0")?;
            }
        }
        o.flush()?;
    }
    Ok(())
}
