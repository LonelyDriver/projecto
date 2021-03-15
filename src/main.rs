use std::env;
use std::process;
use projecto;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = projecto::check_args(&args).unwrap_or_else(|err| {
        println!("Parsing arguments error: {}", err);
        process::exit(-1);
    });
    
    if let Err(e) = projecto::create_project(&config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
