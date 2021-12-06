### Farm Usage Simulator

Given a few basic charasteristics of a render farm and render jobs this app runs a few randomized farm usage scenarios and plots the results. You can use this to help tweak your render farm and job settings.

### usage

The app is written in Rust. You'll need Rust installed. Once you have it, build the project and run the binary:

```bash
# in the repo root containing the "src" dir.
$ cargo b --release
...

# move the binary here from the build dir for convenience...
$ mv target/release/farmsim .

# run
$ ./farmsim
```

On the first use the app will save the default config file next to current directory. the config file is called: `farmsimconf.json`. Every run saves a png image of the farm usage statistics. The plot png file is called: `farm_usage_plot.png` and it's 1600 x 900 pixels.

- After the 1st run saved the config json, you can update it and render the sims again.
- The files are overwritten. If you want to keep one, rename the file.

*The default config runs in ~0.2 second on a pentium cpu.*

![default config](/example_renders/farm_usage_plot_01.png?raw=true "default config")

### tweak the config, save the file, analyse the results:

The next few sims show 10 sims of 1600 cycles. The config is 500 cpus, 100 randomised jobs submitted. According to the results after only tweaking the chunk size, starting with 500 cpus finishing on average after ~1000 cycles you can save 300 cpus and finish on average after ~900 cycles.*

 \* *This is a Rust R'n'D hobby project. There can be any number of mistakes in the calculations.*

![default config](/example_renders/farm_usage_plot_02.png?raw=true "default config")
![default config](/example_renders/farm_usage_plot_03.png?raw=true "default config")
![default config](/example_renders/farm_usage_plot_04.png?raw=true "default config")
![default config](/example_renders/farm_usage_plot_05.png?raw=true "default config")
