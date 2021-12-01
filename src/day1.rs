use std::fs;

pub fn day1() {
    println!("Day 1 solution:");
    let filename = "src/1input.txt";
    println!("The input filename chosen is '{}'", filename);

    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file");
    //println!("{}", contents);
    let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);

    // Part 1: Looping over all values to find out how much its increased
    let mut counter: i32 = 0;

    for n in 1..splitlines.len() {
        let prev: i32 = splitlines[n-1].parse::<i32>().unwrap();
        let curr: i32 = splitlines[n].parse::<i32>().unwrap();
        if prev < curr {
            counter += 1
        }
        //println!("This is linenumber {}, with value: {}", n, splitlines[n].parse::<i32>().unwrap())
    }
    println!("The total number of increases is: {}", counter);

    // Part 2: using the sum of the 3 closest numbers
    let mut sums: Vec<i32> = vec![0; splitlines.len()-2];

    for n in 1..(splitlines.len() - 1) {
        let prev: i32 = splitlines[n-1].parse::<i32>().unwrap();
        let curr: i32 = splitlines[n].parse::<i32>().unwrap();
        let next: i32 = splitlines[n+1].parse::<i32>().unwrap();
        let sum = prev + curr + next;
        sums[n-1] = sum;
    }
    let mut sumscounter: i32 = 0;
    for n in 1..sums.len() {
        let prev: i32 = sums[n-1];
        let curr: i32 = sums[n];
        if prev < curr {
            sumscounter += 1;
        }
    }
    println!("The total number of increases in a 3-sum is: {}", sumscounter)
}