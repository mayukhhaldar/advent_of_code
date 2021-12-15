use anyhow::Result;

mod day1;
mod day2;
mod day3;

fn main() -> Result<()> {

    let filename = "input";
    
    let day = 3;

    match day {

        1 => day1::day_1_2018(filename)?,
        2 => day2::day_2_2018(filename)?,
        3 => day3::day_3_2018(filename)?,
        _ => println!("Enter a day"),


    };

    Ok(())
}

