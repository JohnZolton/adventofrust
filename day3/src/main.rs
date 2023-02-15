use std::{fs};

fn main() {
    let path = "input.txt";
    let data = fs::read_to_string(path).unwrap();
    let mut ans : i32 = 0;
    let lines = data.split("\n");
    let mut linesvec: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let linevec: Vec<char> = line.chars().collect();
        linesvec.push(linevec);
    } 
    for i in 0..linesvec.len() {
        if i % 3 == 0 {
            for item in &linesvec[i] {
                if (linesvec[i+1].contains(&item)) && ( linesvec[i+2].contains(&item)){
                    println!("{}", item);
                    if item.is_uppercase() {
                        let add = *item as i32 - 64 + 26;
                        println!("{} = {}", item, add);
                        ans += add; 
                    } else {
                        let add = *item as i32 - 96;
                        println!("{} = {}", item, add);
                        ans += add;
                    }
                    break
                }
            }
        }
    }
    println!("{}", ans)
}

fn _firstans() {
    let path = "input.txt";
    let data = fs::read_to_string(path).unwrap();
    let mut ans : i32 = 0;
    let lines = data.split("\n");
    for line in lines {
        let linevec :Vec<char> = line.chars().collect();
        println!("{:?}", linevec);
        let midpoint = linevec.len()/2;
        let firsthalf = linevec[0..midpoint].to_vec();
        let secondhalf =  linevec[midpoint..].to_vec();

        for item in firsthalf {
            if secondhalf.contains(&item) {
                if item.is_uppercase() {
                    let add = item as i32 - 64 + 26;
                    println!("{} = {}", item, add);
                    ans += add; 
                } else {
                    let add = item as i32 - 96;
                    println!("{} = {}", item, add);
                    ans += add;
                }
                break
            }
        }
        }
        println!("{}", ans)
}