use std::fs;

pub fn main() {

    let data = fs::read_to_string("input/day0.txt").expect("Unable to read file");
    
    for line in data.lines(){
        println!("{}", line);
    }

}
