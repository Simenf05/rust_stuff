
use std::env;










pub fn run() {


    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        println!("Too few arguments. Try using --help for manual.");
        return;
    }

    println!("{args:?}")
    

}

