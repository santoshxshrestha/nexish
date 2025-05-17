#![allow(unused)]
use std::iter::Cloned;
use std::process::Child;
use std::string;
use std::sync::{Arc, Mutex};
use std::{env, io::{self, stdin, stdout, Write}, process::{Command, Stdio}};
use std::path::Path;

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        // let mut previous_command = None;
        // let mut previous_command = Arc::new(Mutex::new(None::<String>));
        // let cloned_prev  = Arc::clone(&previous_command);

        let prev_command = Arc::new(Mutex::new(None::<String>));
        let previous_command  = Arc::clone(&prev_command);


        while let Some(command) = commands.next() {
            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            // let args = parts;
            let args = parts.next();

            match command {
                "cd" => {
                    let new_dir = args.iter().peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);

                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}",e);
                    }

                    // previous_command = None;
                    let mut prev = previous_command.lock().unwrap();
                    *prev = None;
                },

                "exit" => return,

                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some(){
                        Stdio::piped()
                    }else{
                        Stdio::inherit()
                    };


                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        },

                        Err(e) => {
                            previous_command = None;
                            eprintln!("nexsh: command not found: {}",command);

                        }
                    }

                },
            }
            if let Some(mut final_command) = previous_command{
                final_command.wait();
            }

        }




    }
}
