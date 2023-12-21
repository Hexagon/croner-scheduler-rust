use chrono::Local;
use croner::Cron;
use croner_scheduler::{CronScheduler, SchedulerResult};
use std::thread;

fn main() {
    // Prepare a cron pattern
    let cron: Cron = Cron::new("0/2 * * * * *")
        .with_seconds_optional()
        .parse()
        .expect("Invalid cron expression");

    // Create cron scheduler
    let mut scheduler = CronScheduler::new(cron);

    // Set up the trigger
    // - The trigger closure must be set up to accept an optional context
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
