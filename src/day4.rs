use std::fs;

fn read_data(filename: &str) -> (Vec<usize>, Vec<Vec<Vec<usize>>>){
    
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
    let mut row: Vec<usize> = vec![];
    for line in lines {
        if line.len() < 4 {
            matrices.push(curr_table);
            curr_table = vec![];
            continue;
        } else {
            row = line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
            curr_table.push(row);
        }
    }
    //println!("{:?}", matrices);

    return (random_numbers, matrices);

}

pub fn day4() {
    println!("---------------");
    println!("Day 4 solution:");
    println!("---------------");

    let filename = "inputs/4.txt";
    let result = read_data(filename);
    let random_numbers: Vec<usize> = result.0;
    let mut matrices: Vec<Vec<Vec<usize>>> = result.1;

    let start_matrices: usize = matrices.len();
    

    // Step 2: Find the winning board
    'outer: for num in random_numbers {

        // Vector to collect matrix indexes to remove after bingoed
        let mut remove_matrices: Vec<usize> = vec![];

        for mat_index in 0..matrices.len() {
            for row_index in 0..matrices[mat_index].len() {
                for val_index in 0..matrices[mat_index][row_index].len() {

                    if num == matrices[mat_index][row_index][val_index] {
                        matrices[mat_index][row_index][val_index] += 100; // Move value of element outside the range of 0-100 which all normal numbers are.
                    }
                }
            }
            let mut bingo: bool;
            // Now the matrix is marked with the new tested numbers so we can check if any row or column is totally marked.
            bingo = is_this_bingo(&matrices[mat_index]);


            // Now scan the matrix columns, only bother to do it if it isnt already bingoed
            if bingo == false {
                let transposed: Vec<Vec<usize>> = transpose(matrices[mat_index].clone());
                bingo = is_this_bingo(&transposed);
            }
            

            if bingo {
                remove_matrices.push(mat_index);
                //calc_score(matrices[mat_index].clone(), num);
            }

        }
        if remove_matrices.len() > 0 {
            for (i, index) in remove_matrices.iter().enumerate() {
                //If this is the first matrix to bingo
                if matrices.len() == start_matrices {
                    println!("----------------------");
                    println!("FIRST MATRIX TO BINGO:");
                    calc_score(matrices[*index].clone(), num);
                }
                //If there is only one matrix left, this is the last one without bingo
                if matrices.len() == 1 {
                    println!("---------------------");
                    println!("LAST MATRIX TO BINGO:");
                    calc_score(matrices[0].clone(), num);
                    break 'outer;
                }
                matrices.remove(index - i);
            }
        }
    }

}

fn is_this_bingo(v: &Vec<Vec<usize>>) -> bool {
    let mut bingo: bool = false;
    'outer: for row_index in 0..v.len() {
        let mut count: usize = 0;
        for val_index in 0..v[row_index].len() {
            if v[row_index][val_index] > 99 {
                count += 1;
            }
        }
        if count == 5 {
            bingo = true;
            break 'outer;
        }
    }
    return bingo;
}

fn calc_score(v: Vec<Vec<usize>>, num: usize){
    println!("Random number: {:?}, matrix: {:?}",num, v );
    let sum:usize = sum_of_unmarked(v.clone());
    println!("Sum of unmarked in matrix: {}", sum);
    println!("Sum times last random num: {}", sum*num);
}


fn sum_of_unmarked(v: Vec<Vec<usize>>) -> usize {
    let mut sum: usize = 0;
    for row_index in 0..v.len() {
        for val_index in 0..v[row_index].len() {
            if v[row_index][val_index] < 100 {
                sum += v[row_index][val_index];
            }
        }
    }
    return sum;
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}