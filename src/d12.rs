use core::num;
use std::collections::HashMap;

pub fn exec(input: String) -> () {
    let mut debug_int = 0;
    let mut count = 0;
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    for line in input.lines() {
        debug_int += 1;
        let mut number: i32 = 0;
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_numeric() {
                number = number + line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32 * 10;
                break;
            }
            if i + 4 >= line.len() {
                let substring = &line[i..];
                for (key, value) in &map {
                    if substring.contains(key) {
                        number = number + value * 10;
                        break;
                    }
                }
                if number != 0 {
                    break;
                }
                for c in substring.chars() {
                    if c.is_numeric() {
                        number = number + c.to_digit(10).unwrap() as i32 * 10;
                        break;
                    }
                }
                break;
            }
            let substring = &line[i..i + 5];
            let mut number_found = false;
            let mut number_found_value = 0;
            let mut number_found_key = 0;
            let mut char_found = false;
            let mut char_found_value = 0;
            let mut char_found_key = 0;
            for (key, value) in &map {
                if substring.contains(key) {
                    number_found = true;
                    number_found_value = value.clone();
                    number_found_key = substring.find(key).unwrap();
                    break;
                }
            }
            for c in substring.chars() {
                if c.is_numeric() {
                    char_found = true;
                    char_found_value = c.to_digit(10).unwrap() as i32;
                    char_found_key = substring.find(c).unwrap();
                    break;
                }
            }
            if number_found && char_found {
                if number_found_key < char_found_key {
                    number = number + number_found_value * 10;
                    break;
                } else {
                    number = number + char_found_value * 10;
                    break;
                }
            } else if number_found {
                number = number + number_found_value * 10;
                break;
            } else if char_found {
                number = number + char_found_value * 10;
                break;
            }
        }
        for i in (0..line.len()).rev() {
            if line.chars().nth(i).unwrap().is_numeric() {
                number = number + line.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                break;
            }

            if i as i32 - 3 <= 0 {
                let substring = &line[0..i + 1];
                for (key, value) in &map {
                    if substring.contains(key) {
                        number = number + value;
                        break;
                    }
                }
                if number % 10 != 0 {
                    break;
                }
                for c in substring.chars().rev() {
                    if c.is_numeric() {
                        number = number + c.to_digit(10).unwrap() as i32;
                        break;
                    }
                }
                break;
            }

            let substring = &line[i - 4..i + 1];
            let mut number_found = false;
            let mut number_found_value = 0;
            let mut number_found_key = 0;
            let mut char_found = false;
            let mut char_found_value = 0;
            let mut char_found_key = 0;
            for (key, value) in &map {
                if substring.contains(key) {
                    number_found = true;
                    number_found_value = value.clone();
                    number_found_key = substring.find(key).unwrap();
                    break;
                }
            }

            for c in substring.chars().rev() {
                if c.is_numeric() {
                    char_found = true;
                    char_found_value = c.to_digit(10).unwrap() as i32;
                    char_found_key = substring.find(c).unwrap();
                    break;
                }
            }
            if number_found && char_found {
                if number_found_key > char_found_key {
                    number = number + number_found_value;
                    break;
                } else {
                    number = number + char_found_value;
                    break;
                }
            } else if number_found {
                number = number + number_found_value;
                break;
            } else if char_found {
                number = number + char_found_value;
                break;
            }
        }
        // println!("Line: {}, result: {}", debug_int, number);
        count += number;
    }

    println!("{}", count);
}
