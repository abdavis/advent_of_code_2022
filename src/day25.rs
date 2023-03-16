use std::fmt::Display;

const INPUT: &str = include_str!("inputs/day25.txt");
pub fn run() -> String {
    format!(
        "{}",
        SnafuNum::from(
            INPUT
                .lines()
                .map(|l| isize::from(&SnafuNum::from(l)))
                .sum::<isize>()
        )
    )
}
struct SnafuNum(Vec<SnafuDigit>);
impl From<&str> for SnafuNum {
    fn from(value: &str) -> Self {
        let mut vec: Vec<SnafuDigit> = value.chars().map(|c| c.into()).collect();
        vec.reverse();
        Self(vec)
    }
}
impl From<isize> for SnafuNum {
    fn from(mut value: isize) -> Self {
        let mut out = vec![];
        while value > 0 {
            match value % 5 {
                d @ 0 | d @ 1 | d @ 2 => {
                    value /= 5;
                    out.push(SnafuDigit::from(d));
                }
                d @ 3 => {
                    out.push(SnafuDigit::DoubleMinus);
                    value /= 5;
                    value += 1;
                }
                d @ 4 => {
                    out.push(SnafuDigit::Minus);
                    value /= 5;
                    value += 1;
                }
                d => panic!("{value} % 5 = {d}, which is not in proper range"),
            }
        }

        Self(out)
    }
}
impl Display for SnafuNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.0.iter().rev() {
            write!(f, "{}", char::from(digit))?;
        }
        Ok(())
    }
}

enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl From<char> for SnafuDigit {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '1' => Self::One,
            '0' => Self::Zero,
            '-' => Self::Minus,
            '=' => Self::DoubleMinus,
            c => panic!("{c} is not a valid snafu digit"),
        }
    }
}

impl From<&SnafuDigit> for char {
    fn from(value: &SnafuDigit) -> Self {
        match value {
            SnafuDigit::Two => '2',
            SnafuDigit::One => '1',
            SnafuDigit::Zero => '0',
            SnafuDigit::Minus => '-',
            SnafuDigit::DoubleMinus => '=',
        }
    }
}

impl From<isize> for SnafuDigit {
    fn from(value: isize) -> Self {
        match value {
            2 => Self::Two,
            1 => Self::One,
            0 => Self::Zero,
            -1 => Self::Minus,
            -2 => Self::DoubleMinus,
            n => panic!("{n} cannot be a snafu digit"),
        }
    }
}

impl From<&SnafuNum> for isize {
    fn from(value: &SnafuNum) -> Self {
        let mut out = 0;
        for (i, num) in value.0.iter().enumerate() {
            out += isize::from(num) * 5isize.pow(i as u32);
        }
        out
    }
}
impl From<&SnafuDigit> for isize {
    fn from(value: &SnafuDigit) -> Self {
        match value {
            SnafuDigit::Two => 2,
            SnafuDigit::One => 1,
            SnafuDigit::Zero => 0,
            SnafuDigit::Minus => -1,
            SnafuDigit::DoubleMinus => -2,
        }
    }
}

const PRACTICE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
