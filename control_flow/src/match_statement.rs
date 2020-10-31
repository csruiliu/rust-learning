
fn main() {
    let country_code = 1001;
    
    // rust forces uers to consider all possible cases in match
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        //inclusive 1000
        1..=1000 => "unknown",
        _ => "invalid" 
    };

    println!("the country with the code {} is {}", country_code, country);
}
