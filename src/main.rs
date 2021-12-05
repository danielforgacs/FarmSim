struct Job {
    id: i32,
    frames: i32,
    total_frames: i32,
    chunk_size: i32,
    task_count: i32,
}

struct Farm {
    jobs: Vec<Job>,
    cpus: i32,
    free_cpus: i32,
}

impl Job {
    fn new(id: i32, frames: i32, chunk_size: i32) -> Self {
        let mut tasks = frames / chunk_size;
        if frames % chunk_size > 0 {
            tasks += 1;
        }
        Self {
            id,
            frames,
            total_frames: frames,
            chunk_size,
            task_count: tasks,
        }
    }

    fn render(&mut self) {
        if self.frames > 0 {
            self.frames -= 1;
        }
    }
}

impl Farm {
    fn new(cpus: i32) -> Self {
        Self {
            jobs: Vec::new(),
            cpus,
            free_cpus: cpus,
        }
    }

    fn submit(&mut self, job: Job) {
        self.jobs.push(job);
    }

    fn render(&mut self) {
        'mainloop: for job in self.jobs.iter_mut() {
            for _ in 0..job.task_count {
                if self.free_cpus == 0 {
                    break 'mainloop;
                }
                self.free_cpus -= 1;
                job.render();
                println!("job id: {}, frames: {}, chunk size: {}, tasks: {}, frames left: {}", job.id, job.total_frames, job.chunk_size, job.task_count, job.frames);
                if job.frames == 0 {
                    break;
                }
            }
        }
        self.jobs.retain(|x| x.frames > 0);
        self.free_cpus = self.cpus;
    }
}

fn main() {
    let job1 = Job::new(0, 4, 2);
    let job2 = Job::new(1, 4, 2);
    let mut farm = Farm::new(4);
    farm.submit(job1);
    farm.submit(job2);
    for cycle in 0..=11 {
        println!("--- cycle: {} -------------------", cycle);
        println!(
            "job count: {}",
            &farm.jobs.len()
        );
        farm.render();
    }
}
