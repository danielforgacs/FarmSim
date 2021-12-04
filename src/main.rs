struct Job {
    frames: i32,
}

impl Job {
    fn new() -> Self {
        Self { frames: 10 }
    }

    fn render(&mut self) {
        self.frames -= 1;
    }
}


fn main() {
    let mut job = Job::new();
    job.render();
}
