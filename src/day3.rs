use std::fs;


pub fn main() {

    let data = fs::read_to_string("input/day3.txt").expect("Unable to read file");
    
    for line in data.lines(){
        println!("{}", line);
    }


}
