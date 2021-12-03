use std::fs;

pub fn day3() {
    day3pt1();
    day3pt2();
}

fn day3pt1() {
    println!("----------------------");
    println!("Day 3 part 1 solution:");
    println!("----------------------");

    let filename = "inputs/3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    let mut commonbit: Vec<u32> = vec![0; 12];
    for line in lines {
        // Get the chars out of the string, and map it all to digits with base10, before collecting in a vector
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        // Do vector addition with commonbit
        commonbit = commonbit.iter().zip(digits.iter()).map(|(x, y)| x + y).collect();
    }

    let mut bits_of_gamma: String = "".to_string();
    let mut bits_of_epsilon: String = "".to_string();

    // If the sum of bits are larger than half the length, the most common bit is 1, else its 0
    for n in commonbit {
        if n as f32 > len as f32 / 2. {
            bits_of_gamma.push('1');
            bits_of_epsilon.push('0');
        } else {
            bits_of_gamma.push('0');
            bits_of_epsilon.push('1');
        }
    }
    println!("The bits of gamma are: {}, the bits of epsilon are: {}.", bits_of_gamma, bits_of_epsilon);

    let int_of_gamma = isize::from_str_radix(&bits_of_gamma, 2).unwrap();
    let int_of_epsilon = isize::from_str_radix(&bits_of_epsilon, 2).unwrap();

    println!("The integer of gamma is: {}, the integer of epsilon is: {}.", int_of_gamma, int_of_epsilon);

    let result = int_of_gamma * int_of_epsilon;
    println!("The power consumption, gamma times epsilon, is: {}", result);
}


fn day3pt2() {
    println!("");
    println!("----------------------");
    println!("Day 3 part 2 solution:");
    println!("----------------------");

    let filename = "inputs/3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let lines: Vec<_> = contents.lines().collect();

    // OXY first
    let mut oxy_data: Vec<_> = lines.clone(); 
    let mut oxy_int: u32 = 0;

    for i in 0..12 {
        let mut bitsum: u32 = 0;
        let dominating: u32;
        for line in oxy_data.iter(){
            bitsum += line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }

        if 2*bitsum >= oxy_data.len() as u32 {
            // 1 is dominating, or 1 and 0 is even
            dominating = 1;
        } else {
            //0 is dominating
            dominating = 0;
        }

        let mut new_oxy_data: Vec<&str> = vec![];

        for line in oxy_data.iter() {
            if line.chars().nth(i).unwrap().to_digit(10).unwrap() == dominating {
                new_oxy_data.push(line);
            }
        }

        if new_oxy_data.len() == 1 {
            println!("The oxygen binary left is: {}", new_oxy_data[0]);
            oxy_int = u32::from_str_radix(new_oxy_data[0], 2).unwrap();
            println!("OXY INT: {}", oxy_int);
            break;
        }
        oxy_data = new_oxy_data;

    }


    // CO2
    let mut co2_data: Vec<_> = lines.clone(); 
    let mut co2_int: u32 = 0;

    for i in 0..12 {

        let mut bitsum: u32 = 0;
        let notdominating: u32;

        for line in co2_data.iter(){
            bitsum += line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }

        if 2*bitsum >= co2_data.len() as u32 {
            // 1 is dominating, or 1 and 0 is even
            notdominating = 0;
        } else {
            //0 is dominating
            notdominating = 1;
        }

        let mut new_co2_data: Vec<&str> = vec![];

        for line in co2_data.iter() {
            if line.chars().nth(i).unwrap().to_digit(10).unwrap() == notdominating {
                new_co2_data.push(line);
            }
        }

        if new_co2_data.len() == 1 {
            println!("The CO2 binary left is: {}", new_co2_data[0]);
            co2_int = u32::from_str_radix(new_co2_data[0], 2).unwrap();
            println!("CO2 INT: {}", co2_int);
            break;
        }
        co2_data = new_co2_data;

    }

    println!("The life support rating is: {}", co2_int*oxy_int);
}