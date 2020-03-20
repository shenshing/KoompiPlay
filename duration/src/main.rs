use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;

fn main() {
    let time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("error");
    // let time1: i32 = time.parse().unwrap();
    // let time1: f64 = time.parse().unwrap();
    println!("{:?}", time);

    let f2 = time + Duration::new(86400000, 0);
    println!("{:?}", f2);

    let f = Duration::new(5, 0);
    let f1 = f + Duration::new(0, 5);
    println!("{:?}", f1);
}
