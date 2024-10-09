
use std::env;
use std::fs::{OpenOptions, File};
use std::io::Read;
use whoami;


fn manual() {
println!("\n
add [note]  . . adding note
remove [id] . . removes the note
wipe  . . . . . wipes all notes
ls  . . . . . . list all notes")
}


fn add(mut file: File, args: &Vec<String>) {

    if args.len() < 3 {return;}

    let note = &args[2];

    let mut content = String::new();

    let _ = file.read_to_string(&mut content);
    

    println!("{note:?}");


}


fn remove(file: File) {
    println!("{file:?}")
}


fn wipe(file: File) {
    println!("{file:?}")
}


fn ls(file: File) {
    println!("{file:?}")
}


fn open_file() -> Result<File, std::io::Error> {

    let file_path = {
        if cfg!(windows) {format!("C:\\Users\\{}\\.notes.txt", whoami::username())} 
        else if cfg!(target_os = "macos") {format!("/Users/{}/.notes", whoami::username())} 
        else {format!("/home/{}/.notes", whoami::username())}
    };


    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path.as_str())
}


pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Too few arguments. Try using --help for manual.");
        return;
    }

    let file = open_file().unwrap();


    println!("{file:?}");
    

    match args[1].as_str() {
        "add" => add(file, &args),
        "remove" => remove(file),
        "wipe" => wipe(file),
        "ls" => ls(file),
        "man" => manual(),
        _ => println!("Not a valid argument. See --help for manual.")
    }
   

}

