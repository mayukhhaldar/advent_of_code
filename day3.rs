use anyhow::Result;
use std::io::{BufReader, BufRead};
use std::fs::File;


pub fn day_3_2018(filename: &str) -> Result<()> {

    let _variable = day_3_1_2018(filename)?;

    Ok(())
}

struct Fabric {
    grid: Vec<Vec<String>>,
    overlap: i32,
}

impl Fabric {

    fn fabric_intialization(x: usize, y: usize) -> Self {

        let mut grid: Vec<Vec<String>> = Vec::new();
    
        for i in 0..x {
    
            grid.push(Vec::new());
    
            for _j in 0..y {
                
                grid[i].push(String::from('.'));
    
            }
        }
    
        let overlap = 0;
        
        Fabric{grid, overlap}
    
    }

}

struct Claim {
    id: String,
    position: (i32, i32),
    size: (i32, i32),
}

impl Claim {
    fn make_claim(info: String) -> Result<Claim> {

        let mut id: String = String::from("");
        let mut position: (i32, i32) = (-1, -1);
        let mut size = (-1, -1);


        for token in info.split_whitespace() {

            let mut token_vec: Vec<char> = token.chars().collect();

            if token_vec.contains(&'#'){
                token_vec.remove(0);

                id = token_vec.into_iter().collect();

            } else if token_vec.contains(&':') {

                let mut position_parse = token.chars();
                position_parse.next_back();
                let extract = position_parse.as_str();
                let positions: Vec<&str> = extract.split(',').collect();
                let x_pos = positions[0].parse::<i32>()?;
                let y_pos = positions[1].parse::<i32>()?;
                position = (x_pos, y_pos);
                

            } else if token_vec.contains(&'x'){

                let sizes: Vec<&str> = token.split('x').collect();
                let x_size = sizes[0].parse::<i32>()?;
                let y_size = sizes[1].parse::<i32>()?;
                size = (x_size, y_size);

            }
            

        }

        Ok(Claim{id, position, size})

    }
}


fn day_3_1_2018(filename: &str) -> Result<()> {

    let input_file = File::open(&filename)?;

    let input = BufReader::new(input_file);

    let mut fabric = Fabric::fabric_intialization(1000, 1000);

    let mut safe_claims = Vec::new();
    let mut claims = Vec::new();

    for line in input.lines() {

        let mut claim = Claim::make_claim(line.unwrap())?;

        process_claim(&mut fabric, &mut claim);

        claims.push(claim);

    }

    for i in claims {

        if good_claim(&fabric.grid, &i){
            safe_claims.push(i.id);
        }

    }

    

    let overlaps = fabric.overlap;

    //print_fabric(fabric);

    println!("Overlaps: {}", overlaps);

    println!("Safe Claims: {:?}", safe_claims);

    Ok(())
}


#[allow(dead_code)]
fn print_fabric(full_info: Fabric){

    let fabric = full_info.grid;

    for i in 0..fabric.len() {
        for j in 0..fabric[i].len(){

            print!("{}", fabric[i][j])

        }

        print!("\n");
    }


    
}

fn process_claim(fabric: &mut Fabric, claim: &mut Claim) {


    for x in claim.position.0..(claim.position.0 + claim.size.0) {

        for y in claim.position.1..(claim.position.1 + claim.size.1) {

            if fabric.grid[y as usize][x as usize] == "." {
                fabric.grid[y as usize][x as usize] = claim.id.clone();
            } else if fabric.grid[y as usize][x as usize] == "X" {
                continue;
            } else {
                fabric.grid[y as usize][x as usize] = String::from("X");
                fabric.overlap = fabric.overlap + 1;
            }

        }

    }

}

fn good_claim(grid: &Vec<Vec<String>>, claim: &Claim) -> bool {

    let mut safe = true;

    for x in claim.position.0..(claim.position.0 + claim.size.0) {

        for y in claim.position.1..(claim.position.1 + claim.size.1) {

            if grid[y as usize][x as usize] != claim.id {
                safe = false;
                break;
            }

        }

    }

    safe

}


