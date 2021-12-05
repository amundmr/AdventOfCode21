use std::fs;

fn read_data(filename: &str) ->  Vec<Vec<usize>> {
    
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let mut lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    // Step 1: Read in the numbers
    let mut data: Vec<Vec<usize>> = vec![];
    //Format: [ [x1, y1, x2, y2],
    //          [x1, y1, x2, y2],... ]
    
    for line in lines {
        let endpoints: Vec<_> = line.split(" -> ").collect();
        let mut start: Vec<_> = endpoints[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut end: Vec<usize> = endpoints[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut row: Vec<usize> = vec![];
        row.append(&mut start);
        row.append(&mut end);
        
        data.push(row);
        
    }
    //println!("{:?}", data);

    return data;
}

pub fn day5() {
    let filename: &str = "inputs/5.txt";
    let data: Vec<Vec<usize>> = read_data(filename);
    let filtered_data: Vec<Vec<usize>> = filter_horizontal_and_vertical(&data);
    let intermediaries: Vec<Vec<(usize, usize)>> = find_intermediaries(&filtered_data);

    // Spawn matrix to do all the line addition in
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    // for each point, add it to the matrix
    for line in intermediaries {
        for (x,y) in line {
            matrix[x][y] += 1;
        }
    }
    //println!("{:?}", matrix);

}

fn find_intermediaries(data: &Vec<Vec<usize>>) -> Vec<Vec<(usize, usize)>> {
    let mut intermediaries: Vec<Vec<(usize, usize)>> = vec![vec![]];

    for line in data.iter() {
        let mut extended_line: Vec<(usize, usize)> = vec![];
        extended_line.push((line[0], line[1])); // Adding the first point
        extended_line.push((line[2], line[3])); // Adding the last point
        
        let xdist: isize = line[2] as isize - line[0] as isize;
        let ydist: isize = line[3] as isize - line[1] as isize;

        // Only works for horizontal or vertical lines
        if xdist > 1 {
            for i in 1..xdist.abs() {
                extended_line.push((line[0] + i as usize, line[1]));
            }
        } else if xdist < -1 {
            for i in 1..xdist.abs() {
                extended_line.push((line[0] - i as usize, line[1]));
            }

        } else if ydist > 1 {
            for i in 1..ydist.abs() {
                extended_line.push((line[0], line[1] + i as usize));
            }
        } else if ydist < -1 {
            for i in 1..ydist.abs() {
                extended_line.push((line[0], line[1] - i as usize));
            }
        }

        intermediaries.push(extended_line);
        

    }
    return intermediaries;
}

fn filter_horizontal_and_vertical(data: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut filtered: Vec<Vec<usize>> = vec![];

    for line in data.iter() {
        if line[0] == line[2] {
            filtered.push(line.clone());
        } else if line[1] == line[3] {
            filtered.push(line.clone())
        }
    }

    return filtered;
}