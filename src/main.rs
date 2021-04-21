//TODO: check conventions for imports
use std::{env, io, path, thread};
use std::time::Duration;
use std::fs::write;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::{Mutex, Arc};

extern crate base64_url;

use rand::Rng;
use base64_url::{encode, decode};

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
    let path: path::PathBuf = env::current_dir().unwrap().join("words");
    write(path, "").expect("Unable to write file");
}

fn initialize_words(){
    println!("-----WELCOME TO ACTIVITY GAME-----");
    clear_file(env::current_dir().unwrap().join("words"));
    let players = input_int("How many players are going to play?");
    let words_per_player = input_int("How many words per player?");
    //TODO: Put curr_dir as global var
    for p in 0..players{
        for w in 0..words_per_player{
            let prompt: String = format!("Word {} for player {}", w+1, p+1);
            //TODO: Refactor
            write_to_file(
                env::current_dir()
                .unwrap()
                .join("words")
                ,format!("{}\n", encode(input(prompt.as_str()).trim())).as_str());
        }
        // clear terminal
        print!("{}[2J", 27 as char);
    }
    println!("-----Words added, get ready for action!-----");
}

fn round(){
    println!("-----Start round-----");
    let mut words = read_file(env::current_dir().unwrap().join("words"));
    let timeout = input_int("How many seconds per player?") as u64;

    while !words.is_empty(){
        let stop = Arc::new(Mutex::new(false));
        let stop_clone = Arc::clone(&stop);
    
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout/2));
            println!("-----You have {} seconds left-----", timeout/2);
            thread::sleep(Duration::from_secs(timeout - (timeout/2)));
            *stop.lock().unwrap() = true;
            println!("-----Time is up!-----");
        });

        while *stop_clone.lock().unwrap()==false && !words.is_empty() {
            let idx = rand::thread_rng().gen_range(0..words.len());
            let word = &words[idx];
            println!("{}", String::from_utf8(decode(word).unwrap()).expect("Found invalid UTF-8"));
            if !input("").trim().eq("n") && *stop_clone.lock().unwrap()==false {
                words.remove(idx);
            }
        }
        print!("{}[2J", 27 as char);
        println!("-----Click any button for next player-----");
        input("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone().to_string();
    match command.as_str() {
        "start" => initialize_words(),
        "round" => round(),
        _ => println!("Invalid argument")
    }
}
