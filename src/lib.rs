use std::{
    process::Command,
    time::{Duration, Instant},
};

const TEMP_DIR: &str = "/tmp/garfield";
const ATTEMPTS: u32 = 10;
const TIMEOUT: u32 = 10;

pub struct Everygarf {
    pub images: u32,
    pub jobs: u32,
    pub cache: bool,
}

impl Everygarf {
    pub fn new(images: u32, jobs: u32, cache: bool) -> Self {
        Self {
            images,
            jobs,
            cache,
        }
    }

    pub fn execute(self) -> Option<Duration> {
        let mut command = Command::new("everygarf");
        command.arg(TEMP_DIR);
        command.arg("--remove-all");
        command.arg("--notify-fail");
        command.args(["--attempts", &ATTEMPTS.to_string()]);
        command.args(["--timeout", &TIMEOUT.to_string()]);
        command.args(["--max", &self.images.to_string()]);
        command.args(["--jobs", &self.jobs.to_string()]);
        if !self.cache {
            command.arg("--no-cache");
        }

        let start_time = Instant::now();
        let output = command.output().expect("Failed to run command");

        if !output.status.success() {
            eprintln!("Failed with code {}", output.status);
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return None;
        }

        let elapsed_time = start_time.elapsed();
        Some(elapsed_time)
    }
}
