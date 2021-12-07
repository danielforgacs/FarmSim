use rand::prelude::*;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;

struct Job {
    // id: i32,
    frames: i32,
    // total_frames: i32,
    // chunk_size: i32,
    task_count: i32,
}

struct Farm {
    jobs: Vec<Job>,
    cpus: i32,
    free_cpus: i32,
}

#[derive(Serialize, Deserialize)]
struct Config {
    repetitions: i32,
    max_cycles: i32,
    cpus: i32,
    job_count: i32,
    min_frames: i32,
    max_frames: i32,
    min_chunk_size: i32,
    max_chunk_size: i32,
    min_frame_cycles: i32,
    max_frame_cycles: i32,
    min_startup_cycles: i32,
    max_startup_cycles: i32,
}

impl Job {
    fn new(mut frames: i32, chunk_size: i32, startup_cycles: i32) -> Self {
        let mut tasks = frames / chunk_size;
        if frames % chunk_size > 0 {
            tasks += 1;
        }
        frames += tasks * startup_cycles;

        Self {
            // id,
            frames,
            // total_frames: frames,
            // chunk_size,
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

    fn render(&mut self) -> f32 {
        let mut log = String::new();
        'mainloop: for job in self.jobs.iter_mut() {
            for _ in 0..job.task_count {
                if self.free_cpus == 0 {
                    break 'mainloop;
                }
                self.free_cpus -= 1;
                job.render();
                // println!(
                //     "job id: {}, frames: {}, chunk size: {}, tasks: {}, frames left: {}",
                //     job.id,
                //     job.total_frames,
                //     job.chunk_size,
                //     job.task_count,
                //     job.frames,
                // );
                if job.frames == 0 {
                    break;
                }
            }
        }
        self.jobs.retain(|x| x.frames > 0);
        let mut usage = 100f32;
        let used = self.cpus - self.free_cpus;
        if used != self.cpus {
            usage = (used as f32 / self.cpus as f32) * 100f32;
        }
        log += format!(
            "{:w1$}%, jobs: {:w2$}",
            usage as u8,
            self.jobs.len(),
            w1=3,
            w2=5,
            )
            .as_str();
        self.free_cpus = self.cpus;
        // println!("{}", log);
        usage
    }
}

impl Config {
    fn new() -> Self {
        println!(
"Missing config file or error in the json data.
Writing default \"farmsimconf.json\" config file."
        );
        let config = Self {
            repetitions: 10,
            max_cycles: 1600,
            cpus: 100,
            job_count: 100,
            min_frames: 500,
            max_frames: 500,
            min_chunk_size: 1,
            max_chunk_size: 1,
            min_frame_cycles: 1,
            max_frame_cycles: 1,
            min_startup_cycles: 1,
            max_startup_cycles: 1,
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
    if config.min_frames < 1 || config.max_frames > 4800 || config.max_frames < config.min_frames {
        println!("Config frame range: {} - {}.", config.min_frames, config.max_frames);
        println!("Good frame range: 1 - 4800.");
        return;
    }
    if config.min_chunk_size < 1 || config.max_chunk_size > 4800 || config.max_chunk_size < config.min_chunk_size {
        println!("config chunk size range: {} - {}", config.min_chunk_size, config.max_chunk_size);
        println!("Good chunk size range:   1 - 4800.");
        return;
    }
    if config.max_cycles < 1 || config.max_cycles > 1600 {
        println!("config cycles: {}", config.max_cycles);
        println!("Good cycles range:   1 - 1600.");
        return;
    }
    if config.min_frame_cycles < 1 || config.max_frame_cycles > 100 || config.max_frame_cycles < config.min_frame_cycles {
        println!("frame cycles: {} - {}", config.min_frame_cycles, config.max_frame_cycles);
        println!("Good cycles range:   1 - 100.");
        return;
    }
    sim(&config);
}

fn sim(config: &Config) {
    let mut rng = thread_rng();
    let root = BitMapBackend::new("farm_usage_plot.png", (1600, 900))
        .into_drawing_area();
    root.fill(&WHITE)
        .expect("can't fill the image.");
    let text_x = 50;
    let text_y = 400;
    let y_diff = 25;
    root.draw(&Text::new(format!("repetitions: {}", config.repetitions), (text_x, text_y + (0 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("max_cycles: {}", config.max_cycles), (text_x, text_y + (1 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("cpus: {}", config.cpus), (text_x, text_y + (2 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("job_count: {}", config.job_count), (text_x, text_y + (3 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frames: {} - {}", config.min_frames, config.max_frames), (text_x, text_y + (4 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("chunk_size: {} - {}", config.min_chunk_size, config.max_chunk_size), (text_x, text_y + (5 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frame_cycles: {} - {}", config.min_frame_cycles, config.max_frame_cycles), (text_x, text_y + (6 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("min_startup_cycles: {} - {}", config.min_startup_cycles, config.max_startup_cycles), (text_x, text_y + (7 * y_diff)), ("Arial", 20).into_font())).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0_f32..config.max_cycles as f32, 0_f32..100_f32).expect("chart build failed.");
    chart.configure_mesh()
        .draw().expect("chart draw failed.");

    for rep in 0..config.repetitions {
        print!("rep: {}", rep);
        let mut farm = Farm::new(config.cpus);

        for _ in 0..config.job_count {
            let mut frames = config.min_frames;
            if config.max_frames != frames {
                frames = rng.gen_range(config.min_frames..=config.max_frames);
            }
            frames *= rng.gen_range(config.min_frame_cycles..=config.max_frame_cycles);
            let mut chunk_size = config.min_chunk_size;
            if config.min_chunk_size < config.max_chunk_size {
                chunk_size = rng.gen_range(config.min_chunk_size..=config.max_chunk_size);
            }
            let startup_cycles = rng.gen_range(config.min_startup_cycles..=config.max_startup_cycles);
            let job = Job::new(frames, chunk_size, startup_cycles);
            farm.submit(job);
        }

        let mut usage_seq: Vec<f32> = Vec::new();
        let mut jobs_done: Vec<f32> = Vec::new();
        let mut finished = false;

        for _ in 0..=config.max_cycles {
            // println!("--- cycle: {} -------------------", cycle);
            let usage = farm.render();
            let done_p = config.job_count as f32 / farm.jobs.len() as f32;
            jobs_done.push(done_p);
            usage_seq.push(usage);
            if finished {
                break;
            }
            if farm.jobs.len() == 0 {
                finished = true;
            }
        }
        print!(" - done. rendering...");
        chart.draw_series(LineSeries::new((
            0..=usage_seq.len() - 1).map(|x| (x as f32, usage_seq[x] as f32)),
        &BLACK,
        )).expect("failed to draw chart");
        chart.draw_series(LineSeries::new((
            0..=usage_seq.len() - 1).map(|x| (x as f32, jobs_done[x] as f32)),
        &GREEN,
        )).expect("failed to draw chart");
        println!(" - done.");
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_job_framge_count() {
        let job = Job::new(1, 1, 0);
        assert_eq!(job.frames, 1);
        let job = Job::new(2, 1, 0);
        assert_eq!(job.frames, 2);
        let job = Job::new(1, 100, 0);
        assert_eq!(job.frames, 1);
        let job = Job::new(1, 1, 1);
        assert_eq!(job.frames, 2);
        let job = Job::new(1, 1, 2);
        assert_eq!(job.frames, 3);
        let job = Job::new(2, 1, 2);
        assert_eq!(job.frames, 6);
        let job = Job::new(10, 1, 10);
        assert_eq!(job.frames, 110);
        let job = Job::new(10, 2, 10);
        assert_eq!(job.frames, 60);
        let job = Job::new(10, 5, 10);
        assert_eq!(job.frames, 30);
        let job = Job::new(10, 10, 10);
        assert_eq!(job.frames, 20);
    }

    #[test]
    fn job_render_takes_one_frame_until_zero() {
        let mut job = Job::new(2, 10, 0);
        assert_eq!(job.frames, 2);
        job.render();
        assert_eq!(job.frames, 1);
        job.render();
        assert_eq!(job.frames, 0);
        job.render();
        assert_eq!(job.frames, 0);
        job.render();
        assert_eq!(job.frames, 0);
    }
}
