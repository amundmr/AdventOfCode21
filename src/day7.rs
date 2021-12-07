use std::fs;

pub fn day7() {
    let filename: &str = "inputs/7.txt";
    let mut crab_pos: Vec<usize> = read_data(filename);

    //println!("{:?}", crab_pos);

    crab_pos.sort();

    let median: usize = crab_pos[crab_pos.len() / 2 as usize];
    println!("The median is: {}", median);

    let fuel: usize = calculate_fuel(&crab_pos, median);
    println!("The total fuel consumption is: {}", fuel);

    let mut fuels: Vec<usize> = vec![];
    let mut points: Vec<usize> = vec![];
    for point in *crab_pos.iter().min().unwrap()..*crab_pos.iter().max().unwrap() {
        fuels.push(calculate_expensive_fuel(&crab_pos, point));
        points.push(point);
    }

    let min: usize = *fuels.iter().min().unwrap();
    println!("The minimum amount of fuel they can use is: {}", min);

}

fn calculate_expensive_fuel(crab_pos: &Vec<usize>, point: usize) -> usize {
    let mut total_cost: usize = 0;
    for pos in crab_pos.iter() {
        let dist: usize = (*pos as isize - point as isize).abs() as usize;
        total_cost += dist * (dist + 1) / 2;
    }
    total_cost
}


fn calculate_fuel(crab_pos: &Vec<usize>, median: usize) -> usize {
    let mut fuel: usize = 0;

    for crab in crab_pos {
        fuel += (*crab as isize - median as isize).abs() as usize;
    }

    fuel
}

fn read_data(filename: &str) -> Vec<usize> {
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let lines: Vec<_> = contents.lines().collect();

    //println!("{:?}", lines);

    let crab_pos: Vec<usize> = lines[0]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    return crab_pos;
}
