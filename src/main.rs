#![allow(dead_code)]

mod defaults;
mod model;
mod services;

use defaults::{generator, storage};
use services::pipeline::calculate_user_stats;

fn main() {
    let mut storage = storage();
    let transfers = generator().build().generate(1);

    storage.insert_all(transfers);

    let stats = calculate_user_stats(storage.get());

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
}
