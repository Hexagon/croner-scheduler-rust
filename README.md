# Croner Scheduler

Croner Scheduler is a standalone Rust library focused on scheduling tasks based on cron patterns. This library is a stand-alone part of the Croner project and is designed for developers who want a lightweight and efficient tool for threaded task scheduling using the familiar cron syntax.

For more information about cron pattern parsing and evaluation, please refer to the [Croner](https://github.com/hexagon/croner) crate.

## Features

- Schedule tasks in separate threads based on cron patterns, enabling concurrent task execution.
- Robust error handling.
- Control execution flow with the ability to pause, resume, or stop scheduled tasks.
- Operates in-memory without the need for persistent storage or configuration files.
- Highly optimized method of finding future/past matches.
- No dependencies except `croner`.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. If not, you can get it from [the official Rust website](https://www.rust-lang.org/).

### Installation

Add `croner-scheduler` to your `Cargo.toml` dependencies:

**Please note that croner for Rust is work in progress and not production-ready**

```toml
[dependencies]
croner-scheduler = "0.0.10" # Adjust the version as necessary
```

### Usage

Here's a quick example to get you started with scheduling a task:

```rust
use chrono::Local;
use croner_scheduler::{CronScheduler, SchedulerResult};
use croner::Cron;
use std::thread;

fn main() {
    // Schedule a task at even seconds
    let cron: Cron = "0/2 * * * * *".parse().expect("Invalid cron expression");
    let mut scheduler = CronScheduler::new(cron);

    // The trigger closure must be set up to accept an optional context
    scheduler.start(|_: Option<&()>| {
        println!("Task 1 triggered at {:?}", Local::now());
    });

    // The tasks can be paused, resumed, or stopped as needed
    // scheduler.pause();
    // scheduler.resume();
    // scheduler.stop();

    // Loop to keep the main process alive
    // - You need to supply a time-zoned "now" to tick, so that
    //   croner knows which timezone to match the pattern against.
    //   Using Local in this example.
    while scheduler.tick(Local::now()) != SchedulerResult::Dead {
        // Sleep for a short duration to prevent busy waiting
        thread::sleep(std::time::Duration::from_millis(300));
    }
}
```

For detailed documentation and examples, please visit [Croner Scheduler on docs.rs](https://docs.rs/croner-scheduler/).

## Contributing

We welcome contributions! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Disclaimer

Please note that Croner Scheduler is currently in its early stages of development. As such, the API is subject to change in future releases, adhering to semantic versioning principles. We recommend keeping this in mind when integrating Croner Scheduler into your projects.

## Contact

If you have any questions or feedback, please open an issue in the repository, and we'll get back to you as soon as possible.