use std::fs;




pub fn day6() {
    let filename: &str = "inputs/6.txt";
    let mut fish_ages: Vec<_> = read_data(filename);
    println!("{:?}", fish_ages);
}

fn read_data(filename: &str) -> Vec<usize> {
    
    let contents = fs::read_to_string(filename).expect("Something went wrong while reading file");

    let mut lines: Vec<_> = contents.lines().collect();
    let len: usize = lines.len();

    //println!("{:?}", lines);

    let fish_ages: Vec<_> = lines[0].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

 
    return fish_ages;

}