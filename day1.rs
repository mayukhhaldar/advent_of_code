use std::io::{BufReader, BufRead};
use std::fs::File;
use anyhow::Result;
use std::collections::HashSet;

pub fn day_1_2018(filename: &str) -> Result<()>{

    println!("Frequency: {:?}", day_1_1_2018(filename));
    println!("First Repetition: {:?}", day_1_2_2018(filename));

    Ok(())

}


fn day_1_1_2018(filename: &str) -> Result<i32> {

    let input_file = File::open(&filename)?;

    let input = BufReader::new(input_file);

    let mut frequency = 0;

    for line in input.lines(){

        let f_change = line.unwrap().parse::<i32>()?;
                
        frequency = frequency + f_change;

    }

    Ok(frequency)

}

fn day_1_2_2018(filename: &str) -> Result<i32> {

    let input_file = File::open(&filename)?;

    let input = BufReader::new(input_file);

    let mut num_keeper = HashSet::new();

    let mut num_vector = Vec::new();

    let mut frequency = 0;

    num_keeper.insert(frequency);

    let mut first_rep: i32 = 0;
    let mut found = false;

    for line in input.lines(){

        let f_change = line.unwrap().parse::<i32>()?;

        num_vector.push(f_change);

        frequency = frequency + f_change;

        if !num_keeper.insert(frequency){
            
            first_rep = frequency;
            found = true;
            break;
            
        }
    
    }

    if !found {

        'find: loop {

            for value in &num_vector {
    
                frequency = frequency + value;
        
                if !num_keeper.insert(frequency){
                    
                    first_rep = frequency;
                    break 'find;
                    
                }
            }
        }

    }
    

    Ok(first_rep)

}