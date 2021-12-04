struct Job {
    frames: i32,
    done: bool,
}

impl Job {
    fn new() -> Self {
        Self {
            frames: 10,
            done: false,
        }
    }

    fn render(&mut self) {
        if !self.get_done() {
            self.frames -= 1;
        }
        if self.frames == 0 {
            self.done = true
        }
    }

    fn get_done(&self) -> bool {
        self.done
    }
}


fn main() {
    let mut job = Job::new();
    let mut cycle = 0;
    while !job.get_done() {
        println!("Cycle: {}", cycle);
        job.render();
        cycle += 1;
    }
}
