mod config;
mod runner;
mod state;

use runner::run;
use std::sync::{Arc, RwLock};

pub fn main() {
    run(Arc::new(RwLock::new(state::State::new())));
}
