use std::fs::read_to_string;

pub fn first(path: &str) -> i64 {
   let file = read_to_string(path).expect("make sure the file is correct");

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in file.lines() {
        let numbers: Vec<u32> = line.split("   ")
            .map(|s| s.parse::<u32>().expect("please provide a correct file"))
            .collect();
        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    }

    first_list.sort();
    second_list.sort();

    let mut total_difference: i64 = 0;

    for i in 0..first_list.len() {
        let a = first_list[i];
        let b = second_list[i];
        total_difference += a.abs_diff(b) as i64;
    }

    total_difference
}

pub fn second(path: &str) -> i64 {
    let file = read_to_string(path).expect("make sure the file is correct");

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for line in file.lines() {
        let numbers: Vec<u32> = line.split("   ")
            .map(|s| s.parse::<u32>().expect("please provide a correct file"))
            .collect();
        first_list.push(numbers[0]);
        second_list.push(numbers[1]);
    }

    let mut similarity_score: i64 = 0;

    for first in first_list.iter() {
        for second in second_list.iter() {
            similarity_score += if second == first { *second as i64 } else { 0 };
        }
    }

    similarity_score
}
