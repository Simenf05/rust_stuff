use rand;
use std::io;

const MAX_NUM: u8 = 10; 

fn game(num: u8) -> Result<bool, String> {

    let mut guess = String::new();

    let _ = io::stdin().read_line(&mut guess);



    let guess_num = guess.trim().parse::<u8>();

    match guess_num {
        Ok(value) => {
            return Ok(value == num);
        }
        Err(e) => {

            let msg = format!("{}", e);

            return Err(msg);
        }
    }
}

pub fn run() {

    let num: u8 = (rand::random::<u8>() % MAX_NUM) + 1;

    println!("Gjett et tall mellom 1 og {MAX_NUM}");

    loop {
        
        let win = game(num);

        match &win {
            Ok(_) => (),
            Err(e) => {
                println!("error: {e}");
                continue;
            },
        }


        if win.unwrap() {
            println!("You won");
            break;
        }
        else {
            println!("Try again");
        }
    }
}

