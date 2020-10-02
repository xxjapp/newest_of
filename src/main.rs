use chrono::prelude::DateTime;
use chrono::Local;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};
use structopt::StructOpt;

/// Get newest or oldest objects of input files or directories recursively by modification time
#[derive(StructOpt, Debug)]
struct Cli {
    /// The input files or directories paths to search
    #[structopt(short, long, default_value = "./", parse(from_os_str))]
    input_paths: Vec<PathBuf>,

    /// The extensions to include for files
    #[structopt(short = "e", long)]
    include_exts: Vec<OsString>,

    /// The extensions to exclude for files
    #[structopt(short = "E", long)]
    exclude_exts: Vec<OsString>,

    /// Output directories
    #[structopt(short = "d", long)]
    output_directory: bool,

    /// The max result file/directory count
    #[structopt(short, long, default_value = "10")]
    count: usize,

    /// Instead of search newest, search oldest
    #[structopt(short, long)]
    reverse: bool,

    /// Do not sort output by modification time, count and reverse will be ignored
    #[structopt(short, long)]
    unordered: bool,
}

struct Res {
    p: PathBuf,
    m: Duration,
}

fn main() {
    let args = Cli::from_args();
    // println!("{:#?}", args);

    // collect results
    let mut results = Vec::new();
    let mut total_count = 0;

    for input_path in &args.input_paths {
        match handle_input_path(&input_path, &mut |path: &Path, is_dir: bool| {
            total_count += 1;

            if is_dir {
                if !args.output_directory {
                    return;
                }
            } else {
                let ext = path.extension().unwrap_or_default().to_os_string();

                if !filter_extension(&ext, &args.include_exts, &args.exclude_exts) {
                    return;
                }
            }

            // create res object
            let mtime = match get_mtime(path) {
                Ok(mtime) => mtime,
                Err(error) => {
                    eprintln!("{:#?} - {:?}", error, path);
                    return;
                }
            };

            if args.unordered {
                // output result directly
                output_result(path, mtime);
            } else {
                // add result to results, do sort and filter
                add_result(path, mtime, &mut results, &args);
            }
        }) {
            Ok(_) => (),
            Err(error) => eprintln!("{:#?}", error),
        };
    }

    // output results
    output_results(&results, total_count);
}

fn output_results(results: &Vec<Res>, total_count: u32) {
    for res in results {
        output_result(&res.p, res.m);
    }

    println!("\ntotal count: {}", total_count);
}

fn output_result(path: &Path, mtime: Duration) {
    let dt = date_time_from_timestamp(mtime).format("%Y-%m-%d %H:%M:%S");
    println!("{} {:?}", dt, path);
}

fn add_result(path: &Path, mtime: Duration, results: &mut Vec<Res>, args: &Cli) {
    let is_full = results.len() >= args.count;

    // no need to add
    if is_full {
        if !args.reverse && mtime <= results[0].m {
            return;
        }
        if args.reverse && mtime >= results[0].m {
            return;
        }
    }

    // add to results
    results.push(Res {
        p: path.to_path_buf(),
        m: mtime,
    });

    // sort to put newest or oldest objects at last position
    results.sort_unstable_by(|a, b| match args.reverse {
        true => b.m.cmp(&a.m),
        _ => a.m.cmp(&b.m),
    });

    // remove first item, which is not so new or old
    if is_full {
        results.remove(0);
    }
}

fn handle_input_path(path: &Path, handle_result: &mut dyn FnMut(&Path, bool)) -> io::Result<()> {
    if path.is_dir() {
        traverse_dir(path, handle_result)?;
    } else {
        handle_result(path, false);
    }

    Ok(())
}

// traverse a directory
fn traverse_dir(path: &Path, handle_result: &mut dyn FnMut(&Path, bool)) -> io::Result<()> {
    handle_result(path, true);

    // NOTE: fs::read_dir will read symbolic links, which is difference with linux find command with default options
    for entry in fs::read_dir(path)? {
        let entry_path = &entry?.path();

        if entry_path.is_dir() {
            traverse_dir(entry_path, handle_result)?;
        } else {
            handle_result(entry_path, false);
        }
    }

    Ok(())
}

fn filter_extension(
    ext: &OsString,
    include_exts: &Vec<OsString>,
    exclude_exts: &Vec<OsString>,
) -> bool {
    if !exclude_exts.is_empty() && exclude_exts.contains(ext) {
        return false;
    }

    return include_exts.is_empty() || include_exts.contains(ext);
}

// get modification time value in seconds since epoch
fn get_mtime(path: &Path) -> Result<Duration, Box<dyn Error>> {
    Ok(path.metadata()?.modified()?.duration_since(UNIX_EPOCH)?)
}

fn date_time_from_timestamp(ts: Duration) -> DateTime<Local> {
    DateTime::<Local>::from(UNIX_EPOCH + ts)
}
