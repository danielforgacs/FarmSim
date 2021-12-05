use rand::prelude::*;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
struct Config {
    job_count: i32,
    min_frames: i32,
    max_frames: i32,
    min_chunk_size: i32,
    max_chunk_size: i32,
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
                println!(
                    "job id: {}, frames: {}, chunk size: {}, tasks: {}, frames left: {}",
                    job.id,
                    job.total_frames,
                    job.chunk_size,
                    job.task_count,
                    job.frames,
                );
                if job.frames == 0 {
                    break;
                }
            }
        }
        self.jobs.retain(|x| x.frames > 0);
        self.free_cpus = self.cpus;
    }
}

impl Config {
    fn new() -> Self {
        println!(
"Missing config file or error in the json data.
Writing default \"farmsimconf.json\" config file."
        );
        let config = Self {
            job_count: 1,
            min_frames: 1,
            max_frames: 1,
            min_chunk_size: 1,
            max_chunk_size: 1,
        };
        let json: String = serde_json::to_string(&config).expect("Can't serialize default config.");
        std::fs::write("farmsimconf.json", json).expect("Can't write default json config.");
        config
    }
}
fn main() {
    let config: Config = match std::fs::read_to_string("farmsimconf.json") {
        Ok(jsontext) => match serde_json::from_str(&jsontext) {
            Ok(config) => config,
            Err(_) => Config::new(),
        },
        Err(_) => Config::new(),
    };
    if config.job_count < 1 || config.job_count > 1000 {
        println!("Config job count: {}", config.job_count);
        println!("Good job count: 1 - 1000.");
        return;
    }
    if config.min_frames < 1 || config.max_frames > 1000 || config.max_frames < config.min_frames {
        println!("Config frame range: {} - {}.", config.min_frames, config.max_frames);
        println!("Good frame range: 1 - 1000.");
        return;
    }
    if config.min_chunk_size < 1 || config.max_chunk_size > 1000 || config.max_chunk_size < config.min_chunk_size {
        println!("config chunk size range: {} - {}", config.min_chunk_size, config.max_chunk_size);
        println!("Good chunk size range:   1 - 1000.");
        return;
    }
    sim(&config);
}

fn sim(config: &Config) {
    let mut rng = thread_rng();
    let mut farm = Farm::new(4);

    for id in 0..config.job_count {
        let mut frames = config.min_frames;
        if config.max_frames != frames {
            frames = rng.gen_range(config.min_frames..=config.max_frames);
        }
        let mut chunk_size = config.min_chunk_size;
        if config.min_chunk_size < config.max_chunk_size {
            chunk_size = rng.gen_range(config.min_chunk_size..=config.max_chunk_size);
        }
        let job = Job::new(id, frames, chunk_size);
        farm.submit(job);

    }

    for cycle in 0..=11 {
        println!("--- cycle: {} -------------------", cycle);
        println!(
            "job count: {}",
            &farm.jobs.len()
        );
        farm.render();
    }
}
