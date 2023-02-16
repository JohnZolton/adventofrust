use std::fs::read_to_string;

fn main() {
    let path = "input.txt";
    let binding = read_to_string(path).unwrap();
    let data : Vec<&str>= binding.split("\n").collect();
    let mut ans : u32 = 0;

    for line in data {
        let current: Vec<&str> = line.split(",").collect();
        let [a, b] = [current[0], current[1]];
        let c: Vec<&str>= a.split("-").collect();
        println!("{:?}", current);   
        let d: Vec<&str>= b.split("-").collect();
        let [first, second] = [c[0].parse::<i32>().unwrap(), c[1].parse::<i32>().unwrap()];
        let [third, fourth] = [d[0].parse::<i32>().unwrap(), d[1].parse::<i32>().unwrap()];
        if third <= first && second <= fourth {
            ans += 1;
        }
        else if first <= third && second >= fourth {
            ans += 1
        }
    }
    println!("{}", ans)
}
