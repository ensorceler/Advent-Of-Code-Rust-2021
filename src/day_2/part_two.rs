use std::fs;

pub fn solution() {
    println!("this is second part of this solution");

    println!("this is day 2 solution!!!");
    let mut content = fs::read_to_string("day_2/input_day2.txt").expect("some error happened");
    let mut tokens: Vec<&str> = content.split('\n').collect();

    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    let mut horizontal_dist = 0;
    let mut depth = 0;
    let mut aim = 0;
    for x in tokens.iter() {
        let temp: Vec<&str> = x.split(' ').collect();
        match temp[0] {
            "forward" => {
                horizontal_dist += temp[1].parse::<i32>().unwrap();
                depth += temp[1].parse::<i32>().unwrap() * aim;
            }
            "down" => {
                aim += temp[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= temp[1].parse::<i32>().unwrap();
            }
            _ => panic!(),
        }
    }

    println!("{} {}", horizontal_dist, depth);
    println!("{}", horizontal_dist * depth);
}
