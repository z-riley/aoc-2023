use std::fs;

enum Perspective {
    First,
    Last,
}


// NOTE: Doesn't work because words2digits not replacing the numeric words in the order they appear. See unit test.
// See https://github.com/coriolinus/adventofcode-2023/blob/91d30b209a3dde475e964fb2222667f26dd1d85f/day01/src/lib.rs#L26-L51

pub fn main() {

    let data = fs::read_to_string("input/day1.txt").expect("Unable to read file");
    
    let mut calibration_values_sum = 0;

    for line in data.lines(){
       
        let line1 = words2digits(line);
        let calibration_str = format!("{}{}", &get_numeric_char(&line1, Perspective::First), &get_numeric_char(&line1, Perspective::Last));

        let calibration_value: i32 = calibration_str.parse().unwrap();
        calibration_values_sum += calibration_value;
    }

    print!("Sum of all of the calibration values: {}", calibration_values_sum);

}

fn get_numeric_char(input: &str, perspective: Perspective) -> String {

    match perspective {
        Perspective::First => {
            for char in input.chars(){
                if char.is_digit(10){
                    return char.to_string();
                }
            }
            return ' '.to_string();
        }
        Perspective::Last => {
            for char in input.chars().rev(){
                if char.is_digit(10){
                    return char.to_string();
                }
            }
            return ' '.to_string();
        }
    };

}

fn words2digits(input: &str) -> String {
    let replacements = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];


    let mut result = input.to_string();

    for &(word, digit) in &replacements {
        result = result.replace(word, digit);
    }

    result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words2digits() {
        assert_eq!(words2digits("two1nine"), "219");
        assert_eq!(words2digits("eightwothree"), "823");
        assert_eq!(words2digits("abcone2threexyz"), "abc123xyz");
        assert_eq!(words2digits("xtwone3four"), "2134");
        assert_eq!(words2digits("4nineeightseven2"), "49872");
        assert_eq!(words2digits("zoneight234"), "18234");
        assert_eq!(words2digits("7pqrstsixteen"), "7pqrst6teen");
    }

}

