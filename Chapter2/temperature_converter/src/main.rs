use std::io;
use regex::Regex;

fn main() -> io::Result<()> {
    println!("Input temperature to convert (i.e. 32C or 32F)");
    
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let regex = Regex::new(r"^([\d]+?)\s?([CFcf]{1})\r\n$").unwrap();

    let format_regex_match = regex.is_match(&buffer);

    if format_regex_match
    {
        let caps = regex.captures(&buffer).unwrap();

        let temperature_number = caps.get(1).unwrap().as_str();
        let temperature_number = temperature_number.parse::<f32>().unwrap();
        
        let is_celsius = {
            let type_str = caps.get(2).unwrap().as_str();
            type_str.contains(['c','C'])
        };

        if is_celsius
        {
            let fahrenheit_temp = temperature_number * 1.8 + 32.0;
            println!("Fahrenheit equivalent: {:.2}F", fahrenheit_temp);
        }
        else
        {
            let celsius_temp = (temperature_number - 32.0) / 1.8;
            println!("Celsius equivalent: {:.2}C", celsius_temp);
        }
    }
    else
    {
        println!("Invalid input :(");
    }

    Ok(())
}