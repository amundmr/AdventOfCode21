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

    // Step 1: Read in the matrices
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
    
    // Step 2: Find the winning board
    for num in random_numbers {
        for matrix in matrices {
            for row in matrix {
                for val in row {
                    if num == val {
                        val + 100; // Move value of element outside the range of 0-100 which all normal numbers are.
                    }
                }
            }
            // Now the matrix is marked with the new tested numbers so we can check if any row or column is totally marked.
            
        }
    }

}