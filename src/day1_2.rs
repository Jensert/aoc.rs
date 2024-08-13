use std::fs;

pub fn main() {
    let file = "puzzles/2023/1.1";
    
    let pattern = get_pattern();
    let pattern = replace_digits(pattern);
    
    let vals = get_values(pattern);

    sum_values(vals);
}

pub fn read_file(file: &str) -> String {
    // --snip--
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    return contents;
}

pub fn replace_digits(pattern: String) -> String {
    let p = pattern;
    let p = p.replace("one","o1e");
    let p = p.replace("two","t2o");
    let p = p.replace("three","t3e");
    let p = p.replace("four","f4r");
    let p = p.replace("five","f5v");
    let p = p.replace("six","s6x");
    let p = p.replace("seven","s7n");
    let p = p.replace("eight","e8t");
    let p = p.replace("nine","n9e");
    p
}

pub fn get_values(pattern: String) -> Vec<i32> {
    //let mut all_vals = Vec::new();
    let mut line_val = Vec::new();
    let mut val = String::from("");
    let mut vals = Vec::new();
    for char in pattern.bytes() {
        if char >= 48 && char <= 57 {
            line_val.push(char);
        }
        if char == 10 {
            val.push(line_val[0] as char);
            val.push(line_val[line_val.len()-1] as char);
            let val_int: i32 = val.parse().unwrap();
            vals.push(val_int);
            val.clear();
            line_val.clear()
        }
    }
    vals
}

pub fn sum_values(vals: Vec<i32>) {
    let sum: i32 = vals.iter().sum();
    println!("{}", sum);
}
