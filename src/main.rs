//
// get newest modified time of files in a folder
//
// Usage
//      # newest_of [directory] [extensions]
//
// Examples:
//      # 1. search newest file in current directory
//      # newest_of
//
//      # 2. search newest file in current directory
//      # newest_of ./
//
//      # 3. search newest file in /tmp directory
//      # newest_of /tmp
//
//      # 4. search newest go file in current directory
//      # newest_of ./ go
//
//      # 5. search newest go and json file in current directory
//      # newest_of ./ go json
//

use chrono::prelude::DateTime;
use chrono::Local;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    let default_path = String::from(".");
    let default_exts = Vec::new();

    let args: Vec<String> = env::args().collect();
    let path0 = args.get(1).unwrap_or(&default_path);
    let exts0 = if args.len() > 2 {
        &args[2..]
    } else {
        &default_exts
    };
    let path = Path::new(path0);

    if path.is_file() {
        match mtime1(path) {
            Ok(mtime) => println!(
                "{:#?}",
                Res {
                    p: PathBuf::from(path0),
                    m: mtime,
                }
            ),
            Err(error) => println!("{:?}", error),
        }
    } else {
        let mut newest = Res {
            p: PathBuf::from(""),
            m: 0,
        };

        let mut cb = |entry: &DirEntry| {
            let mut include = false;

            if exts0.len() > 0 {
                for ext0 in exts0 {
                    if let Some(ext) = entry.path().extension() {
                        include = ext.eq(ext0.as_str());

                        if include {
                            break;
                        }
                    }
                }
            } else {
                include = true
            }

            if include {
                match mtime2(entry) {
                    Ok(mtime) => {
                        if mtime > newest.m {
                            newest.m = mtime;
                            newest.p = entry.path();
                        }
                    }
                    _ => (),
                };
            }
        };

        match traverse(path, &mut cb) {
            Ok(_) => println!("{:#?}", newest),
            Err(error) => println!("{:?}", error),
        }
    }
}

// traverse a directory
fn traverse(path: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            traverse(&path, cb)?;
        } else {
            cb(&entry);
        }
    }

    Ok(())
}

fn mtime1(path: &Path) -> Result<u64, Box<dyn Error>> {
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
