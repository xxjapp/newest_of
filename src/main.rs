use chrono::prelude::DateTime;
use chrono::Local;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

/// Get newest/oldest files or sub objects of directories by modified time
#[derive(StructOpt, Debug)]
struct Cli {
    /// The files or directories paths to search
    #[structopt(short, long, default_value = "./", parse(from_os_str))]
    paths: Vec<PathBuf>,

    /// The extensions to include for files
    #[structopt(short, long)]
    include_exts: Vec<String>,

    /// The extensions to exclude for files
    #[structopt(short, long)]
    exclude_exts: Vec<String>,

    /// Whether to output directories or not
    #[structopt(short, long)]
    output_directory: bool,

    /// The max result file/directory count
    #[structopt(short, long, default_value = "10")]
    count: i32,

    /// Instead of search newest, search oldest
    #[structopt(short, long)]
    reverse: bool,

    /// TODO: Whether to order output or not
    #[structopt(short, long)]
    unordered: bool,
}

struct Res {
    p: PathBuf,
    m: u64,
}

impl fmt::Debug for Res {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dt = date_time_from_timestamp(self.m).format("%Y-%m-%d %H:%M:%S");

        if let Some(p) = self.p.to_str() {
            write!(f, "{} [{}] {:#?}", self.m, dt, p)
        } else {
            write!(f, "{} [{}] {:#?}", self.m, dt, self.p)
        }
    }
}

fn main() {
    let args = Cli::from_args();
    println!("{:#?}", args);

    // collect results
    let mut results = Vec::new();

    for path in &args.paths {
        handle_path(&path, &args, &mut |res: Res| results.push(res));
    }

    // output results
    for res in results {
        println!("{:#?}", res)
    }
}

fn handle_path(path: &PathBuf, args: &Cli, handle_result: &mut dyn FnMut(Res)) {
    if path.is_file() {
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or("");

        if !filter_extension(ext, &args.include_exts, &args.exclude_exts) {
            return;
        }

        match mtime1(path) {
            Ok(mtime) => handle_result(Res {
                p: path.to_path_buf(),
                m: mtime,
            }),
            Err(error) => println!("{:?}", error),
        }
    } else if path.is_dir() {
        if args.output_directory {
            match mtime1(path) {
                Ok(mtime) => handle_result(Res {
                    p: path.to_path_buf(),
                    m: mtime,
                }),
                Err(error) => println!("{:?}", error),
            }
        }

        match traverse_dir(path, args, handle_result, &mut handle_path2) {
            Ok(_) => (),
            Err(error) => println!("{:?}", error),
        }
    } else {
        println!("{:#?} is neither a file nor a directory", path)
    }
}

fn handle_path2(entry: &DirEntry, args: &Cli, handle_result: &mut dyn FnMut(Res)) {
    let entry_path = entry.path();

    if entry_path.is_file() {
        let ext = entry_path.extension().and_then(OsStr::to_str).unwrap_or("");

        if !filter_extension(ext, &args.include_exts, &args.exclude_exts) {
            return;
        }

        match mtime2(entry) {
            Ok(mtime) => handle_result(Res {
                p: entry_path.to_path_buf(),
                m: mtime,
            }),
            Err(error) => println!("{:?}", error),
        };
    } else if entry_path.is_dir() {
        if args.output_directory {
            match mtime2(entry) {
                Ok(mtime) => handle_result(Res {
                    p: entry_path.to_path_buf(),
                    m: mtime,
                }),
                Err(error) => println!("{:?}", error),
            };
        }
    } else {
        println!("{:#?} is neither a file nor a directory", entry_path)
    }
}

fn filter_extension(ext: &str, include_exts: &Vec<String>, exclude_exts: &Vec<String>) -> bool {
    if !exclude_exts.is_empty() && exclude_exts.contains(&ext.to_string()) {
        return false;
    }

    return include_exts.is_empty() || include_exts.contains(&ext.to_string());
}

// traverse a directory
fn traverse_dir(path: &PathBuf, args: &Cli, handle_result: &mut dyn FnMut(Res), cb: &mut dyn FnMut(&DirEntry, &Cli, &mut dyn FnMut(Res))) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        cb(&entry, args, handle_result);

        if path.is_dir() {
            traverse_dir(&path, args, handle_result, cb)?;
        }
    }

    Ok(())
}

fn mtime1(path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    since_epoch(&path.metadata()?.modified()?)
}

fn mtime2(entry: &DirEntry) -> Result<u64, Box<dyn Error>> {
    since_epoch(&entry.metadata()?.modified()?)
}

// get time value in seconds since epoch
fn since_epoch(t: &SystemTime) -> Result<u64, Box<dyn Error>> {
    Ok(t.duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}

fn date_time_from_timestamp(ts: u64) -> DateTime<Local> {
    DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(ts))
}
