use std::cmp::min;
const INPUT: &str = include_str!("inputs/day08.txt");
pub fn run() -> String {
    let forest: Forest<99> = INPUT.into();
    format!("{}\n{}", forest.count_visible(), forest.scenic_score())
}
struct Forest<const N: usize>([[u8; N]; N]);
impl<const N: usize> Forest<N> {
    fn count_visible(&self) -> usize {
        let mut total = 0;
        for row in 1..N - 1 {
            for col in 1..N - 1 {
                if self.0[row][col] > (0..col).map(|col| self.0[row][col]).max().unwrap()
                    || self.0[row][col] > (col + 1..N).map(|col| self.0[row][col]).max().unwrap()
                    || self.0[row][col] > (0..row).map(|row| self.0[row][col]).max().unwrap()
                    || self.0[row][col] > (row + 1..N).map(|row| self.0[row][col]).max().unwrap()
                {
                    total += 1;
                }
            }
        }
        total + (N * N - (N - 2) * (N - 2))
    }
    fn scenic_score(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().map(move |(c, tree)| {
                    min(
                        c,
                        (0..c)
                            .rev()
                            .map(|c| self.0[r][c])
                            .take_while(|n| n < tree)
                            .count()
                            + 1,
                    ) * min(
                        N - c - 1,
                        (c + 1..N)
                            .map(|c| self.0[r][c])
                            .take_while(|n| n < tree)
                            .count()
                            + 1,
                    ) * min(
                        r,
                        (0..r)
                            .rev()
                            .map(|r| self.0[r][c])
                            .take_while(|n| n < tree)
                            .count()
                            + 1,
                    ) * min(
                        N - r - 1,
                        (r + 1..N)
                            .map(|r| self.0[r][c])
                            .take_while(|n| n < tree)
                            .count()
                            + 1,
                    )
                })
            })
            .max()
            .unwrap()
    }
}
impl<const N: usize> From<&str> for Forest<N> {
    fn from(input: &str) -> Self {
        use std::array::from_fn;
        let mut nums = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as u8);
        Self(from_fn(|_| {
            from_fn(|_| {
                nums.next().expect(&format!(
                    "input doesn't have enough chars for forest of size [{N} X {N}]!"
                ))
            })
        }))
    }
}
