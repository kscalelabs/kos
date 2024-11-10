/**
 * Defines the main control loop that runs in its own independent thread.
 */
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::state::State;

pub fn run(_state: Arc<RwLock<State>>) {
    loop {
        println!("Running main control loop...");
        thread::sleep(Duration::from_millis(100));
    }
}
