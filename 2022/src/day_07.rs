/*
    --- Day 7: No Space Left On Device ---
    You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

    The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

    $ system-update --please --pretty-please-with-sugar-on-top
    Error: No space left on device
    Perhaps you can delete some files to make space for the update?

    You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

    $ cd /
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
    The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

    Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

    cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
    cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
    cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
    cd / switches the current directory to the outermost directory, /.
    ls means list. It prints out all of the files and directories immediately contained by the current directory:
    123 abc means that the current directory contains a file named abc with size 123.
    dir xyz means that the current directory contains a directory named xyz.
    Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

    - / (dir)
    - a (dir)
        - e (dir)
        - i (file, size=584)
        - f (file, size=29116)
        - g (file, size=2557)
        - h.lst (file, size=62596)
    - b.txt (file, size=14848514)
    - c.dat (file, size=8504156)
    - d (dir)
        - j (file, size=4060174)
        - d.log (file, size=8033020)
        - d.ext (file, size=5626152)
        - k (file, size=7214296)
    Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

    Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

    The total sizes of the directories above can be found as follows:

    The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
    The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
    Directory d has total size 24933642.
    As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
    To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

    Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

    --- Part Two ---
    Now, you're ready to choose a directory to delete.

    The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

    In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

    To achieve this, you have the following options:

    Delete directory e, which would increase unused space by 584.
    Delete directory a, which would increase unused space by 94853.
    Delete directory d, which would increase unused space by 24933642.
    Delete directory /, which would increase unused space by 48381165.
    Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

    Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
*/

use std::{cell::RefCell, collections::HashMap, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

// This is begging to be rewritten as a parser
lazy_static! {
    static ref RE_CD: Regex = Regex::new(r"cd (.*?)$").unwrap();
    static ref RE_DIR: Regex = Regex::new(r"dir (.*?)$").unwrap();
    static ref RE_FILE: Regex = Regex::new(r"(\d+) (.*?)$").unwrap();
}

#[derive(Debug)]
pub enum Cd {
    In(String),
    Out,
    Root,
}

impl FromStr for Cd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE_CD.captures(s).ok_or(())?;
        let kind = match caps.get(1).unwrap().as_str() {
            ".." => Self::Out,
            "/" => Self::Root,
            x => Self::In(x.to_owned()),
        };
        Ok(kind)
    }
}

#[derive(Debug)]
pub enum Command {
    Cd(Cd),
    Ls,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("$ ").unwrap_or(s);
        if let Ok(cd) = Cd::from_str(s) {
            Ok(Self::Cd(cd))
        } else if s.starts_with("ls") {
            Ok(Self::Ls)
        } else {
            Err(())
        }
    }
}

#[expect(dead_code)]
#[derive(Clone, Debug)]
pub enum Entry {
    Dir(String),
    File(String, u64),
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(dir_caps) = RE_DIR.captures(s) {
            let name = dir_caps.get(1).unwrap().as_str().to_owned();
            Ok(Self::Dir(name))
        } else {
            let file_caps = RE_FILE.captures(s).ok_or(())?;
            let size = file_caps.get(1).unwrap().as_str().parse().unwrap();
            let name = file_caps.get(2).unwrap().as_str().to_owned();
            Ok(Self::File(name, size))
        }
    }
}

#[derive(Debug)]
pub enum CmdOrOut {
    Cmd(Command),
    Output(Entry),
}

impl FromStr for CmdOrOut {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(cmd) = Command::from_str(s) {
            Ok(Self::Cmd(cmd))
        } else if let Ok(entry) = Entry::from_str(s) {
            Ok(Self::Output(entry))
        } else {
            Err(())
        }
    }
}

struct Fs {
    dirs: HashMap<String, Vec<Entry>>,
    dir_size_cache: RefCell<HashMap<String, u64>>,
}

impl Fs {
    fn new(input: &[CmdOrOut]) -> Self {
        let mut dirs = HashMap::new();
        let mut current_path = Vec::new(); // Assume we start at root
        for inp in input {
            match inp {
                CmdOrOut::Cmd(cmd) => match cmd {
                    Command::Cd(cd) => match cd {
                        Cd::In(name) => current_path.push(name.clone()),
                        Cd::Out => {
                            current_path.pop();
                        }
                        Cd::Root => current_path = Vec::new(),
                    },
                    Command::Ls => (),
                },
                CmdOrOut::Output(entry) => {
                    let mut path = current_path.join("/");
                    path.insert(0, '/');
                    let dir = dirs.entry(path).or_insert_with(Vec::new);
                    dir.push(entry.clone());
                }
            }
        }

        Fs {
            dirs,
            dir_size_cache: RefCell::new(HashMap::new()),
        }
    }

    fn size_of_dir(&self, dir: &str) -> u64 {
        if let Some(size) = self.dir_size_cache.borrow().get(dir) {
            return *size;
        }

        let size = self
            .dirs
            .get(dir)
            .unwrap()
            .iter()
            .map(|e| match e {
                Entry::Dir(name) => {
                    let mut path = dir.to_owned();
                    if !path.ends_with('/') {
                        path.push('/');
                    }
                    path.push_str(name);
                    self.size_of_dir(&path)
                }
                Entry::File(_, size) => *size,
            })
            .sum();

        self.dir_size_cache
            .borrow_mut()
            .insert(dir.to_owned(), size);
        size
    }

    fn total_sizes_at_most_100k(&self) -> u64 {
        self.dirs
            .keys()
            .map(|dir| self.size_of_dir(dir))
            .filter(|size| *size <= 100_000)
            .sum()
    }

    fn smallest_to_delete(&self, total_space: u64, required_space: u64) -> u64 {
        let used_space = self.size_of_dir("/");
        assert!(total_space >= used_space);
        let space_to_delete = required_space - (total_space - used_space);

        self.dirs
            .keys()
            .map(|dir| self.size_of_dir(dir))
            .filter(|size| *size >= space_to_delete)
            .min()
            .unwrap()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<CmdOrOut> {
    input
        .lines()
        .map(CmdOrOut::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day7, part1)]
pub fn part1(input: &[CmdOrOut]) -> u64 {
    let fs = Fs::new(input);
    let total = fs.total_sizes_at_most_100k();
    assert_eq!(total, 1453349);
    total
}

#[aoc(day7, part2)]
pub fn part2(input: &[CmdOrOut]) -> u64 {
    let fs = Fs::new(input);
    let to_delete = fs.smallest_to_delete(70_000_000, 30_000_000);
    assert_eq!(to_delete, 2948823);
    to_delete
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
$ cd /
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
7214296 k";

    #[test]
    fn test_total_sizes_at_most_100k() {
        let input = input_generator(EXAMPLE_INPUT);
        let fs = Fs::new(&input);
        let x = fs.total_sizes_at_most_100k();
        assert_eq!(x, 95437);
    }

    #[test]
    fn test_smallest_to_delete() {
        let input = input_generator(EXAMPLE_INPUT);
        let fs = Fs::new(&input);
        let x = fs.smallest_to_delete(70_000_000, 30_000_000);
        assert_eq!(x, 24933642);
    }
}
