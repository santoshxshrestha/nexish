use std::env::current_dir;
use std::process::Child;
use std::{env, io::{ stdin, stdout, Write}, process::{Command, Stdio}};
use std::path::{Path, PathBuf};
use std::fs;
use dirs;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split('|').map(|s| s.trim()).peekable();
        let mut previous_command: Option<Child> = None;

        while let Some(command) = commands.next() {
            let parsed = shell_words::split(command).expect("Failed to parse comand");
            if parsed.is_empty(){
                continue;  //user press enter do nothing
            }

            let command = &parsed[0];
            let args = &parsed[1..];
            // let args = parts.next();

            match command.as_str() {

                "rmdir" => {
                    if args.is_empty(){
                        eprintln!("rmdir: missing operand");
                    }else{
                        for dir in args {
                            match fs::remove_dir(dir){
                                Ok(_) => {}
                                Err(e) => eprintln!("rmdir: failed to remove '{}': {}",dir,e),
                            }
                        }
                    }
                },

                "pwd" => {
                    let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from("."));
                    println!("{}",current_dir.display());
                },

                "cd" => {
                    let new_dir = args.get(0).map(|s| s.as_str());
                    let target_dir = match new_dir {
                        Some(path) => Path::new(path).to_path_buf(),
                        None=> dirs::home_dir().expect("Could not get home directory"),
                    };

                    if let Err(e) = env::set_current_dir(&target_dir){
                        eprintln!("{}",e);
                    }

                    previous_command = None;

                },
                "mkdir" => {
                    let mut args_iter = args.iter();

                    let mut recursive = false;
                    let mut target: Option<&str> = None;

                    while let Some(arg) = args_iter.next() {
                        if arg == "-p"{
                            recursive = true;
                        }else {
                            target = Some(arg);
                            break;
                        }

                    }

                    // let new_dir = args.get(0);
                    match target {
                        Some(dir) => {
                            let path = Path::new(dir);
                            let result  = if recursive {
                                fs::create_dir_all(path)
                            }else {
                                fs::create_dir(path)
                            };

                            if let Err(e) = result {
                                eprintln!("mkdir: cannot create directory '{}': {}",dir,e);
                            }

                        },

                        None => {
                            eprintln!("mkdir: missing operand");
                        },

                    }
                    previous_command = None;
                }

                "exit" => return,

                command => {
                    let stdin = match &mut previous_command {
                        Some(child) => {
                            let stdout = child.stdout.take().expect("Failed to take the stdout");
                            Stdio::from(stdout)
                        },

                        None => Stdio::inherit(),
                    };

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

                        Err(_) => {
                            previous_command = None;
                            eprintln!("nexsh: command not found: {}",command);

                        }
                    }

                },
            }

        }

        if let Some(ref mut final_command) = previous_command{
            final_command.wait().unwrap();
        }

    }
}
