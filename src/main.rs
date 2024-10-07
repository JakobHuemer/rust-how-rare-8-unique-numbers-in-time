use crate::timestamp::Timestamp;

mod timestamp;

fn main() {
    println!("Hello, world!");
    let mut ts = Timestamp::new();

    let mut still_running = true;

    let mut unique: u32 = 0;
    let mut not_unique: u32 = 0;

    while still_running {
        if ts.is_unique() {
            unique += 1;
        } else {
            not_unique += 1;
        }
        still_running = ts.increment();
    }

    println!("unique: {}", unique);
    println!("not_unique: {}", not_unique);

    println!("{}", unique as f64 / (not_unique as f64 + unique as f64));

}
