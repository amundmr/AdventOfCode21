use std::fs;

pub fn day4() {
    println!("----------------------");
    println!("Day 4 part 1 solution:");
    println!("----------------------");

    let filename = "inputs/4.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let mut lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    //println!("{:?}", lines);

    let random_numbers: Vec<_> = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    lines.remove(0); //Delete the random numbers so it wont interfere with the table generation
    lines.remove(0); //Delete the empty line


    println!("The random numbers are: {:?}", random_numbers);

    // Read in the matrices
    let mut matrices: Vec<Vec<Vec<usize>>> = vec![];
    //Format: [ [   [[],[],[]], 
    //              [[],[],[]], 
    //              [[],[],[]] ], ... ]
    let mut curr_table: Vec<Vec<usize>> = vec![];
    for line in lines {
        if line.len() < 4 {
            matrices.push(curr_table);
            curr_table = vec![];
            continue;
        } else {
            let row: Vec<usize> = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
            curr_table.push(row);
        }
    }
    //println!("{:?}", matrices);
    
    

}