use rand::prelude::*;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;

const PLOT_WIDTH: u32 = 1600;
const PLOT_HEIGTH: u32 = 900;

struct Job {
    frame_num: i32,
    task_num: i32,
}

struct Farm {
    jobs: Vec<Job>,
    cpu_num: i32,
    free_cpu_num: i32,
}

#[derive(Serialize, Deserialize)]
struct Config {
    repetitions: i32,
    max_render_cycles: i32,
    cpus: i32,
    jobs: i32,
    min_frames: i32,
    max_frames: i32,
    min_task_frames: i32,
    max_task_frames: i32,
    min_frame_render_cycles: i32,
    max_frame_render_cycles: i32,
    min_task_startup_cycles: i32,
    max_task_startup_cycles: i32,
}

impl Job {
    fn new(mut frames: i32, chunk_size: i32, startup_cycles: i32) -> Self {
        let mut tasks = frames / chunk_size;
        if frames % chunk_size > 0 {
            tasks += 1;
        }
        frames += tasks * startup_cycles;

        Self {
            frame_num: frames,
            task_num: tasks,
        }
    }

    fn render(&mut self) {
        if self.frame_num > 0 {
            self.frame_num -= 1;
        }
    }
}

impl Farm {
    fn new(cpus: i32) -> Self {
        Self {
            jobs: Vec::new(),
            cpu_num: cpus,
            free_cpu_num: cpus,
        }
    }

    fn submit(&mut self, job: Job) {
        self.jobs.push(job);
    }

    fn render(&mut self) -> f32 {
        'mainloop: for job in self.jobs.iter_mut() {
            for _ in 0..job.task_num {
                if self.free_cpu_num == 0 {
                    break 'mainloop;
                }
                self.free_cpu_num -= 1;
                job.render();
                if job.frame_num == 0 {
                    break;
                }
            }
        }
        self.jobs.retain(|x| x.frame_num > 0);
        let mut usage = 100f32;
        let used = self.cpu_num - self.free_cpu_num;
        if used != self.cpu_num {
            usage = (used as f32 / self.cpu_num as f32) * 100f32;
        }
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
            max_render_cycles: 1600,
            cpus: 100,
            jobs: 100,
            min_frames: 500,
            max_frames: 500,
            min_task_frames: 1,
            max_task_frames: 1,
            min_frame_render_cycles: 1,
            max_frame_render_cycles: 1,
            min_task_startup_cycles: 1,
            max_task_startup_cycles: 1,
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
    if config.jobs < 1 || config.jobs > 1000 {
        println!("Config job count: {}", config.jobs);
        println!("Good job count: 1 - 1000.");
        return;
    }
    if config.min_frames < 1 || config.max_frames > 4800 || config.max_frames < config.min_frames {
        println!("Config frame range: {} - {}.", config.min_frames, config.max_frames);
        println!("Good frame range: 1 - 4800.");
        return;
    }
    if config.min_task_frames < 1 || config.max_task_frames > 4800 || config.max_task_frames < config.min_task_frames {
        println!("config chunk size range: {} - {}", config.min_task_frames, config.max_task_frames);
        println!("Good chunk size range:   1 - 4800.");
        return;
    }
    if config.max_render_cycles < 1 || config.max_render_cycles > 1600 {
        println!("config cycles: {}", config.max_render_cycles);
        println!("Good cycles range:   1 - 1600.");
        return;
    }
    if config.min_frame_render_cycles < 1 || config.max_frame_render_cycles > 100 || config.max_frame_render_cycles < config.min_frame_render_cycles {
        println!("frame cycles: {} - {}", config.min_frame_render_cycles, config.max_frame_render_cycles);
        println!("Good cycles range:   1 - 100.");
        return;
    }
    sim(&config);
}

fn sim(config: &Config) {
    let mut rng = thread_rng();
    let root = BitMapBackend::new("farm_usage_plot.png", (PLOT_WIDTH, PLOT_HEIGTH))
        .into_drawing_area();
    root.fill(&WHITE)
        .expect("can't fill the image.");
    let text_x = 50;
    let text_y = 400;
    let y_diff = 25;
    root.draw(&Text::new(format!("repetitions: {}", config.repetitions), (text_x, text_y), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("max_cycles: {}", config.max_render_cycles), (text_x, text_y + y_diff), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("cpus: {}", config.cpus), (text_x, text_y + (2 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("job_count: {}", config.jobs), (text_x, text_y + (3 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frames: {} - {}", config.min_frames, config.max_frames), (text_x, text_y + (4 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("chunk_size: {} - {}", config.min_task_frames, config.max_task_frames), (text_x, text_y + (5 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frame_cycles: {} - {}", config.min_frame_render_cycles, config.max_frame_render_cycles), (text_x, text_y + (6 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("min_startup_cycles: {} - {}", config.min_task_startup_cycles, config.max_task_startup_cycles), (text_x, text_y + (7 * y_diff)), ("Arial", 20).into_font())).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0_f32..config.max_render_cycles as f32, 0_f32..100_f32).expect("chart build failed.");
    chart.configure_mesh()
        .draw().expect("chart draw failed.");

    for rep in 0..config.repetitions {
        print!("rep: {}", rep);
        let mut farm = Farm::new(config.cpus);

        for _ in 0..config.jobs {
            let mut frames = config.min_frames;
            if config.max_frames != frames {
                frames = rng.gen_range(config.min_frames..=config.max_frames);
            }
            frames *= rng.gen_range(config.min_frame_render_cycles..=config.max_frame_render_cycles);
            let mut chunk_size = config.min_task_frames;
            if config.min_task_frames < config.max_task_frames {
                chunk_size = rng.gen_range(config.min_task_frames..=config.max_task_frames);
            }
            let startup_cycles = rng.gen_range(config.min_task_startup_cycles..=config.max_task_startup_cycles);
            let job = Job::new(frames, chunk_size, startup_cycles);
            farm.submit(job);
        }

        let mut usage_seq: Vec<f32> = Vec::new();
        let mut jobs_done: Vec<f32> = Vec::new();
        let mut finished = false;

        for _ in 0..=config.max_render_cycles {
            let usage = farm.render();
            let done_p = config.jobs as f32 / farm.jobs.len() as f32;
            jobs_done.push(done_p);
            usage_seq.push(usage);
            if finished {
                break;
            }
            if farm.jobs.is_empty() {
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
        assert_eq!(job.frame_num, 1);
        let job = Job::new(2, 1, 0);
        assert_eq!(job.frame_num, 2);
        let job = Job::new(1, 100, 0);
        assert_eq!(job.frame_num, 1);
        let job = Job::new(1, 1, 1);
        assert_eq!(job.frame_num, 2);
        let job = Job::new(1, 1, 2);
        assert_eq!(job.frame_num, 3);
        let job = Job::new(2, 1, 2);
        assert_eq!(job.frame_num, 6);
        let job = Job::new(10, 1, 10);
        assert_eq!(job.frame_num, 110);
        let job = Job::new(10, 2, 10);
        assert_eq!(job.frame_num, 60);
        let job = Job::new(10, 5, 10);
        assert_eq!(job.frame_num, 30);
        let job = Job::new(10, 10, 10);
        assert_eq!(job.frame_num, 20);
    }

    #[test]
    fn job_render_takes_one_frame_until_zero() {
        let mut job = Job::new(2, 10, 0);
        assert_eq!(job.frame_num, 2);
        job.render();
        assert_eq!(job.frame_num, 1);
        job.render();
        assert_eq!(job.frame_num, 0);
        job.render();
        assert_eq!(job.frame_num, 0);
        job.render();
        assert_eq!(job.frame_num, 0);
    }
}
