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
pub fn run() -> String {
    todo!()
}

enum FileSystem {
    Dir {
        name: String,
        size: u32,
        content: Vec<FileSystem>,
    },
    File {
        name: String,
        size: u32,
    },
}
