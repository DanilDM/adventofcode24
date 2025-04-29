// use std::env::args;

pub mod solutions;

fn main() {
    // let path = args().last().expect("provide the file as the last command line argument");

    println!("{}", solutions::day3::first("resources/3.txt"));
}
