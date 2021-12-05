use std::fs;

fn read_data(filename: &str) ->  Vec<Vec<usize>> {
    
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let mut lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    //let random_numbers: Vec<_> = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();


    // Step 1: Read in the numbers
    let mut data: Vec<Vec<usize>> = vec![];
    //Format: [ [x1, y1, x2, y2],
    //          [x1, y1, x2, y2],... ]
    
    for line in lines {
        let endpoints: Vec<_> = line.split("->").collect();
        println!("{:?}", endpoints);
        let mut start: Vec<usize> = endpoints[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut end: Vec<usize> = endpoints[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        println!("{:?}", start);
        println!("{:?}", end);

        let mut row: Vec<usize> = vec![];
        row.append(&mut start);
        row.append(&mut end);
        
        data.push(row);
        
    }
    println!("{:?}", data);

    return data;
}

pub fn day5() {
    let filename: &str = "inputs/5.txt";
    let data: Vec<Vec<usize>> = read_data(filename);
}