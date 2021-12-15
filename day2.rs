use std::io::{BufReader, BufRead};
use std::fs::File;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub fn day_2_2018(filename: &str) -> Result<()>{

    println!("Checksum: {:?}", day_2_1_2018(filename));
    println!("Common Letters: {:?}", day_2_2_2018(filename));

    Ok(())

}


fn day_2_1_2018(filename: &str) -> Result<i32>{

    let input_file = File::open(&filename)?;

    let input = BufReader::new(input_file);

    let mut two_letter = 0;

    let mut three_letter = 0;

    for line in input.lines() {

        let mut letter_frequency = HashMap::new();

        let code: Vec<char> = line.unwrap().chars().collect();       

        for letter in code{

            let count = letter_frequency.entry(letter).or_insert(0);
            *count += 1;

        }

        let mut found_two = false;
        let mut found_three = false;

        for (_, count) in letter_frequency.iter(){

            if *count == 2 && !found_two{
                two_letter += 1;
                found_two = true;
            }

            if *count == 3 && !found_three{
                three_letter += 1;
                found_three = true;
            }

        }

        
    }


    Ok(two_letter*three_letter)

}

fn day_2_2_2018(filename: &str) -> Result<String> {

    let input_file = File::open(&filename)?;

    let input = BufReader::new(input_file);

    let mut box_ids = Vec::new();

    for line in input.lines(){

        box_ids.push(line.unwrap());

    }

    for a in 0..box_ids.len(){
        for b in (a+1)..box_ids.len(){
            let (difference, common) = check_difference(&box_ids[a], &box_ids[b]);
            if difference == 1 {
                return Ok(common);
            }
        }
    }


    return Err(anyhow!("Nothing Found"));

}

fn check_difference(box1: &String, box2: &String) -> (i32, String) {

    let id1: Vec<char> = box1.chars().collect();
    let id2: Vec<char> = box2.chars().collect();

    let mut common = Vec::new();

    let mut similarities = 0;

    for it in id1.iter().zip(id2.iter()){

        let (ai, bi) = it;

        if *ai == *bi {
            common.push(*ai);
            similarities  = similarities  + 1;
        }

    }
   
    (id1.len() as i32 - similarities, common.into_iter().collect()) 


}