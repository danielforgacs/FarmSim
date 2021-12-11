### Farm Usage Simulator

Given a few basic characteristics of a render farm and render jobs this app runs randomized farm usage scenarios and plots the results. You can use this to help to tweak your render farm and job settings to save time and money.

### usage

*The app is written in **`Rust`**. You'll need Rust installed. Once you have it, build the project and run the binary:*

```bash
# in the repo root containing the "src" dir.
$ cargo b --release
...

# move the binary to the current dir from the build dir for convenience...
$ mv target/release/farmsim .

# run
$ ./farmsim
```

On the first use the app will save the default config file in the current directory. the config file is called: `farmsimconf.json`. Change the values and run the app again. Every run saves a png image of the farm usage statistics with the current configuration.

- After the 1st run saved the `config json`, you can change it and render the sims again.
- A `new, numbered image` is saved after every run.
- The plot png file is called: `farm_usage_plot.####.png` and it's 1800 x 900 pixels.
- Sim `job parameters are randomised` within the ranges in the config for ever job, every repetition.
- The `black lines` are farm utilisation in `%`.
- The `green line` is finished jobs `%` of the total jobs initially submitted.

### Examples

**1st run:**

![default config](/example_renders/farm_usage_plot.0001.png?raw=true "default config")

### now tweak the config & analyse the results

After the defaults that's a bit too artifical, let's have a more realistic configuration:

Let's say you have a `farm with 1000 CPUs`. If you `submit 1000 scenes` that take `1-25` time units, aka. `cycles to open`, are `1 - 250 frames long`, with `frames that take 1 - 25 cycles to render` with `tasks that are 1 frame` long, then your farm finishes in 23,000 cycles on average and looks like this:

![sim plot](/example_renders/farm_usage_plot.0002.png?raw=true "sim plot")

Now start tweaking. The **most important** parameter seems to be **task size**. The deal with task size is, that that's the parm that controls how many times you force the farm to spend time on just opening scenes.

![sim plot](/example_renders/farm_usage_plot.0003.png?raw=true "sim plot")

If the only parameter I change is that I let CPUs render 2 frames of a job instead of just one, the same farm with the same jobs finishes in just about 10,000 cycles earlier on average.

Now you can tweak farm parameters, run the sim with the same job settings and see what's best for you.

**The most important takeaway probably is**, that a task size of 1 feels like the fastest way to get renders, but then a 60 frames scene that takes a minute to open spends 59 minutes extra farm time just being reopened on different machines. This works when there's only one job, but when new jobs keep coming the new ones will have to wait much longer to be started in the first place.

And then experiment...

![sim plot](/example_renders/farm_usage_plot.0004.png?raw=true "sim plot")

and then at this point 100 CPUs are enough to finish with the same jobs in the same time.

![sim plot](/example_renders/farm_usage_plot.0005.png?raw=true "sim plot")

Other configurations also make sense. If you keep the CPU count, the jobs finish earlier. You can tweak it to keep like 5% of the farm finish fast so new artists can have their first few frames to check faster. And so on...

![sim plot](/example_renders/farm_usage_plot.0006.png?raw=true "sim plot")

### Config

`Frames rendering doesn't last in term of time. The sim works with cycles`. Timing is based on proportions, `duration is a multiplier of cycles`. A job has a configurable, randomised start up and render time. How long a frame renders, how long it takes to open up a scene and start a render is all specified in cycles. Like a job with one 1 hour frame takes just as long as a job with 6 frames that take 10 minutes to render each. Configure these in min, max ranges and each job gets a random value within the range for every sim.

If you think of a `cycle` as a `minute`, you'll get `all times in minutes`. Like, given the farm is empty, 1 job with 10 frames, 5 chunk size -> meaning 2 tasks with 5 frames, 1 minute startup and 1 minute per frame render time => 2 times 6 minutes jobs. On one cpu it takes 12 minutes, on 2 cpus it's 6 minutes.

- `repetitions`: Number of sims to run and plot at once.
- `max_render_cycles`: Number of cycles to run max. Sim finish early when all the jobs finish.
- `farm_cpus`: Number of CPUs (blades?) on the farm.
- `initial_job_count`: Number of submitted jobs when the sim starts. A sim runs with this initial state, no new jobs are submitted and the sim shows how those jobs render.
- `min_frames_per_job`: random job frame range min length
- `max_frames_per_job`: random job frame range max length
- `min_render_cycles_per_frame`: random job chunk size range min
- `max_render_cycles_per_frame`: random job chunk size range max
- `min_frames_per_task`: min random number of cycles of rendering a frame
- `max_frames_per_task`: max random number of cycles of rendering a frame
- `min_task_startup_cycles`: min random number of cycles of opening a scene befor starting a task render
- `max_task_startup_cycles`: max random number of cycles of opening a scene befor starting a task render

**config values with ranges are randomised per job, per sim.**

***

- *The default config runs in ~0.2 second on a pentium cpu.*
- *Also, the images are saved with the 1st version. I won't put too much effort into updating these. Your sims might look different.*
- *Finally, this is a Rust R'n'D hobby project. There can be any number of mistakes in these not too complicated calculations.*
