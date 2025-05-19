use std::io::Write;
use std::env::current_dir;
use std::fmt;
use std::process::Child;
use std::{env, io::{ stdin, stdout}, process::{Command, Stdio}};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use dirs;

struct LsEntry(String);
impl fmt::Display for LsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  ", self.0)
    }
}

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
                "ls" => {
                    let mut targets:Vec<&str> = Vec::new();
                    let mut hidden = false;
                    
                    for arg in args.iter(){
                        if arg == "-a"{
                            hidden = true;
                        }else{
                            targets.push(arg);
                        }
                    }
                    let entries: Result<fs::ReadDir, std::io::Error> = if let Some(dir) = targets.iter().next(){
                        fs::read_dir(dir)
                    }else {
                        fs::read_dir(".")
                    };

                    for entry in entries {
                        match entry {
                            Ok(read_dir) => {
                                let file_name = read_dir.file_name();
                                let entry = Vec::new();
                                if hidden {
                                    continue;
                                }else {
                                    entry = LsEntry(file_name.to_string_lossy().to_string());
                                    entry.push(LsEntry(file_name))
                                }
                                print!("{}",entry);
                            },
                            Err(e) => {
                                eprintln!("ls: Error rendering entry: {}",e);
                            }
                        }
                    }
                    println!();
                },

                "touch" => {
                    if args.is_empty(){
                        eprintln!("touch: missing file operand");
                    }else {
                        for file in args{
                            match File::create_new(file) {
                                Ok(_) => {},
                                Err(e) => eprintln!("touch: file creating failed '{}': {}",file,e),
                            }
                        }
                    }
                },

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
                    let mut recursive = false;
                    let mut targets:Vec<&str> = Vec::new();
                    let mut args_iter = args.iter();


                    while let Some(arg) = args_iter.next() {
                        if arg == "-p"{
                            recursive = true;
                        }else {
                            targets.push(arg);
                        }
                    }


                    for dir in targets {
                        let path = Path::new(dir);
                        let result = if recursive {
                            fs::create_dir_all(path)
                        }else {
                            fs::create_dir(path)
                        };

                        if let Err(e) =  result {
                            eprintln!("mkdir: cannot create directory '{}': {}",dir,e);
                        }
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
