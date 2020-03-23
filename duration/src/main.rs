use std::time::{SystemTime};
use std::time::UNIX_EPOCH;

extern crate chrono;
use chrono::{DateTime, Utc};
use chrono::Duration;
fn main() {
/*
    let time = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("error");
    // let time1: i32 = time.parse().unwrap();
    // let time1: f64 = time.parse().unwrap();
    println!("{:?}", time);

    let f2 = time + Duration::new(86400000, 0);
    println!("{:?}", f2);

    let f = Duration::new(5, 0);
    let f1 = f + Duration::new(0, 5);
    println!("{:?}", f1);


    // let time = DateTime::now();

    let now: DateTime<Utc> = Utc::now();

    println!("{}", now);
    let milli = now.timestamp_millis();
    println!("{}", milli);
*/
    // let dmy = milli.from_timestamp();

    // let time = Utc.timestamp_millis(1431648000).timestamp();
    // println!("{:?}", time);


//work withd DateTime<Utc>
//oldDuration is the time that we issue and duration can be 1day, 1h, or whatever

    let date:  DateTime<Utc> = Utc::now();
    println!("{:?}", date);

    let old_time = date;
    let duration = Duration::days(1i64);
    // let new_time = date.checked_add_signed(old_time);
    let new_time = date.checked_add_signed(duration);
    println!("{:?}", new_time.unwrap());
}
