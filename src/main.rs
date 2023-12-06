use std::fs;
use std::io::Write;
use std::path::Path;

use chrono::Utc;
use everygarf_stats::{Everygarf, TEMP_DIR};

fn main() {
    let dir = Path::new("stats");
    let old_dir = Path::new("old");

    if dir.exists() {
        if !old_dir.exists() {
            fs::create_dir_all(old_dir).expect("Failed to make old directory");
        }
        let date = Utc::now().format("%y%m%d-%H%M").to_string();
        fs::rename(dir, old_dir.join(Path::new(&date))).expect("Failed to move old directory");
    }
    fs::create_dir(dir).expect("Failed to create directory");

    run(&dir.join(&Path::new("no-cache.csv")), false);
    run(&dir.join(&Path::new("cache.csv")), true);
}

fn run(filepath: &Path, cache: bool) {
    let images_values = [1, 2, 3, 5, 10, 20, 30, 50, 100, 200, 300, 500, 1_000];
    let jobs_values = [
        // 1, 2, 3, 4,
        5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        30, 35, 40, 45, 50,
    ];

    fs::remove_dir_all(TEMP_DIR).expect("Failed to remove temp image output directory");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath)
        .expect("Failed to open file");

    macro_rules! write_file {
        ( $($tt:tt)* ) => {
            write!(file, $($tt)*).expect("Failed to append to file");
        };
    }

    write_file!("{:-7}", "");
    for images in images_values.clone() {
        write_file!(", {:-7}", images);
    }
    write_file!("\n");

    println!("\x1b[1mprogress    jobs   images      time\x1b[0m");
    for (i, jobs) in jobs_values.iter().enumerate() {
        write_file!("{:-7}", jobs);
        for (j, images) in images_values.iter().enumerate() {
            let Some(time) = Everygarf::new(*images, *jobs, cache).execute() else {
                return;
            };
            let time = time.as_secs_f32() / *images as f32 * 100.0;

            let percent = 100.0 * (i * images_values.len() + j) as f32
                / (images_values.len() * jobs_values.len()) as f32;
            println!("{percent:-7.1}% {jobs:-7}  {images:-7}  {time:-7.0}s");

            write_file!(", {:-7.2}", time);
        }
        write_file!("\n");
    }
}
