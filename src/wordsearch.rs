use std::error::Error;
use std::fs::read_to_string;

pub fn find_all_in_file(path: &str) -> Result<(), Box<dyn Error>> {
    let data = read_to_string(path)?;
    // todo - map 'Q' to 'Qu'
    let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    println!("todo: get all the words from {:?}", data);
    Ok(())
}
