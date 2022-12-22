use core::iter::Peekable;
const INPUT: &str = include_str!("inputs/day07.txt");
const TEST: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

const SPACE_NEEDED: usize = 30_000_000;
const TOTAL_SPACE: usize = 70_000_000;
pub fn run() -> String {
    let tree: FileSystem = INPUT.into();

    format!(
        "{}\n{}",
        tree.get_size_sum().1,
        tree.deleted_folder_size().1
    )
}
#[derive(Debug)]
enum FileSystem {
    Dir {
        name: String,
        content: Vec<FileSystem>,
    },
    File {
        name: String,
        size: usize,
    },
}
impl FileSystem {
    fn deleted_folder_size(&self) -> (usize, usize) {
        let used = self.get_size();
        let available = TOTAL_SPACE - used;
        let needed = SPACE_NEEDED - available;
        self.find_smallest(needed)
    }
    fn find_smallest(&self, needed: usize) -> (usize, usize) {
        use std::cmp::min;
        match self {
            Self::File { size, name: _ } => (*size, usize::MAX),
            Self::Dir { name: _, content } => {
                let mut minimum = usize::MAX;
                let mut folder_size = 0;
                for val in content {
                    let (size, mini_val) = val.find_smallest(needed);
                    folder_size += size;
                    minimum = min(minimum, mini_val);
                }
                if folder_size >= needed {
                    minimum = min(minimum, folder_size);
                }
                (folder_size, minimum)
            }
        }
    }
    fn get_size(&self) -> usize {
        match self {
            Self::File { name, size } => *size,
            Self::Dir { name, content } => {
                let mut sum = 0;
                for val in content {
                    sum += val.get_size();
                }
                sum
            }
        }
    }
    fn get_size_sum(&self) -> (usize, usize) {
        const size_cutoff: usize = 100_000;
        match self {
            Self::File { name: _, size } => (*size, 0),
            Self::Dir { name: _, content } => {
                let mut folder_size = 0;
                let mut sum = 0;
                for val in content {
                    let (val_size, val_sum) = val.get_size_sum();
                    folder_size += val_size;
                    sum += val_sum;
                }
                if folder_size <= size_cutoff {
                    sum += folder_size;
                }
                (folder_size, sum)
            }
        }
    }
    fn print_tree(&self) -> String {
        self.print_tree_recurse(0)
    }
    fn print_tree_recurse(&self, depth: usize) -> String {
        use std::iter::once;
        match self {
            Self::File { name, size } => {
                let mut out: String = (0..depth).map(|_| "|   ".to_string()).collect();
                out += &format!("{name} {size}\n");
                out
            }
            Self::Dir { name, content } => {
                let mut out: String = (0..depth).map(|_| "|   ".to_string()).collect();
                out += &format!("{name}\n");
                for val in content {
                    out.push_str(&val.print_tree_recurse(depth + 1));
                }
                out
            }
        }
    }
}
impl From<&str> for FileSystem {
    fn from(input: &str) -> Self {
        let mut cmds = input.split_terminator("$ ").skip(1).peekable();
        (&mut cmds).into()
    }
}
impl<'a, T: Iterator<Item = &'a str>> From<&mut Peekable<T>> for FileSystem {
    fn from(input: &mut Peekable<T>) -> Self {
        let name = input
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .into();
        let mut content: Vec<Self> = input
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .filter_map(|l| {
                if l.starts_with("dir") {
                    None
                } else {
                    let mut words = l.split_whitespace();
                    let size = words.next().unwrap().parse().unwrap();
                    let name = words.next().unwrap().into();
                    Some(Self::File { name, size })
                }
            })
            .collect();
        while let Some(cmd) = input.peek() {
            if cmd.starts_with("cd ..") {
                input.next();
                break;
            }
            content.push(input.into());
        }
        Self::Dir { name, content }
    }
}
