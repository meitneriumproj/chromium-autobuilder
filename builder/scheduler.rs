use std::thread;
use std::time::Duration;

use crate::builder;

const INTERVAL: u64 = 900; // 15 minutes

pub fn start_scheduler() {

    println!("Builder scheduler started (15 minute interval)");

    loop {

        println!("Running build check...");

        builder::check_for_update();

        thread::sleep(Duration::from_secs(INTERVAL));
    }
}