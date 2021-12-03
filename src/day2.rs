use std::fs;

pub fn day2() {
    println!("Day 2 solution:");
    let filename = "inputs/2.txt";
    println!("The input filename chosen is '{}'", filename);

    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file");
    //println!("{}", contents);
    //let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);

    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for line in contents.lines() {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let distance = split[1].parse::<i32>().unwrap();
        //println!("Direction: {}, amount: {}", direction, distance);
        if direction == "down" {
            depth += distance;
        } else if direction == "up" {
            depth -= distance;
        } else if direction == "forward" {
            horiz += distance;
        }
    }
    println!("Final Depth: {}, Final Horizontal position: {}, Multiplied: {}", depth, horiz, depth*horiz);

    // Part two
    println!("Part two:");
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;
    let mut aim: i32 = 0;

    for line in contents.lines() {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let distance = split[1].parse::<i32>().unwrap();
        //println!("Direction: {}, amount: {}", direction, distance);
        if direction == "down" {
            aim += distance;
        } else if direction == "up" {
            aim -= distance;
        } else if direction == "forward" {
            horiz += distance;
            depth += aim*distance;
        }
    }
    println!("Final Depth: {}, Final Horizontal position: {}, Multiplied: {}", depth, horiz, depth*horiz);
}