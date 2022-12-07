use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    parent_dir: Weak<RefCell<Directory>>,
    sub_dirs: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: HashMap::new(),
            parent_dir: Weak::new(),
            sub_dirs: HashMap::new(),
        }
    }
    fn total_size(&self) -> usize {
        let mut sum: usize = self.files.iter().map(|(_, f)| f.size).sum();
        for (_, dir) in self.sub_dirs.iter() {
            sum += dir.borrow().total_size();
        }
        sum
    }
    fn print(&self, prefix_spaces: usize) {
        let prefix = " ".repeat(prefix_spaces);
        let prefix_contained = " ".repeat(prefix_spaces + 2);
        println!("{}{} (dir)", &prefix, self.name);
        for (_, file) in self.files.iter() {
            println!(
                "{}{} (file, size={})",
                &prefix_contained, file.name, file.size
            );
        }
        for (_, dir) in self.sub_dirs.iter() {
            dir.borrow().print(prefix_spaces + 2);
        }
    }
    fn sum_of_smaller_dirs(&self, inclusive_threshold: usize) -> usize {
        let mut sum = 0;
        for (_, subdir) in self.sub_dirs.iter() {
            let subdir_size = subdir.borrow().total_size();
            if subdir_size <= inclusive_threshold {
                sum += subdir_size;
            }
            sum += subdir.borrow().sum_of_smaller_dirs(inclusive_threshold);
        }
        sum
    }
    fn get_dir_sizes_suitable_for_deletion(&self, min_to_del: usize, vec_to_fill: &mut Vec<usize>) {
        let size = self.total_size();
        if size >= min_to_del {
            vec_to_fill.push(size);
        }
        for (_, subdir) in self.sub_dirs.iter() {
            subdir
                .borrow()
                .get_dir_sizes_suitable_for_deletion(min_to_del, vec_to_fill);
        }
    }
    // fn get_foo(w: Weak<RefCell<Directory>>) -> Directory {
    // }
}

struct TaskData {
    root: Rc<RefCell<Directory>>,
}

fn parse_input(input: &str) -> Result<TaskData> {
    lazy_static! {
        static ref CD_RE: Regex = Regex::new(r"\$ cd (.*)").unwrap();
        static ref LS_RE: Regex = Regex::new(r"\$ ls").unwrap();
        static ref DIR_RE: Regex = Regex::new(r"dir (.*)").unwrap();
        static ref FILE_RE: Regex = Regex::new(r"(\d+) (.*)").unwrap();
    }
    let root = Rc::new(RefCell::new(Directory::new(String::from("/"))));
    let mut current = Rc::downgrade(&root);
    for line in input.lines().skip(1) {
        let rc_current = current.upgrade().unwrap();
        if let Some(caps) = CD_RE.captures(line) {
            // change current
            match &caps[1] {
                ".." => {
                    let parent = &rc_current.borrow().parent_dir;
                    current = Weak::clone(&parent);
                }
                name => {
                    let subdir = &rc_current.borrow().sub_dirs[name];
                    current = Rc::downgrade(&subdir);
                }
            }
        } else if LS_RE.is_match(line) {
            // do nothing
        } else if let Some(caps) = DIR_RE.captures(line) {
            // add dir to current
            let name = String::from(&caps[1]);
            let new_dir = Rc::new(RefCell::new(Directory::new(name.clone())));
            new_dir.borrow_mut().parent_dir = Rc::downgrade(&rc_current);
            rc_current
                .borrow_mut()
                .sub_dirs
                .insert(name.clone(), new_dir);
        } else if let Some(caps) = FILE_RE.captures(line) {
            // add file to current
            let size = usize::from_str_radix(&caps[1], 10).unwrap();
            let name = String::from(&caps[2]);
            rc_current
                .borrow_mut()
                .files
                .insert(name.clone(), File { name, size });
        } else {
            println!("{}", line);
            panic!("unknown line");
        }
    }
    Ok(TaskData { root })
}

fn part_one(input: &str) -> Result<usize> {
    let TaskData { root } = parse_input(input)?;
    root.borrow().print(0);
    let answer = root.borrow().sum_of_smaller_dirs(100000);
    Ok(answer)
}

fn part_two(input: &str) -> Result<usize> {
    let TaskData { root } = parse_input(input)?;
    let total_size = root.borrow().total_size();
    let total_disk = 70000000;
    let need_free = 30000000;
    let current_free = total_disk - total_size;
    let min_space_to_free = need_free - current_free;
    let mut suitable_for_deletion = Vec::new();
    root.borrow()
        .get_dir_sizes_suitable_for_deletion(min_space_to_free, &mut suitable_for_deletion);
    suitable_for_deletion.sort();
    Ok(suitable_for_deletion[0])
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::fs;

    lazy_static! {
        static ref TEST: String = read_from_file("test.txt");
        static ref INPUT: String = read_from_file("input.txt");
    }

    fn read_from_file(filename: &str) -> String {
        fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, 95437);
        Ok(())
    }

    // Use "cargo test --release -- part_one --nocapture" to print the time
    #[test]
    fn part_one() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_one(&INPUT)?;
        eprintln!("Part one took {:0.2?}", t.elapsed());
        assert_eq!(answer, 1908462);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, 24933642);
        Ok(())
    }

    // Use "cargo test --release -- part_two --nocapture" to print the time
    #[test]
    fn part_two() -> Result<()> {
        let t = std::time::Instant::now();
        let answer = super::part_two(&INPUT)?;
        eprintln!("Part two took {:0.2?}", t.elapsed());
        assert_eq!(answer, 3979145);
        Ok(())
    }
}
