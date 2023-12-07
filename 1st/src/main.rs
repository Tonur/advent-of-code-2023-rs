fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    
    // split by newline
    let lines = input.split("\n");

    let mut sum: u32 = 0;

    for line in lines {
        // find the first number
        if line.is_empty() {
            continue;
        }
        
        let digits: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();

        let first = digits.first().unwrap().to_digit(10).unwrap();
        let last = digits.last().unwrap().to_digit(10).unwrap();

        let combined = first * 10 + last;

        sum += combined;
    }

    println!("{}", sum);

    Ok(())
}
