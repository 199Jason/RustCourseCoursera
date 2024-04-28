use std::io;

fn increasenumber(number: i32) -> i32 {

    let mut x = 0;
    let mut ynumber = number;
    while ynumber < 1000 {
        ynumber += 1;
        x += 1;
    };
    x 
    
}


fn main () {

    println!("Input number:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        
        Err(_) => {
            println!("Error: input not valid");
            return;
        },
    };
    
    println!("The input is: {}", number);
    
    if number >= 1000 {
        println!("Your number is right");
    } else if number < 1000 {
        println!("Your number is not high enough");
    };

    println!("Do you want to increase it? (y/n)");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "y" => {
            let x = increasenumber(number);
            println!("Number increased by {}", x);
        },
        "n" => {
            println!("No increasing. Retry");
        },
        _ => {
            println!("Answer not valid");
        },
    } 
}
