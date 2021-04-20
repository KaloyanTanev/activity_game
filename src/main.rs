//TODO: check conventions for imports
use std::{env, io, path};
use std::fs::write;
use std::fs::read_to_string;
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
        .expect("Please give me correct string number!");

    inp
}

fn read_file(path: path::PathBuf) -> Vec<String>{
    read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
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

fn clear_file(path: path::PathBuf){
    let path: path::PathBuf = env::current_dir().unwrap().join("tmp/words");
    write(path, "").expect("Unable to write file");
}

fn initialize_words(){
    println!("-----WELCOME TO ACTIVITY GAME-----");
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
    println!("-----Words added, get ready for action!-----");
}

fn clear(){
    clear_file(env::current_dir().unwrap().join("tmp/words"));
    println!("-----File cleared, you can begin new game-----");
}

fn round(){
    println!("-----Start round-----");
    let words = read_file(env::current_dir().unwrap().join("tmp/words"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone().to_string();
    match command.as_str() {
        "start" => initialize_words(),
        "clear" => clear(),
        "round" => round(),
        _ => println!("Invalid argument")
    }
}
