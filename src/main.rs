use std::time::Instant;
mod day01;
mod day02;
mod day03;

fn main() {
    let start = Instant::now();
    let day03 = day03::run();
    let time = start.elapsed();
    println!("Day 3 completed in {time:?}\n{day03}");
    // day01::run();
}
