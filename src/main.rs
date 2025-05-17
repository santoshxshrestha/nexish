use std::process::Child;
use std::{env, io::{ stdin, stdout, Write}, process::{Command, Stdio}};
use std::path::Path;

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
                continue;
            }

            let command = &parsed[0];
            let args = &parsed[1..];
            // let args = parts.next();

            match command.as_str() {
                "cd" => {
                    let new_dir = args.get(0).map(|s| s.as_str()).unwrap_or("/");
                    let root = Path::new(new_dir);

                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}",e);
                    }

                    previous_command = None;

                },

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
