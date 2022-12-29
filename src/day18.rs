const INPUT: &str = include_str!("inputs/day18.txt");
const EXAMPLE: &str = include_str!("example_inputs/day18.txt");
pub fn run() -> String {
    let voxels: Voxels<20> = INPUT.into();
    format!("{}\n{}", voxels.surface_area(), voxels.outside_area())
}

struct Voxels<const N: usize>([[[State; N]; N]; N]);
#[derive(Clone, Copy)]
enum State {
    Empty,
    Lava,
    Outside,
}
impl<const N: usize> Voxels<N> {
    fn surface_area(&self) -> usize {
        let mut count = 0;
        for x in 0..N {
            for y in 0..N {
                for z in 0..N {
                    if let State::Lava = self.0[x][y][z] {
                        if x == 0 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x - 1][y][z] {
                            count += 1;
                        }
                        if x == N - 1 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x + 1][y][z] {
                            count += 1;
                        }
                        if y == 0 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x][y - 1][z] {
                            count += 1;
                        }
                        if y == N - 1 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x][y + 1][z] {
                            count += 1;
                        }
                        if z == 0 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x][y][z - 1] {
                            count += 1;
                        }
                        if z == N - 1 {
                            count += 1;
                        } else if let State::Empty | State::Outside = self.0[x][y][z + 1] {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
    fn outside_area(&self) -> usize {
        let mut count = 0;
        for x in 0..N {
            for y in 0..N {
                for z in 0..N {
                    if let State::Lava = self.0[x][y][z] {
                        if x == 0 {
                            count += 1;
                        } else if let State::Outside = self.0[x - 1][y][z] {
                            count += 1;
                        }
                        if x == N - 1 {
                            count += 1;
                        } else if let State::Outside = self.0[x + 1][y][z] {
                            count += 1;
                        }
                        if y == 0 {
                            count += 1;
                        } else if let State::Outside = self.0[x][y - 1][z] {
                            count += 1;
                        }
                        if y == N - 1 {
                            count += 1;
                        } else if let State::Outside = self.0[x][y + 1][z] {
                            count += 1;
                        }
                        if z == 0 {
                            count += 1;
                        } else if let State::Outside = self.0[x][y][z - 1] {
                            count += 1;
                        }
                        if z == N - 1 {
                            count += 1;
                        } else if let State::Outside = self.0[x][y][z + 1] {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
    fn set_outside(&mut self) {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 0));
        while let Some((x, y, z)) = queue.pop_front() {
            if let s @ State::Empty = &mut self.0[x][y][z] {
                *s = State::Outside;
                if x > 0 {
                    queue.push_back((x - 1, y, z));
                }
                if x < N - 1 {
                    queue.push_back((x + 1, y, z));
                }
                if y > 0 {
                    queue.push_back((x, y - 1, z));
                }
                if y < N - 1 {
                    queue.push_back((x, y + 1, z));
                }
                if z > 0 {
                    queue.push_back((x, y, z - 1));
                }
                if z < N - 1 {
                    queue.push_back((x, y, z + 1));
                }
            }
        }
    }
}
impl<const N: usize> From<&str> for Voxels<N> {
    fn from(value: &str) -> Self {
        let mut nums = value
            .split(['\n', ','])
            .filter_map(|n| n.parse::<usize>().ok());
        let mut voxels = [[[State::Empty; N]; N]; N];
        while let (Some(x), Some(y), Some(z)) = (nums.next(), nums.next(), nums.next()) {
            voxels[x][y][z] = State::Lava;
        }
        let mut out = Self(voxels);
        out.set_outside();

        out
    }
}
