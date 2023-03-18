use std::array::from_fn;

pub fn run() -> String {
    let mut input: File<5_000> = INPUT.into();
    let mut input2 = input.clone();
    input.mix(1);
    input2.decrypt(811589153);
    input2.mix(10);
    format!(
        "{}\n{}",
        input.calc_coordinates(),
        input2.calc_coordinates()
    )
}
#[derive(Debug, Clone)]
struct File<const T: usize> {
    nums: [(i64, i64); T],
}
impl<const T: usize> File<T> {
    fn mix(&mut self, loops: usize) {
        for _ in 0..loops {
            for i in 0..T {
                let (offset, idx) = self.nums[i];
                let target_idx =
                    ((offset + idx) % (T - 1) as i64 + (T - 1) as i64) % (T - 1) as i64;
                for (_, modifying_idx) in &mut self.nums {
                    if *modifying_idx == idx {
                        *modifying_idx = target_idx;
                    } else if *modifying_idx < idx && *modifying_idx >= target_idx {
                        *modifying_idx += 1;
                    } else if *modifying_idx > idx && *modifying_idx <= target_idx {
                        *modifying_idx -= 1;
                    }
                }
            }
        }
        self.nums.sort_unstable_by_key(|(_, idx)| *idx);
        // println!("{:?}\n", self);
    }
    fn calc_coordinates(&self) -> i64 {
        let zero_idx = self.nums.iter().position(|(val, _)| *val == 0).unwrap();
        let first = self.nums[(zero_idx + 1_000) % T].0;
        let second = self.nums[(zero_idx + 2_000) % T].0;
        let third = self.nums[(zero_idx + 3_000) % T].0;
        first + second + third
    }
    fn decrypt(&mut self, key: i64) {
        for (num, _) in &mut self.nums {
            *num *= key;
        }
    }
}

impl<const T: usize> From<&str> for File<T> {
    fn from(value: &str) -> Self {
        let mut parser = value.lines().map(|l| l.parse().unwrap());
        Self {
            nums: from_fn(|i| (parser.next().unwrap(), i as i64)),
        }
    }
}
const INPUT: &str = include_str!("inputs/day20.txt");
