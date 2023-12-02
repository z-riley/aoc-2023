use std::fs;

enum Perspective {
    First,
    Last,
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

pub fn main() {

    let data = fs::read_to_string("input/day1.txt").expect("Unable to read file");
    
    let mut calibration_values_sum = 0;

    for line in data.lines(){
       
        let calibration_str = format!("{}{}", &get_numeric_char(line, Perspective::First), &get_numeric_char(line, Perspective::Last));

        let calibration_value: i32 = calibration_str.parse().unwrap();
        calibration_values_sum += calibration_value;
    }

    print!("Sum of all of the calibration values: {}", calibration_values_sum);

}
