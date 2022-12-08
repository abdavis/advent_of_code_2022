use std::time::Instant;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod up_the_ante;

fn main() {
    let start = Instant::now();
    let ante_day05 = up_the_ante::day05::run();
    let time = start.elapsed();
    println!("Day 5 completed in {time:?}\n{ante_day05}");
}
