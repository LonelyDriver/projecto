use std::env;
use std::process;
use projecto;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let name = projecto::check_args(&args).unwrap_or_else(|err| {
        println!("Parsing arguments error: {}", err);
        process::exit(-1);
    });
    
    println!("Name={}", name);
    
    if let Err(e) = projecto::create_project(&name) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
