// use std::env::args;

pub mod solutions;

fn main() {
    // let path = args().last().expect("provide the file as the last command line argument");

    println!("{}", solutions::day2::first("resources/2.txt"));
}
