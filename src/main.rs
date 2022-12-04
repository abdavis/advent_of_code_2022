use std::time::Instant;
mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let start = Instant::now();
    let day03 = day03::run();
    let time = start.elapsed();
    println!("Day 3 completed in {time:?}\n{day03}");
    // day01::run();
    let start = Instant::now();
    let day04 = day04::run();
    let time = start.elapsed();
    println!("Day 4 completed in {time:?}\n{day04}");
}
