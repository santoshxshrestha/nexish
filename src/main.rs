use std::io::Write;
use std::env::current_dir;
use std::fmt;
use std::process::Child;
use std::{env, io::{ stdin, stdout}, process::{Command, Stdio}};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use dirs;
use nix::unistd::Uid;
use users::{get_user_by_uid};
use chrono::{Local,self};

struct LsEntry(String);
impl fmt::Display for LsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  ", self.0)
    }
}

impl AsRef<str> for LsEntry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

struct LsEntries(Vec<LsEntry>);
impl LsEntries {
    fn new()-> Self {
        LsEntries(Vec::new())

    }

    fn push(&mut self, entry: LsEntry) {
        self.0.push(entry);
    }
}
impl<'a> IntoIterator for &'a LsEntries{
    type Item = &'a LsEntry;
    type IntoIter = std::slice::Iter<'a, LsEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl fmt::Display for LsEntries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.0{
            write!(f, "{entry}")?;
        }
        Ok(())
    }

}

fn get_username()-> String{
    let uid = Uid::current().as_raw();
    get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(||"unknown".to_string())
}

fn get_relative_dir() -> String {
    let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if let Some(home_dir) = dirs::home_dir(){
        if let Ok(stripped) = current_dir.strip_prefix(&home_dir){
            return format!("~/{}", stripped.display());
        }
    }
    current_dir.display().to_string()
}

fn get_current_dir() -> std::io::Result<PathBuf> {
    std::env::current_dir()
}

fn get_time() -> String {
    let current_time = Local::now();
    let fomatted = format!("{}",current_time.format("%H:%M"));
    return fomatted;
}


fn main() {
    loop {
        println!("  {} in {} at  {}",get_username(), get_relative_dir(),get_time());
        print!("-> ");
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
                    let mut hidden = false;
                    let mut dir_path = ".".to_string();

                    // Parse arguments
                    for arg in args {
                        if arg == "-a" {
                            hidden = true;
                        } else {
                            dir_path = arg.to_string();
                        }
                    }

                    let entries = match fs::read_dir(&dir_path) {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("ls: Error reading directory: {}", e);
                            continue;
                        }
                    };

                    let mut output = LsEntries::new();
                    for entry in entries {
                        match entry {
                            Ok(content) => {
                                let file_name = content.file_name();
                                let file_name_str = file_name.to_string_lossy();
                                if !hidden && file_name_str.starts_with('.') {
                                    continue;
                                }
                                output.push(LsEntry(file_name_str.to_string()));
                            }
                            Err(e) => {
                                eprintln!("ls: Error rendering entry: {}", e);
                            }
                        }
                    }
                    println!("{}", output);
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
                    match get_current_dir() {
                        Ok(path) => println!("{}",path.display()),
                        Err(e) => eprintln!("pwd: error retrieving current directory: {}",e),
                        
                    }
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
                            eprintln!("nexish: command not found: {}",command);

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

