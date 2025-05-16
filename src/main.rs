#![allow(unused)]
use std::{env, io::{self, stdin, stdout, Write}, process::Command};
use std::path::Path;

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();


        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        // let args = parts;
        let args = parts.next();


        match command {
            "cd" => {
                let new_dir = args.iter().peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}",e);
                }
            },

            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();
                child.wait();
            }   

        }
    }

}
