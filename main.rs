use calc_cli::{tokenize, parse};
use std::io::{self, Write};

fn main() {
    println!("Simple Calculator - Type expressions or 'quit' to exit");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match run_calculation(input) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn run_calculation(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input).map_err(|e| format!("{:?}", e))?;
    let expr = parse(&tokens).map_err(|e| format!("{:?}", e))?;
    let result = expr.eval().map_err(|e| format!("{:?}", e))?;
    Ok(result)
}

