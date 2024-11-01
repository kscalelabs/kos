/**
 * Defines the main control loop that runs in its own independent thread.
 */
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::state::State;

pub fn run(state: Arc<RwLock<State>>) {
    loop {
        let state = state.read().unwrap();
        println!("Running main control loop... State: {:?}", state);
        thread::sleep(Duration::from_millis(10));
    }
}
