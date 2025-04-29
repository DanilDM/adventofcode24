use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn first(path: &str) -> i64 {
    let file = File::open(path).expect("provide a valid file");

    let reader = BufReader::new(file);

    let mut answer: i64 = 0;
    for line in reader.lines() {
        let line = line.expect("sths wrong with the lines");
        let mut stage = Start;
        let mut line_result: i64 = 0;
        for ch in line.chars() {
            match ch {
                'm' => {
                    if let Start = stage { stage = M; } else { stage = Start; }
                }
                'u' => {
                    if let M = stage { stage = U; } else { stage = Start; }
                }
                'l' => {
                    if let U = stage { stage = L; } else { stage = Start; }
                }
                '(' => {
                    if let L = stage { stage = Opbr; } else { stage = Start; }
                }
                '0'..='9' => {
                    let digit = ch.to_digit(10).unwrap();

                    if let Opbr = stage { stage = N1( digit ); }
                    else if let N1(num) = stage { stage = N1( num*10 + digit ); }
                    
                    else if let Sep(num) = stage { stage = N2( num, digit ); }
                    else if let N2(num1, num2) = stage { stage = N2( num1, num2*10 + digit ); }

                    else { stage = Start; }
                }
                ',' => {
                    if let N1(num) = stage { stage = Sep(num); } else { stage = Start; }
                }
                ')' => {
                    if let N2(num1, num2) = stage {
                        line_result += num1 as i64 * num2 as i64;
                    }
                    stage = Start;
                }
                _ => { stage = Start; }
            }
        }
        answer += line_result;
    }
    answer
}

enum Stage {
    Start,
    M,
    U,
    L,
    Opbr,
    N1(u32),
    Sep(u32),
    N2(u32, u32)
} use Stage::*;
