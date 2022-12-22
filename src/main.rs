use std::time::Instant;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
//mod day16;

fn main() {
    let start = Instant::now();
    let day01 = day01::run();
    let time = start.elapsed();
    println!("Day 1 completed in {time:?}\n{day01}\n");

    let start = Instant::now();
    let day02 = day02::run();
    let time = start.elapsed();
    println!("Day 2 completed in {time:?}\n{day02}\n");

    let start = Instant::now();
    let day03 = day03::run();
    let time = start.elapsed();
    println!("Day 3 completed in {time:?}\n{day03}\n");

    let start = Instant::now();
    let day04 = day04::run();
    let time = start.elapsed();
    println!("Day 4 completed in {time:?}\n{day04}\n");

    let start = Instant::now();
    let day05 = day05::run();
    let time = start.elapsed();
    println!("Day 5 completed in {time:?}\n{day05}\n");

    let start = Instant::now();
    let day06 = day06::run();
    let time = start.elapsed();
    println!("Day 6 completed in {time:?}\n{day06}\n");

    let start = Instant::now();
    let day07 = day07::run();
    let time = start.elapsed();
    println!("Day 7 completed in {time:?}\n{day07}\n");

    let start = Instant::now();
    let day08 = day08::run();
    let time = start.elapsed();
    println!("Day 8 completed in {time:?}\n{day08}\n");

    let start = Instant::now();
    let day09 = day09::run();
    let time = start.elapsed();
    println!("Day 9 completed in {time:?}\n{day09}\n");

    let start = Instant::now();
    let day10 = day10::run();
    let time = start.elapsed();
    println!("Day 10 completed in {time:?}\n{day10}\n");

    let start = Instant::now();
    let day11 = day11::run();
    let time = start.elapsed();
    println!("Day 11 completed in {time:?}\n{day11}\n");

    let start = Instant::now();
    let day12 = day12::run(false);
    let time = start.elapsed();
    println!("Day 12 completed in {time:?}\n{day12}\n");

    //let start = Instant::now();
    //let day14 = day14::run();
    //let time = start.elapsed();
    //println!("Day 14 completed in {time:?}\n{day14}\n");
    //let start = Instant::now();
    //let day16 = day16::run();
    //let time = start.elapsed();
    //println!("Day 16 completed in {time:?}\n{day16}\n");
}
