//
// get newest modified time of files in a folder
//
// Usage examples:
//      # newest_of ./
//      # newest_of /tmp
//

use std::cmp;
use std::env;
use std::error::Error;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("file or folder path not specified");
    let path = Path::new(path);

    if path.is_file() {
        match mtime1(path) {
            Ok(newest) => println!("{}", newest),
            Err(error) => println!("{:?}", error),
        }
    } else {
        let mut newest = 0;

        let mut cb = |entry: &DirEntry| {
            match mtime2(entry) {
                Ok(mtime) => newest = cmp::max(newest, mtime),
                _ => return,
            };
        };

        match traverse(path, &mut cb) {
            Ok(_) => println!("{}", newest),
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
