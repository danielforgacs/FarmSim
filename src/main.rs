struct Job {
    frames: i32,
    done: bool,
}

struct Farm {
    jobs: Vec<Job>,
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

impl Farm {
    fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    fn submit(&mut self, job: Job) {
        self.jobs.push(job);
    }

    fn render(&mut self) {
        for job in self.jobs.iter_mut() {
            job.render();
        }
    }
}

fn main() {
    let job = Job::new();
    let mut farm = Farm::new();
    farm.submit(job);
    for cycle in 0..=11 {
        println!("cycle: {}", cycle);
        farm.render();
    }
}
