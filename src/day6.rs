use std::fs;

pub fn day6() {
    let filename: &str = "inputs/6.txt";
    let fish_ages: Vec<_> = read_data(filename);

    //Only need to save how many fish is in what age
    let mut num_ages: [usize;9] = [0; 9];
    //Count input
    for fish_age in fish_ages {
        num_ages[fish_age] += 1;
    }
    println!("{:?}", num_ages);

    let num_fish: usize = run_better_sim(&mut num_ages);
    println!("Total number of fish after 256 days: {}", num_fish);
}

fn run_better_sim(num_ages: &mut [usize]) -> usize {
    for day in 0..256 {
  
        // Shift all ages one closer to new birth
        num_ages.rotate_left(1);
        num_ages[6] += num_ages[8];

        println!("Just finished day {}", day);
    }

    // Sum all amount of fish
    num_ages.iter().sum()
}

fn run_simple_sim(mut fish_ages: Vec<usize>) -> usize {
    for _day in 0..256 {
        let mut num_newborn: usize = 0;
        for fish in 0..fish_ages.len() {
            if fish_ages[fish] > 0 {
                fish_ages[fish] -= 1;
            } else {
                fish_ages[fish] = 6;
                num_newborn += 1;
            }
        }
        if num_newborn > 0 {
            for _newborn in 0..num_newborn {
                fish_ages.push(8 as usize);
            }
        }
    }
    println!("Fishcount after 80 days: {:?}", fish_ages.len());
    println!("Fishcount after 256 days: {:?}", fish_ages.len());
    return fish_ages.len();
}

fn read_data(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let lines: Vec<_> = contents.lines().collect();

    //println!("{:?}", lines);

    let fish_ages: Vec<_> = lines[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    return fish_ages;
}
