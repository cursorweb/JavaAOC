use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::read;

pub fn run() {
    // 1st = $ cd /
    let file = read!().skip(1);

    let folder = Folder::init();

    let mut curr_folder = Rc::clone(&folder);

    for line in file {
        if line.starts_with('$') {
            let line = &line[2..];
            if line.starts_with("cd") {
                let folder = &line[3..];

                if folder == ".." {
                    let folder = Rc::clone(
                        &curr_folder
                            .borrow()
                            .parent
                            .as_ref() // &Option<T> -> Option<&T>
                            .expect("Parent should not be None")
                            .upgrade()
                            .expect("Parent should not be dropped"),
                    );

                    curr_folder = folder;
                } else {
                    let entry = Rc::clone(
                        curr_folder
                            .borrow_mut()
                            .child
                            .entry(folder)
                            .or_insert(Folder::new(&curr_folder)),
                    );

                    curr_folder = entry;
                }
            }

            // we ignore ls because that's just 'syntax sugar'
        } else if !line.starts_with("dir") {
            // we ignore dir because if you don't cd to it, it's an empty folder.
            let num: i32 = line
                .split(" ")
                .next()
                .unwrap()
                .parse()
                .expect("Should be a number");
            curr_folder.borrow_mut().file_size += num;
        }
    }

    let mut folder_sizes = vec![];
    let total_size = folder.borrow().size(&mut folder_sizes);

    let part1: i32 = folder_sizes.iter().filter(|&&n| n <= 100_000).sum();
    
    let disc_space = 70_000_000;
    let unused_space = disc_space - total_size;
    let unused = 30_000_000;

    // this is the number to be bigger than!
    let space_to_be_freed = unused - unused_space;

    let mut folder_sizes: Vec<i32> = folder_sizes.into_iter().filter(|&n| n >= space_to_be_freed).collect();
    folder_sizes.sort();

    println!("Part1: {part1}");
    println!("Part2: {}", folder_sizes[0]);
}

#[derive(Debug)]
struct Folder<'a> {
    file_size: i32,
    child: HashMap<&'a str, Rc<RefCell<Folder<'a>>>>,
    parent: Option<Weak<RefCell<Folder<'a>>>>,
}

impl<'a> Folder<'a> {
    fn new(parent: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            file_size: 0,
            child: HashMap::new(),
            parent: Some(Rc::downgrade(parent)),
        }))
    }

    fn init() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            file_size: 0,
            child: HashMap::new(),
            parent: None,
        }))
    }

    fn size(&self, acc: &mut Vec<i32>) -> i32 {
        let sum = self
            .child
            .values()
            .fold(self.file_size, |p, f| p + f.borrow().size(acc));
        acc.push(sum);
        return sum;
    }
}
