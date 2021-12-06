use std::fs;

pub fn solution() {
    println!("this is day 2 solution!!!");

    let mut content = fs::read_to_string("day_2/input_day2.txt").expect("some error happened");

    let mut tokens: Vec<&str> = content.split('\n').collect();

    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    for x in tokens.iter() {
        let temp: Vec<&str> = x.split(' ').collect();
        match temp[0] {
            "forward" => position += temp[1].parse::<i32>().unwrap(),
            "up" => depth -= temp[1].parse::<i32>().unwrap(),
            "down" => depth += temp[1].parse::<i32>().unwrap(),
            _ => panic!(),
        };
    }

    println!("{} {}", position, depth);

    let mut ans: i32 = position * depth;
    println!("{:?}", ans);
}
