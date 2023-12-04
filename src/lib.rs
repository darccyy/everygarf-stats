use std::{
    process::Command,
    time::{Duration, Instant},
};

const TEMP_DIR: &str = "/tmp/garfield";
const ATTEMPTS: u32 = 10;
const TIMEOUT: u32 = 10;

pub struct Everygarf {
    pub max: u32,
    pub jobs: u32,
}

impl Everygarf {
    pub fn new(max: u32, jobs: u32) -> Self {
        Self { max, jobs }
    }

    pub fn execute(self) -> Option<Duration> {
        let mut command = Command::new("everygarf");
        command.arg(TEMP_DIR);
        command.arg("--remove-all");
        command.arg("--notify-fail");
        command.args(["--attempts", &ATTEMPTS.to_string()]);
        command.args(["--timeout", &TIMEOUT.to_string()]);
        command.args(["--max", &self.max.to_string()]);
        command.args(["--jobs", &self.jobs.to_string()]);

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
