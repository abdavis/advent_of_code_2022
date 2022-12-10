use std::time::Instant;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

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
    let start = Instant::now();
    let day05 = day05::run();
    let time = start.elapsed();
    println!("Day 5 completed in {time:?}\n{day05}");
    let start = Instant::now();
    let day06 = day06::run();
    let time = start.elapsed();
    println!("Day 6 completed in {time:?}\n{day06}");
    //    let start = Instant::now();
    //    let day07 = day07::run();
    //    let time = start.elapsed();
    //    println!("Day 7 completed in {time:?}\n{day07}");
    let start = Instant::now();
    let day08 = day08::run();
    let time = start.elapsed();
    println!("Day 8 completed in {time:?}\n{day08}");
}
