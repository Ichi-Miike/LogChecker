mod utils;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No file specified");
        std::process::exit(-1);
    }
}



#[cfg(test)]
mod main_tests {
    
}