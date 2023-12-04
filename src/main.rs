use std::fs;
use std::io::Write;
use std::path::Path;

use everygarf_stats::Everygarf;

fn main() {
    let output_file = "everygarf.csv";
    let images_values = [
        1, 5, 10, 20, 30, 50, 100, 200, 300, 500, 1_000, 5_000, 1_500,
    ];
    let jobs_min = 1;
    let jobs_max = 50;

    let mut rows = Vec::new();

    if Path::new(output_file).exists() {
        fs::write(output_file, "").expect("Failed to truncate file");
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
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
    for (i, jobs) in (jobs_min..=jobs_max).enumerate() {
        let mut cells = Vec::new();

        write_file!("{:-7}", jobs);
        for (j, images) in images_values.iter().enumerate() {
            let Some(time) = Everygarf::new(*images, jobs as u32).execute() else {
                return;
            };
            let time = time.as_secs_f32();
            let time = (time * 10.0).round() / 10.0;

            let percent = 100.0 * (i * images_values.len() + j) as f32
                / (images_values.len() * (1 + jobs_max - jobs_min)) as f32;
            println!("{percent:-7.1}% {jobs:-7}  {images:-7}  {time:-7}s");

            write_file!(", {:-7}", time);
            cells.push(time);
        }
        write_file!("\n");

        rows.push(cells);
    }
}
