use std::collections::HashMap;

use crate::Output;

pub fn run(input: &str) -> Output {
    let mut lines = input.lines();

    // part 1
    let mut dirs: HashMap<String, u32> = HashMap::new();
    let mut files: HashMap<String, u32> = HashMap::new();
    let mut cwd: Vec<String> = Vec::new();
    loop {
        match lines.next() {
            None => break, // end of output
            Some(line) => {
                let cmd = line.split(" ").collect::<Vec<&str>>();
                match cmd[0] {
                    "$" => {
                        match cmd[1] {
                            "cd" => match cmd[2] {
                                "/" => {
                                    cwd.clear();
                                }
                                ".." => {
                                    cwd.pop();
                                }
                                dirname => {
                                    cwd.push(dirname.to_string());
                                }
                            },
                            "ls" => continue, // ignore
                            _ => panic!("Unknown command: {}", line),
                        }
                    }
                    "dir" => continue, // do nothing
                    // file: do a buncha work
                    size_raw => {
                        let filesize = size_raw.parse::<u32>().unwrap();
                        // add to files
                        let basename = cmd[1];
                        let mut filepath = "/".to_owned();
                        filepath.push_str(&cwd.join("/"));
                        filepath.push_str(basename);
                        files.insert(filepath, filesize);
                        // update dirs sizes
                        let mut cwd_str = "/".to_owned();
                        for dir in &cwd {
                            cwd_str.push_str(&dir);
                            cwd_str.push_str("/");
                            let dirsize = dirs.get(&cwd_str).or(Some(&0)).unwrap();
                            dirs.insert(cwd_str.clone(), dirsize + filesize);
                        }
                    }
                }
            }
        }
    }

    let mut total_dirsize: u32 = 0;
    for (_, dirsize) in &dirs {
        if dirsize <= &100000 {
            total_dirsize += dirsize;
        }
    }

    // part 2
    let system_max = 70000000;
    let needed = 30000000;
    let mut root_dirsize: u32 = 0;
    for (_, filesize) in &files {
        root_dirsize += filesize;
    }
    let unused = system_max - root_dirsize;
    let mut smallest_dirsize: u32 = system_max; // max size
    for (_, dirsize) in &dirs {
        if dirsize < &smallest_dirsize && dirsize + unused >= needed {
            smallest_dirsize = dirsize.clone();
        }
    }

    return Output {
        part1: total_dirsize.to_string(),
        part2: smallest_dirsize.to_string(),
    };
}
