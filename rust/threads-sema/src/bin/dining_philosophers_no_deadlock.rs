use lazy_static::*;
use std::env::args;
use std::io::{self, Write};
use std::thread;
use std_semaphore::Semaphore;

lazy_static! {
    static ref FORKS: Vec<Semaphore> = vec![
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1)
    ];
}

fn think() {}

fn eat() {}

fn left(i: i32) -> usize {
    i as usize
}

fn right(i: i32) -> usize {
    ((i + 1) % 5) as usize
}

fn get_forks(i: i32) {
    if i == 4 {
        FORKS[right(i)].acquire();
        FORKS[left(i)].acquire();
    } else {
        FORKS[left(i)].acquire();
        FORKS[right(i)].acquire();
    }
}

fn put_forks(i: i32) {
    FORKS[right(i)].release();
    FORKS[left(i)].release();
}

fn philosopher(id: i32, num_loops: i32) {
    for _ in 0..num_loops {
        think();
        get_forks(id);
        eat();
        put_forks(id);
    }
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr
            .write(b"usage: dining_philosophers <num_loops>\n")
            .unwrap();
        std::process::exit(1);
    }
    let num_loops = args().nth(1).unwrap().parse::<i32>().unwrap();
    println!("dining: started");

    let mut handles = Vec::new();
    for i in 0..5 {
        handles.push(thread::spawn(move || philosopher(i as i32, num_loops)));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("dining: finished");
}
