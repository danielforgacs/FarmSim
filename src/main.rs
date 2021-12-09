use rand::prelude::*;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;

const PLOT_WIDTH: u32 = 1600;
const PLOT_HEIGTH: u32 = 900;

struct Job {
    frame_count: u32,
    task_count: u32,
}

struct Farm {
    jobs: Vec<Job>,
    cpu_count: u32,
    free_cpu_count: u32,
}

#[derive(Serialize, Deserialize)]
struct Config {
    repetitions: u32,
    max_render_cycles: u32,
    farm_cpus: u32,
    initial_job_count: u32,
    min_frames_per_job: u32,
    max_frames_per_job: u32,
    min_render_cycles_per_frame: u32,
    max_render_cycles_per_frame: u32,
    min_frames_per_task: u32,
    max_frames_per_task: u32,
    min_task_startup_cycles: u32,
    max_task_startup_cycles: u32,
}

struct SimResult {
    farm_usage: Vec<f32>,
    last_cycle: u32,
    total_frames: u32,
}

impl Job {
    fn new(mut frames: u32, task_frames: u32, startup_cycles: u32) -> Self {
        let mut tasks = frames / task_frames;
        if frames % task_frames > 0 {
            tasks += 1;
        }
        frames += tasks * startup_cycles;

        Self {
            frame_count: frames,
            task_count: tasks,
        }
    }

    fn render(&mut self) {
        if self.frame_count > 0 {
            self.frame_count -= 1;
        }
    }
}

impl Farm {
    fn new(cpu_num: u32) -> Self {
        Self {
            jobs: Vec::new(),
            cpu_count: cpu_num,
            free_cpu_count: cpu_num,
        }
    }

    fn submit(&mut self, job: Job) {
        self.jobs.push(job);
    }

    fn render(&mut self) -> f32 {
        'mainloop: for job in self.jobs.iter_mut() {
            for _ in 0..job.task_count {
                if self.free_cpu_count == 0 {
                    break 'mainloop;
                }
                self.free_cpu_count -= 1;
                job.render();
                if job.frame_count == 0 {
                    break;
                }
            }
        }
        self.jobs.retain(|x| x.frame_count > 0);
        let mut usage = 100f32;
        let used = self.cpu_count - self.free_cpu_count;
        if used != self.cpu_count {
            usage = (used as f32 / self.cpu_count as f32) * 100f32;
        }
        self.free_cpu_count = self.cpu_count;
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
            repetitions: 1,
            max_render_cycles: 1600,
            farm_cpus: 1,
            initial_job_count: 1,
            min_frames_per_job: 1,
            max_frames_per_job: 1,
            min_frames_per_task: 1,
            max_frames_per_task: 1,
            min_render_cycles_per_frame: 1,
            max_render_cycles_per_frame: 1,
            min_task_startup_cycles: 0,
            max_task_startup_cycles: 0,
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

    if let Some(error_message) = sanity_check_config(&config) {
        println!("{}", error_message);
        return;
    }

    let mut all_results: Vec<SimResult> = Vec::new();

    for _ in 0..config.repetitions {
        let mut farm = Farm::new(config.farm_cpus);
        for _ in 0..config.initial_job_count {
            let init_data = generate_job_init_values(&config);
            let job = Job::new(init_data[0], init_data[1], init_data[2]);
            farm.submit(job);
        }
        all_results.push(run_sim(farm, config.max_render_cycles));
    }
    process_results(all_results, &config);
    // sim(&config);
}

fn process_results(all_results: Vec<SimResult>, config: &Config) {
    for result in all_results {
        println!("----------------------------------------");
        println!("total frames: {}", result.total_frames);
        println!("last cycle: {}", result.last_cycle);
        println!("usage: {:?}", result.farm_usage);
    }
    let a = generate_plot_path();
    let root = BitMapBackend::new(&a, (PLOT_WIDTH, PLOT_HEIGTH))
        .into_drawing_area();
    root.fill(&WHITE)
        .expect("can't fill the image.");
    let text_x = 50;
    let text_y = 400;
    let y_diff = 25;
    root.draw(&Text::new(format!("repetitions: {}", config.repetitions), (text_x, text_y), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("max_cycles: {}", config.max_render_cycles), (text_x, text_y + y_diff), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("cpus: {}", config.farm_cpus), (text_x, text_y + (2 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("job_count: {}", config.initial_job_count), (text_x, text_y + (3 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frames: {} - {}", config.min_frames_per_job, config.max_frames_per_job), (text_x, text_y + (4 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("chunk_size: {} - {}", config.min_frames_per_task, config.max_frames_per_task), (text_x, text_y + (5 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frame_cycles: {} - {}", config.min_render_cycles_per_frame, config.max_render_cycles_per_frame), (text_x, text_y + (6 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("min_startup_cycles: {} - {}", config.min_task_startup_cycles, config.max_task_startup_cycles), (text_x, text_y + (7 * y_diff)), ("Arial", 20).into_font())).unwrap();

}

fn generate_plot_path() -> String {
    "farm_usage_plot.png".to_string()
}

fn run_sim(mut farm: Farm, max_cycles: u32) -> SimResult {
    let total_frames = farm.jobs.iter().map(|x| x.frame_count).sum();
    let mut finished = false;
    let mut farm_usage: Vec<f32> = Vec::new();
    let mut last_cycle = 0_u32;
    for cycle in 0..max_cycles {
        farm_usage.push(farm.render());
        if finished {
            last_cycle = cycle;
            break;
        }
        if farm.jobs.is_empty() {
            finished = true;
        }
    }
    SimResult {
        farm_usage,
        last_cycle,
        total_frames,
    }
}

fn sanity_check_config(config: &Config) -> Option<&str> {
    if config.min_frames_per_job < 1 || config.max_frames_per_job < config.min_frames_per_job {
        return Some("Bad frame range.");
    }
    if config.min_frames_per_task < 1 || config.max_frames_per_task < config.min_frames_per_task{
        return Some("Bad task range.");
    }
    Option::None
}

fn generate_job_init_values(config: &Config) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut frames = rng.gen_range(config.min_frames_per_job..=config.max_frames_per_job);
    frames *= rng.gen_range(config.min_render_cycles_per_frame..=config.max_render_cycles_per_frame);
    let task_frames = rng.gen_range(config.min_frames_per_task..=config.max_frames_per_task);
    let startup_cycles = rng.gen_range(config.min_task_startup_cycles..=config.max_task_startup_cycles);
    vec![frames, task_frames, startup_cycles]
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
    root.draw(&Text::new(format!("cpus: {}", config.farm_cpus), (text_x, text_y + (2 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("job_count: {}", config.initial_job_count), (text_x, text_y + (3 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frames: {} - {}", config.min_frames_per_job, config.max_frames_per_job), (text_x, text_y + (4 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("chunk_size: {} - {}", config.min_frames_per_task, config.max_frames_per_task), (text_x, text_y + (5 * y_diff)), ("Arial", 20).into_font())).unwrap();
    root.draw(&Text::new(format!("frame_cycles: {} - {}", config.min_render_cycles_per_frame, config.max_render_cycles_per_frame), (text_x, text_y + (6 * y_diff)), ("Arial", 20).into_font())).unwrap();
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
        let mut farm = Farm::new(config.farm_cpus);

        for _ in 0..config.initial_job_count {
            let mut frames = config.min_frames_per_job;
            if config.max_frames_per_job != frames {
                frames = rng.gen_range(config.min_frames_per_job..=config.max_frames_per_job);
            }
            frames *= rng.gen_range(config.min_render_cycles_per_frame..=config.max_render_cycles_per_frame);
            let mut chunk_size = config.min_frames_per_task;
            if config.min_frames_per_task < config.max_frames_per_task {
                chunk_size = rng.gen_range(config.min_frames_per_task..=config.max_frames_per_task);
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
            let done_p = config.initial_job_count as f32 / farm.jobs.len() as f32;
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
        assert_eq!(job.frame_count, 1);
        let job = Job::new(2, 1, 0);
        assert_eq!(job.frame_count, 2);
        let job = Job::new(1, 100, 0);
        assert_eq!(job.frame_count, 1);
        let job = Job::new(1, 1, 1);
        assert_eq!(job.frame_count, 2);
        let job = Job::new(1, 1, 2);
        assert_eq!(job.frame_count, 3);
        let job = Job::new(2, 1, 2);
        assert_eq!(job.frame_count, 6);
        let job = Job::new(10, 1, 10);
        assert_eq!(job.frame_count, 110);
        let job = Job::new(10, 2, 10);
        assert_eq!(job.frame_count, 60);
        let job = Job::new(10, 5, 10);
        assert_eq!(job.frame_count, 30);
        let job = Job::new(10, 10, 10);
        assert_eq!(job.frame_count, 20);
    }

    #[test]
    fn job_render_takes_one_frame_until_zero() {
        let mut job = Job::new(2, 10, 0);
        assert_eq!(job.frame_count, 2);
        job.render();
        assert_eq!(job.frame_count, 1);
        job.render();
        assert_eq!(job.frame_count, 0);
        job.render();
        assert_eq!(job.frame_count, 0);
        job.render();
        assert_eq!(job.frame_count, 0);
    }

    #[test]
    fn farm_render_test_01() {
        let job = Job::new(1, 1000, 0);
        let mut farm = Farm::new(1);
        farm.submit(job);
        let usage = farm.render();
        assert_eq!(usage, 100.0);
        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    #[test]
    fn farm_render_test_02() {
        let job = Job::new(3, 1000, 0);
        let mut farm = Farm::new(1);
        farm.submit(job);
        let usage = farm.render();
        assert_eq!(usage, 100.0);
        let usage = farm.render();
        assert_eq!(usage, 100.0);
        let usage = farm.render();
        assert_eq!(usage, 100.0);
        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    #[test]
    fn farm_render_test_03() {
        let job = Job::new(2, 1000, 0);
        let mut farm = Farm::new(2);
        farm.submit(job);
        let usage = farm.render();
        assert_eq!(usage, 50.0);
        let usage = farm.render();
        assert_eq!(usage, 50.0);
        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    #[test]
    fn farm_render_test_04() {
        let job = Job::new(4, 1000, 0);
        let mut farm = Farm::new(4);
        farm.submit(job);

        for _ in 0..4 {
            let usage = farm.render();
            assert_eq!(usage, 25.0);
        }

        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    #[test]
    fn farm_render_test_05() {
        let job = Job::new(10, 1000, 0);
        let mut farm = Farm::new(10);
        farm.submit(job);

        for _ in 0..10 {
            let usage = farm.render();
            assert_eq!(usage, 10.0);
        }

        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    #[test]
    fn farm_render_test_06() {
        let job = Job::new(10, 2, 0);
        let mut farm = Farm::new(10);
        farm.submit(job);

        for _ in 0..2 {
            let usage = farm.render();
            assert_eq!(usage, 50.0);
        }

        let usage = farm.render();
        assert_eq!(usage, 0f32);
    }

    use std::collections::HashMap;
    #[test]
    fn farm_render_test_07() {
        // let test_cases: Vec<HashMap<str, u32>>


//         let test_cases = [
//             1, 1000, 0,
//         ]
//         let job = Job::new(10, 2, 0);
//         let mut farm = Farm::new(10);
//         farm.submit(job);
//
//         for _ in 0..2 {
//             let usage = farm.render();
//             assert_eq!(usage, 50.0);
//         }
//
//         let usage = farm.render();
//         assert_eq!(usage, 0f32);
    }

    #[test]
    fn sim_01() {
        let mut config = Config::new();
        config.repetitions = 1;
        config.max_render_cycles = 100;
        config.farm_cpus = 1;
        config.initial_job_count = 1;
        config.min_frames_per_job = 1;
        config.max_frames_per_job = 1;
        config.min_frames_per_task = 1;
        config.max_frames_per_task = 1;
        config.min_render_cycles_per_frame = 1;
        config.max_render_cycles_per_frame = 1;
        config.min_task_startup_cycles = 0;
        config.max_task_startup_cycles = 0;

        let mut farm = Farm::new(config.farm_cpus);
        for _ in 0..config.initial_job_count {
            let init_data = generate_job_init_values(&config);
            let job = Job::new(init_data[0], init_data[1], init_data[2]);
            farm.submit(job);
        }
        let result = run_sim(farm, config.max_render_cycles);
        assert_eq!(result.total_frames, 1);
        assert_eq!(result.last_cycle, 1);
        assert_eq!(result.farm_usage, vec![100.0, 0.0]);
    }

    #[test]
    fn sim_02() {
        let mut config = Config::new();
        config.repetitions = 1;
        config.max_render_cycles = 1000;
        config.farm_cpus = 2;
        config.initial_job_count = 2;
        config.min_frames_per_job = 10;
        config.max_frames_per_job = 10;
        config.min_frames_per_task = 5;
        config.max_frames_per_task = 5;
        config.min_render_cycles_per_frame = 1;
        config.max_render_cycles_per_frame = 1;
        config.min_task_startup_cycles = 2;
        config.max_task_startup_cycles = 2;

        let mut farm = Farm::new(config.farm_cpus);
        for _ in 0..config.initial_job_count {
            let init_data = generate_job_init_values(&config);
            let job = Job::new(init_data[0], init_data[1], init_data[2]);
            farm.submit(job);
        }
        let result = run_sim(farm, config.max_render_cycles);
        assert_eq!(result.total_frames, 28);
        assert_eq!(result.last_cycle, 14);
    }

    #[test]
    fn gen_job_parms_and_init_job() {
        let mut config = Config::new();
        config.min_frames_per_job = 1;
        config.max_frames_per_job = 1;
        config.min_frames_per_task = 1;
        config.max_frames_per_task = 1;
        config.min_render_cycles_per_frame = 1;
        config.max_render_cycles_per_frame = 1;
        config.min_task_startup_cycles = 0;
        config.max_task_startup_cycles = 0;
        let job_values = generate_job_init_values(&config);
        assert_eq!(job_values, vec![1, 1, 0]);
        let job = Job::new(job_values[0], job_values[1], job_values[2]);
        assert_eq!(job.frame_count, 1);
        assert_eq!(job.task_count, 1);

        let mut config = Config::new();
        config.min_frames_per_job = 10;
        config.max_frames_per_job = 10;
        config.min_frames_per_task = 1;
        config.max_frames_per_task = 1;
        config.min_render_cycles_per_frame = 1;
        config.max_render_cycles_per_frame = 1;
        config.min_task_startup_cycles = 0;
        config.max_task_startup_cycles = 0;
        let job_values = generate_job_init_values(&config);
        assert_eq!(job_values, vec![10, 1, 0]);
        let job = Job::new(job_values[0], job_values[1], job_values[2]);
        assert_eq!(job.frame_count, 10);
        assert_eq!(job.task_count, 10);

        let mut config = Config::new();
        config.min_frames_per_job = 10;
        config.max_frames_per_job = 10;
        config.min_frames_per_task = 2;
        config.max_frames_per_task = 2;
        config.min_render_cycles_per_frame = 3;
        config.max_render_cycles_per_frame = 3;
        config.min_task_startup_cycles = 0;
        config.max_task_startup_cycles = 0;
        let job_values = generate_job_init_values(&config);
        assert_eq!(job_values, vec![30, 2, 0]);
        let job = Job::new(job_values[0], job_values[1], job_values[2]);
        assert_eq!(job.frame_count, 30);
        assert_eq!(job.task_count, 15);

        let mut config = Config::new();
        config.min_frames_per_job = 10;
        config.max_frames_per_job = 10;
        config.min_frames_per_task = 2;
        config.max_frames_per_task = 2;
        config.min_render_cycles_per_frame = 3;
        config.max_render_cycles_per_frame = 3;
        config.min_task_startup_cycles = 0;
        config.max_task_startup_cycles = 0;
        let job_values = generate_job_init_values(&config);
        assert_eq!(job_values, vec![30, 2, 0]);
        let job = Job::new(job_values[0], job_values[1], job_values[2]);
        assert_eq!(job.frame_count, 30);
        assert_eq!(job.task_count, 15);

        let mut config = Config::new();
        config.min_frames_per_job = 10;
        config.max_frames_per_job = 10;
        config.min_render_cycles_per_frame = 3;
        config.max_render_cycles_per_frame = 3;
        config.min_frames_per_task = 5;
        config.max_frames_per_task = 5;
        config.min_task_startup_cycles = 3;
        config.max_task_startup_cycles = 3;
        let job_values = generate_job_init_values(&config);
        assert_eq!(job_values, vec![30, 5, 3]);
        let job = Job::new(job_values[0], job_values[1], job_values[2]);
        assert_eq!(job.frame_count, 48);
        assert_eq!(job.task_count, 6);
    }
}
