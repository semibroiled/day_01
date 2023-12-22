use std::collections::HashMap;

fn main() {
    println!("Starting Part 1");

    // Run Test Case
    let input: &str = include_str!("./input_ex1.txt");
    let output: i32 = solution_pt1(input);
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);

    // Run actual Case
    let input: &str = include_str!("./input1.txt");
    let output: i32 = solution_pt1(input);
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);

    println!("Starting Part 2");
    // Run Test Case
    let input: &str = include_str!("./input_ex2.txt");
    let output: i32 = solution_pt2(input, &get_mapping());
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);

    // Run actual Case
    let input: &str = include_str!("./input1.txt");
    let output: i32 = solution_pt2(input, &get_mapping());
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);

    // Run actual Case ammended
    // Run Test Case
    let input: &str = include_str!("./input_ex2.txt");
    let output: i32 = solution_pt2_fixed(input, &get_mapping());
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);

    // Run actual
    let input: &str = include_str!("./input1.txt");
    let output: i32 = solution_pt2_fixed(input, &get_mapping());
    dbg!(output); // Debugs out Output
    println!("Output is {}", output);
}
enum NumberBase {
    DecimalFmt = 10,
}

fn solution_pt1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let first_num: Option<char> = line
            .chars()
            .find(|c| c.is_digit(NumberBase::DecimalFmt as u32));
        let last_num: Option<char> = line
            .chars()
            .rev()
            .find(|c| c.is_digit(NumberBase::DecimalFmt as u32));

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let number = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
            sum += number;
        }
    }
    return sum;
}

// Solution to part 2 invovled more steps

// First part, get numbers from word

fn get_mapping() -> HashMap<&'static str, char> {
    let mappings = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    // Iterate over list of tuples, clone to get data not just reference, then build to HashMap
    mappings.iter().cloned().collect()
}

fn solution_pt2(input: &str, digit_mapping: &HashMap<&str, char>) -> i32 {
    let mut sum: i32 = 0;

    let mut first_num: Option<char> = None;
    let mut last_num: Option<char> = None;
    let mut current_word: String = String::new();

    let mut lineno = 0;

    for line in input.lines() {
        lineno += 1;
        for c in line.chars() {
            if c.is_alphabetic() {
                current_word.push(c);
                for i in 0..current_word.len() {
                    let suffix = &current_word[i..]; //suffix of word amalgamation
                    if let Some(digit) = digit_mapping.get(suffix) {
                        last_num = Some(*digit);
                        if first_num.is_none() {
                            first_num = Some(*digit);
                        }
                        // dbg!(suffix);

                        current_word.clear();
                        break; // Break after first match
                    }
                }
            } else if c.is_digit(NumberBase::DecimalFmt as u32) {
                last_num = Some(c);
                if first_num.is_none() {
                    first_num = Some(c);
                }
                current_word.clear();
            }
        }

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let number = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
            println!("line {}: {}", lineno, number);
            sum += number;
            println!("{}", sum)
        }
        current_word.clear();
        first_num = None;
        last_num = None;
    }

    return sum;
}

fn solution_pt2_fixed(input: &str, digit_mapping: &HashMap<&str, char>) -> i32 {
    let mut sum: i32 = 0;
    let mut lineno = 0;
    for line in input.lines() {
        let mut first_num: Option<char> = None;
        let mut last_num: Option<char> = None;
        let mut current_word: String = String::new();
        lineno += 1;

        // Process first number
        for c in line.chars() {
            if c.is_digit(NumberBase::DecimalFmt as u32) {
                if first_num.is_none() {
                    first_num = Some(c);
                }
            } else if c.is_alphabetic() {
                current_word.push(c);
                if first_num.is_none() {
                    for i in 0..current_word.len() {
                        let suffix = &current_word[i..]; //suffix of word amalgamation
                        if let Some(&mapped_fdigit) = digit_mapping.get(suffix) {
                            first_num = Some(mapped_fdigit);
                        }
                    }
                }
            }
        }

        // Reset for Processing Last number
        current_word.clear();

        // Process Last Number, iterate in reverse order
        for c in line.chars().rev() {
            println!("line {}: char: {}", line, c);
            if c.is_digit(NumberBase::DecimalFmt as u32) {
                {
                    last_num = Some(c);
                    break;
                }
            } else if c.is_alphabetic() {
                current_word.insert(0, c); // Prepend to reverse
                for i in 0..current_word.len() {
                    let suffix = &current_word[..i + 1]; //suffix of word amalgamation
                    println!("{}", suffix);
                    if let Some(&mapped_ldigit) = digit_mapping.get(suffix) {
                        last_num = Some(mapped_ldigit);
                        dbg![last_num];
                        break;
                    }
                }
            }
            if last_num.is_some() {
                break;
            }
        }

        if let (Some(first), Some(last)) = (first_num, last_num) {
            let number = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
            println!("line {}: {}", lineno, number);
            sum += number;
            println!("{}", sum)
        }
    }
    sum
}

// Testing in rust
#[cfg(test)]
mod tests {
    // Uses Everything in Parent Caste i.e. Everything up there
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let operation = 69 + 42;
        let result = 111;
        assert_eq!(
            operation, result,
            "We are testing {} with {}",
            operation, result
        )
    }

    #[rstest]
    #[case("twolnine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7parstsixteen", 76)]
    #[case("rnbchhfk6884fivejtr5twonet", 61)]
    #[case("rkclvq4seven446dkjfpgtseightwobc", 42)]

    fn line_test(#[case] line: &str, #[case] expected: i32) {
        println!(
            "Expected was {}, got {}",
            expected,
            solution_pt2_fixed(line, &get_mapping())
        );
        assert_eq!(expected, solution_pt2_fixed(line, &get_mapping()));
    }

    // // From Test module - mod, with test macro - test, on function to be tested
    // #[test]
    // fn test_part1() {
    //     let result = part1("");
    //     assert_eq!(result, result)
    // }

    #[test]
    fn example_case_pt1() {
        let text: &str = include_str!("./input_ex1.txt");
        let my_answer = solution_pt1(text);
        let answer: i32 = 142;

        assert_eq!(answer, my_answer)
    }

    #[test]
    fn example_case_pt2() {
        let text: &str = include_str!("./input_ex2.txt");
        let my_answer = solution_pt2(text, &get_mapping());
        let answer: i32 = 281;

        assert_eq!(answer, my_answer)
    }

    #[test]
    fn example_case_pt2_fixed() {
        let text: &str = include_str!("./input_ex2.txt");
        let my_answer = solution_pt2_fixed(text, &get_mapping());
        let answer: i32 = 281;

        assert_eq!(answer, my_answer)
    }
}
