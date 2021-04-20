use std::{env, io, fs, path};

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
    fs::write(path, data).expect("Unable to write file");
}

fn initialize_words(){
    let players = input_int("How many players are going to play?");
    let words_per_player = input_int("How many players are going to play?");
    let path: path::PathBuf = env::current_dir().unwrap().join("tmp/words");
    write_to_file(path, "check")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone().to_string();
    println!("-----WELCOME TO ACTIVITY GAME-----");
    match command.as_str() {
        "start" => initialize_words(),
        _ => println!("Invalid argument")
    }
    print!("{}[2J", 27 as char);
}
