use chrono::prelude::*;
use log::{error, warn, info, debug, trace};
use std::convert::TryInto;

fn calc_easter(year:i32) -> Date<chrono::Utc> {
    info! ("entering calc_easter");
    let a = year % 19;
    debug!("a is  {}", a);
    let b = year / 100;
    debug!("b is  {}", b);
    let c = year % 100;
    debug!("c is  {}", c);
    let d = b / 4;
    debug!("d is  {}", d);
    let e = b % 4;
    debug!("e is  {}", e);
    let f = (b + 8) / 25;
    debug!("f is  {}", f);
    let g = (b -f + 1)/3;
    debug!("g is  {}", g);
    let h = (19 * a + b - d - g + 15) % 30;
    debug!("h is  {}", h);
    let i = c / 4;
    debug!("i is  {}", i);
    let k = c % 4;
    debug!("k is  {}", k);
    let l = (32 + 2*e + 2*i - h - k) % 7;
    debug!("l is  {}", l);
    let m = (a + 11*h + 22*l)/451;
    debug!("m is  {}", m);
    // type conversion to unsigned int
    let mon:u32 = ((h + l - 7*m + 114) / 31).try_into().unwrap();
    debug!("mon is  {}", mon);
    let day:u32 = (((h + l - 7*m + 114) % 31) + 1).try_into().unwrap();
    debug!("day is  {}", day);
    let easter = Utc.ymd(year, mon, day);
    easter
}

fn main() {
    // Set the environment variable level before executing the binary.
    // e.g. "env RUST_LOG=info cargo run"
    // e.g. "env RUST_LOG=debug cargo run"
    env_logger::init();
    info! ("entering calc_easter");
    let utc: DateTime<Utc> = Utc::now();
    debug!("The current UTC date/time is {}", utc);
    debug!("The current UTC date is {}", Utc::today());
    debug!("The current year is {}", Utc::today().year());
    debug!("The current month is {}", Utc::today().month());
    debug!("The current day is {}", Utc::today().day());
    let result1 = calc_easter(Utc::today().year());
    println!("easter is  {}", result1);
    let result2 = calc_easter(1961);
    println!("easter is  {}", result2);
}
