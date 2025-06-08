use rust_challenge::factories::defaults::generator;
use rust_challenge::factories::defaults::storage;
use rust_challenge::services::analytics::Analytics;

fn main() {
    let mut storage = storage();
    let transfers = generator().build().generate(10_000);

    storage.insert_all(transfers);

    let stats = Analytics::new(storage).get_stats();

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
}
