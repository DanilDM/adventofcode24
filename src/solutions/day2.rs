use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn first(path: &str) -> u32 {
    let file = File::open(path).expect("provide a valid file");

    let reader = BufReader::new(file);
    
    let mut safe_reports_amount = 0;

    for (i, line) in reader.lines().enumerate() {
        let msg = format!("line {} is not formatted correctly", i);
        let nums = line.expect(&msg).split(' ').map(|s| s.parse::<u32>().expect(&msg)).collect::<Vec<u32>>();

        let valid = check_sequence(&nums);

        safe_reports_amount += valid as u32;
    }

    safe_reports_amount
}

pub fn second(path: &str) -> u32 {
    let file = File::open(path).expect("provide a valid file");

    let reader = BufReader::new(file);
    
    let mut safe_reports_amount = 0;

    for (i, line) in reader.lines().enumerate() {
        let msg = format!("line {} is not formatted correctly", i);
        let nums = line.expect(&msg).split(' ').map(|s| s.parse::<u32>().expect(&msg)).collect::<Vec<u32>>();

        let mut valid = check_sequence(&nums);

        for i in 0..nums.len() {
            let mut clone = nums.clone();
            clone.remove(i);
            valid = valid || check_sequence(&clone);
        }

        safe_reports_amount += valid as u32;
    }

    safe_reports_amount
}

fn check_sequence(nums: &Vec<u32>) -> bool {
    let mut valid = true;
    let increasing = nums[1] > nums[0];

    for i in 1..nums.len() {
        if nums[i].abs_diff(nums[i-1]) > 3 
            || nums[i] == nums[i-1] 
            || ((nums[i]>nums[i-1]) != increasing)
        { valid = false; }
    }

    valid
}