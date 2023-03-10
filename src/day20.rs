use std::array::from_fn;

pub fn run() -> String {
    let mut test = File {
        nums: [(1, 0), (2, 1), (-3, 2), (3, 3), (-2, 4), (0, 5), (4, 6)],
    };
    let mut input: File<5_000> = INPUT.into();
    test.mix();
    input.mix();
    format!("{}\n{}", test.calc_coordinates(), input.calc_coordinates())
}
#[derive(Debug)]
struct File<const T: usize> {
    nums: [(i64, i64); T],
}
impl<const T: usize> File<T> {
    fn mix(&mut self) {
        use std::cmp::Ordering::*;
        for i in 0..T {
            let (offset, idx) = self.nums[i];
            let target_idx = ((offset + idx) % T as i64 + T as i64) % T as i64;
            //     println!("{:?}", self);
            //     println!("{offset}, {idx}, {target_idx}");
            match (offset.cmp(&0), idx.cmp(&target_idx)) {
                (_, Equal) | (Equal, _) => (),

                (Greater, Less) => {
                    for (_, old_idx) in &mut self.nums {
                        if *old_idx == idx {
                            *old_idx = target_idx;
                        } else if *old_idx > idx && *old_idx <= target_idx {
                            *old_idx -= 1;
                        }
                    }
                }
                (Less, Greater) => {
                    for (_, old_idx) in &mut self.nums {
                        if *old_idx == idx {
                            *old_idx = target_idx
                        } else if *old_idx < idx && *old_idx >= target_idx {
                            *old_idx += 1;
                        }
                    }
                }
                (Less, Less) => {
                    for (_, old_idx) in &mut self.nums {
                        if *old_idx == idx {
                            *old_idx = target_idx;
                        } else if *old_idx == (T as i64 - 1) {
                            *old_idx = 0;
                        } else if *old_idx < idx || *old_idx >= target_idx {
                            *old_idx += 1;
                        }
                    }
                }
                (Greater, Greater) => {
                    for (_, old_idx) in &mut self.nums {
                        if *old_idx == idx {
                            *old_idx = target_idx;
                        } else if *old_idx == 0 {
                            *old_idx = T as i64 - 1;
                        } else if *old_idx > idx || *old_idx <= target_idx {
                            *old_idx -= 1;
                        }
                    }
                }
            }
            //     println!("{:?}\n", self);
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
