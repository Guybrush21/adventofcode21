use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();    
    println!("starting with argument: {:?}", args);    

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
     
    let lines = contents.lines();
    let mut increase = -1;    
    let mut prev = 0;    

    for elem in lines {
        let curr = elem.parse::<i32>().unwrap();        
        if curr > prev{
            increase += 1;
            println!("increased");
        }
        prev = curr;
    }

    println!("Total increas = {}", increase);
   
}
