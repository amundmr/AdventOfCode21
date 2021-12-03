
use std::fs;

pub fn day3() {
    //day3pt1();
    day3pt2();
}
fn day3pt1() {
    println!("Day 3 part 1 solution:");

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
    println!("{:?}", commonbit);

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

fn day3pt2(){
    println!("Day 3 part 2 solution:");

    let filename = "inputs/3.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    // Lets put all these numbers in a nested vector
    let mut all_binarys: Vec<Vec<u32>> = vec![ vec![0; 12] ; len];

    for (i, line) in lines.iter().enumerate() {
        all_binarys[i] = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    }

    // Looping over all the digits
    let mut oxygen_binarys: Vec<Vec<u32>> = all_binarys.clone();
    let mut co2_binarys: Vec<Vec<u32>> = all_binarys;

    let mut more: u32 = 0;
    let mut less: u32 = 0;

    for digit in 0..11{

        if oxygen_binarys.len() > 1 {
            // Lets first do the oxygen stuff
            let mut sum: u32 = 0;

            for binary in oxygen_binarys.iter() {
                sum += binary[digit];
            }

            if sum*2 < len as u32 {
                // the dominating digit is 0
                more = 0;
            } else {
                // there are more 1 than 0 or they are equal
                more = 1;
            }
            let mut new_oxygen_binarys: Vec<Vec<u32>> = vec![ vec![0; 12] ; 0];
            for binary in oxygen_binarys.iter() {
                if binary[digit] == more {
                    new_oxygen_binarys.push(binary.to_vec());
                }
            }
            oxygen_binarys = new_oxygen_binarys;
        }

        if co2_binarys.len() > 1 {
            // Now the CO2
            let mut sum: u32 = 0;

            for binary in co2_binarys.iter() {
                sum += binary[digit];
            }

            if sum*2 < len as u32 {
                // the dominating digit is 0, undominating =1 
                less = 1;
            } else {
                // there are more 1 than 0, or they are equal, thus the undominating one is 0
                less = 0;
            }
            let mut new_co2_binarys: Vec<Vec<u32>> = vec![ vec![0; 12] ; 0];
            for binary in co2_binarys.iter() {
                if binary[digit] == less {
                    new_co2_binarys.push(binary.to_vec());
                }
            }
            co2_binarys = new_co2_binarys;
        }
    }
    let oxygen_binary: Vec<u32> = oxygen_binarys[0].clone();
    let co2_binary: Vec<u32> = co2_binarys[0].clone();
    let o2_str: String = oxygen_binary.into_iter().map(|i| i.to_string()).collect::<String>();
    let co2_str: String = co2_binary.into_iter().map(|i| i.to_string()).collect::<String>();
    println!("Oxygen binary: {}",o2_str);
    println!("CO2 binary: {}", co2_str);

    let o2_int = isize::from_str_radix(&o2_str, 2).unwrap();
    let co2_int = isize::from_str_radix(&co2_str, 2).unwrap();
    println!("Oxygen integer: {}",o2_int);
    println!("CO2 integer: {}", co2_int);

    let life_support_rating: isize = o2_int * co2_int;
    println!("Life support rating found to be: {}", life_support_rating);

}
