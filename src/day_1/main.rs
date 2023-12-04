use std::fs;

fn main() {
    //Get file input
    let mut file_input =
        fs::read_to_string("inputs/day_1.txt").expect("Problem reading input file");

    println!("Part one:");
    part_one(&file_input);
    println!("\nPart two:");
    part_two(&mut file_input);
}

fn part_one(input: &str) {
    let mut sum = 0;

    //Iterate through input
    for line in input.lines() {
        let (mut tens_place, mut ones_place) = ('0', '0');

        for ch in line.chars() {
            if ch.is_ascii_digit() {
                tens_place = ch;
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                ones_place = ch;
                break;
            }
        }

        let num = 10 * tens_place.to_digit(10).unwrap() + ones_place.to_digit(10).unwrap();
        println!("line: {line}, chars: {tens_place}{ones_place}, num: {num}");
        sum += num;
    }
    println!("Sum: {}", sum);
}
fn part_two(input: &mut str) {
    let mut sum = 0;

    //Iterate through input
    for line in input.lines() {
        let (mut tens_place, mut ones_place) = (0, 0);

        for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                tens_place = 10 * ch.to_digit(10).unwrap();
                break;
            } else {
                let digit = word_to_digit(line, i);
                if let Some(d) = digit {
                    tens_place = d * 10;
                    break;
                }
            }
        }

        for (i, ch) in line.chars().rev().enumerate() {
            if ch.is_ascii_digit() {
                ones_place = ch.to_digit(10).unwrap();
                break;
            } else {
                let digit = word_to_digit(line, line.len() - 1 - i);
                if let Some(d) = digit {
                    ones_place = d;
                    break;
                }
            }
        }

        let num = tens_place + ones_place;
        println!("line: {line}, places: {tens_place} {ones_place}, num: {num}");
        sum += num;
    }
    println!("Sum: {}", sum);
}

fn word_to_digit(line: &str, i: usize) -> Option<u32> {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for idx in 0..10 {
        let word: &str = words[idx];
        // println!("{}", String::from(&line[i..i + word.len()]));
        if line.len() - i >= word.len() && word.contains(&line[i..i + word.len()]) {
            return Some(idx as u32);
        }
    }
    None
}
