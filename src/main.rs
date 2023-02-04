use std::fs;

fn main() {
    let data = fs::read_to_string("test2.txt").expect("could not open file");

    let lines = data.split("\r\n");
    let mut cal_vec: Vec<i32> = Vec::new();
    let mut cumsum = 0;
    for line in lines {
        if line != "" {
            cumsum += line.parse::<i32>().unwrap_or(0);
        } else {
            cal_vec.push(cumsum);
            cumsum = 0;
        }
    }
    cal_vec.sort();
    println!("{}", cal_vec.iter().max().unwrap());
    let mut ans = 0;
    for n in 1..4{
        ans += cal_vec.pop().unwrap();
    }
    println!("{ans}");
}