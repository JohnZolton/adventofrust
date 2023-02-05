use std::{fs};
fn main() {
    let data = fs::read_to_string("test.txt").expect("could not open");
    let lines = data.split("\r\n");
    let mut score: u32 = 0;
    for line in lines {
        let  a = line.split(" ").collect::<Vec<&str>>();
        let b = createoutcome(&a);
        let c: Vec<&str> = vec![a[0], &b];
        score += findwinner(&c);
        score += getvalue(b);
    }
    println!("{}", score)
}


fn findwinner(a: &Vec<&str>) -> u32 {
    match [a[0], a[1]] {
        ["A", "X"] => 3,
        ["A", "Y"] => 6,
        ["A", "Z"] => 0,
        ["B", "X"] => 0,
        ["B", "Y"] => 3,
        ["B", "Z"] => 6,
        ["C", "X"] => 6,
        ["C", "Y"] => 0,
        ["C", "Z"] => 3,
        _ => 0
    }
}

fn createoutcome<'a>(a: &'a Vec<&'a str>) -> &'a str {
    match [a[0], a[1]] {
        ["A", "X"] => "Z",
        ["A", "Y"] => "X",
        ["A", "Z"] => "Y",
        ["B", "X"] => "X",
        ["B", "Y"] => "Y",
        ["B", "Z"] => "Z",
        ["C", "X"] => "Y",
        ["C", "Y"] => "Z",
        ["C", "Z"] => "X",
        _ => ""
    }
}
fn getvalue(ourhand:&str) -> u32 {
    match ourhand {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}