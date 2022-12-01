const INPUT: &str = include_str!("inputs/day01.txt");

pub fn day01() {
    let (max, max3) = sum3_calories(INPUT);
    println!("Max: {max}\nMax 3 foods: {max3}");
}

fn sum3_calories(input: &str) -> (u32, u32) {
    let (mut max, mut mid, mut low) = (0, 0, 0);
    let sums = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<u32>().unwrap()).sum());
    for sum in sums {
        if sum > max {
            low = mid;
            mid = max;
            max = sum;
        } else if sum > mid {
            low = mid;
            mid = sum;
        } else if sum > low {
            low = sum;
        }
    }
    (max, max + mid + low)
}
