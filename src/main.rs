use clap::Parser;
use clap::ValueEnum;
use std::fs;

mod digest;

#[derive(ValueEnum, Debug, Clone)]
enum Action {
    Digest,
    Diff,
}

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long)]
    action: Action,

    #[arg(short, long)]
    folder: Option<String>,

    #[arg(short, long)]
    min_file_size: Option<u64>,

    #[arg(short, long)]
    src: Option<String>,

    #[arg(short, long)]
    dst: Option<String>,
}

/// Generates a script containing `mkdir` and `mv` commands that moves the files in the `dst`
/// folder to match the file structure of the `src` folder.
///
/// This utility is helpful for avoiding file copy (often from a remote computer) when using `rsync`
/// and the files in the `src` folder have been moved:
///
/// 1. It detects a file that have been moved in `src` that already existing in `dst`, and generate
/// a `mv` command to move the file.
///
/// 2. It detects duplication with file size and modified date. So it's assuming `rsync` is copying
/// the file attributes over. E.g., using `rsync -a`.
///
/// 3. By default files smaller than 1MB is ignored, since copying them with `rsync` does not add
/// much time. The threshold can be controlled by the `--min-file-size` flag.
///
/// 4. All symlinks are ignored.
///
/// Command:
/// ```bash
/// sync_mv --action digest --folder source_folder --min-file-size 1000000 > src.json
/// # or: ssh user@host_ip '/home/user/sync_mv --action digest --folder source_folder --min-file-size 1000000' > src.json
/// sync_mv --action digest --folder destination_folder --min-file-size 1000000 > dst.json
/// sync_mv --action diff --src src.json --dst dst.json > diff.txt
/// ```
///
/// The above command generates a diff.txt containing the command to run. You can inspect the file
/// and run it and then optionally sync the folders with `rsync`. E.g.,
///
/// ```bash
/// bash diff.txt
/// rsync -avh source_folder destination_folder
/// ```
fn main() {
    let args = Args::parse();

    match args.action {
        Action::Digest => {
            let min_file_size = args.min_file_size.unwrap_or(1_000_000);
            println!("{}", digest::get(&args.folder.unwrap(), min_file_size));
        }
        Action::Diff => {
            println!(
                "{}",
                digest::diff(
                    &fs::read_to_string(&args.src.unwrap()).unwrap(),
                    &fs::read_to_string(&args.dst.unwrap()).unwrap()
                )
            );
        }
    }
}
