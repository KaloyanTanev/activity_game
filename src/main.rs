//TODO: check conventions for imports
use std::{env, io, path};
use std::fs::OpenOptions;
use std::io::prelude::*;

fn input(prompt: &str) -> String{
    let mut input = String::new();

    println!("{}", prompt);
    io::stdin().read_line(&mut input)
        .expect("Couldn't read line");

    input
}

fn input_int(prompt: &str) -> i32{
    let inp: i32 = input(prompt).trim().parse()
        .expect("please give me correct string number!");

    inp
}

fn write_to_file(path: path::PathBuf, data: &str){
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();  

    if let Err(e) = write!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }

}

fn initialize_words(){
    let players = input_int("How many players are going to play?");
    let words_per_player = input_int("How many words per player?");
    //TODO: Put curr_dir as global var
    for p in 0..players{
        for w in 0..words_per_player{
            let prompt: String = format!("Word {} for player {}", w+1, p+1);
            write_to_file(env::current_dir().unwrap().join("tmp/words"), input(prompt.as_str()).as_str());
        }
        // clear terminal
        print!("{}[2J", 27 as char);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone().to_string();
    println!("-----WELCOME TO ACTIVITY GAME-----");
    match command.as_str() {
        "start" => initialize_words(),
        _ => println!("Invalid argument")
    }
}
