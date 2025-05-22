#![allow(unused)]
use std::env::{self, current_dir};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::borrow::Cow;

use chrono::Local;
use dirs;
use git2::Repository;
use os_info;
use reedline::ExampleHighlighter;
use reedline::MenuBuilder;
use whoami;
use nu_ansi_term::Style;

use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, Emacs, FileBackedHistory,
    KeyCode, KeyModifiers, Prompt, PromptEditMode, PromptHistorySearch, Reedline, ReedlineEvent,
    ReedlineMenu, Highlighter, StyledText,
};

struct ShellHighlighter {
    commands: Vec<String>,
}

impl ShellHighlighter {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }
}

impl Highlighter for ShellHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut styled = StyledText::new();
        let mut parts = line.split_whitespace();

        if let Some(first) = parts.next() {
            let is_cmd = self.commands.iter().any(|c| c == first);
            let style = if is_cmd {
                Style::new().fg(nu_ansi_term::Color::Cyan).bold()
            } else {
                Style::new().fg(nu_ansi_term::Color::White).bold()
            };
            styled.push((style, first.to_string()));
        }
        for arg in parts {
            styled.push((Style::new(), " ".to_string())); 
            let style = if arg.starts_with('-') {
                Style::new().fg(nu_ansi_term::Color::Yellow).bold()
            } else if arg.contains('/') || arg.starts_with('.') {
                Style::new().fg(nu_ansi_term::Color::Blue)
            } else {
                Style::new().fg(nu_ansi_term::Color::White)
            };
            styled.push((style, arg.to_string()));
        }
        styled
    }
}

struct ShellPrompt;
impl Prompt for ShellPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Owned(format!(
            "{}{} in {} {}at  {}",
            device_logo(),
            get_username(),
            get_relative_dir(),
            git_current_branch(),
            get_time()
        ))
    }
    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }
    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("-> ")
    }
    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("::: ")
    }
    fn render_prompt_history_search_indicator(&self, _history_search: PromptHistorySearch) -> Cow<str> {
        Cow::Borrowed("history: ")
    }
}

fn get_relative_dir() -> String {
    let current_dir = current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if let Some(home_dir) = dirs::home_dir() {
        if let Ok(stripped) = current_dir.strip_prefix(&home_dir) {
            return format!("~/{}", stripped.display());
        }
    }
    current_dir.display().to_string()
}
fn get_current_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
fn get_time() -> String {
    let current_time = Local::now();
    format!("{}", current_time.format("%H:%M"))
}
fn get_username() -> String {
    whoami::realname()
}
fn device_logo() -> &'static str {
    match os_info::get().os_type() {
        os_info::Type::Windows => " ",
        os_info::Type::Macos => "🍏",
        os_info::Type::Linux => "",
        os_info::Type::Android => "",
        os_info::Type::Redhat => " ",
        os_info::Type::Arch => "󰣇 ",
        os_info::Type::Pop => " ",
        os_info::Type::Ubuntu => " ",
        os_info::Type::Kali => " ",
        os_info::Type::Mint => "󰣭 ",
        os_info::Type::SUSE => " ",
        os_info::Type::Artix => " ",
        os_info::Type::Void => " ",
        os_info::Type::NixOS => " ",
        os_info::Type::Alpine => " ",
        os_info::Type::CentOS => " ",
        os_info::Type::Debian => " ",
        os_info::Type::Gentoo => " ",
        os_info::Type::Unknown => " ",
        _ => "💻",
    }
}
fn git_current_branch() -> String {
    if let Ok(repo) = Repository::discover(".") {
        if let Ok(head) = repo.head() {
            if let Some(branch) = head.shorthand() {
                return format!("  {} ", branch);
            }
        }
    }
    String::new()
}
fn main() {
    let commands = vec![
        "ls".to_string(),
        "rm".to_string(),
        "rmdir".to_string(),
        "touch".to_string(),
        "pwd".to_string(),
        "cd".to_string(),
        "mkdir".to_string(),
        "whoami".to_string(),
        "exit".to_string(),

    ];

    let mut file_candidates = vec![];
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let fname = entry.file_name().to_string_lossy().to_string();
            file_candidates.push(fname);
        }
    }
    let mut completion_list = commands.clone();
    completion_list.extend(file_candidates);

    let completer = Box::new(DefaultCompleter::new_with_wordlen(completion_list.clone(), 1));
    let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::MenuNext,
        ]),
    );

    let edit_mode = Box::new(Emacs::new(keybindings));

    let history_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".nexish_history");

    let mut line_editor = Reedline::create()
        .with_completer(completer)
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(edit_mode)
        .with_highlighter(Box::new(ExampleHighlighter::new(commands.clone())))
        .with_history(Box::new(
            FileBackedHistory::with_file(1000, history_path).expect("history file error"),
        ));

    let prompt = ShellPrompt;

    loop {
        match line_editor.read_line(&prompt) {
            Ok(signal) => match signal {
                reedline::Signal::Success(buffer) => {
                    let input = buffer.trim();
                    if input.is_empty() {
                        continue;
                    }

                    let mut commands = input.split('|').map(|s| s.trim()).peekable();
                    let mut previous_command: Option<Child> = None;

                    while let Some(command) = commands.next() {
                        let parsed = shell_words::split(command).expect("Failed to parse command");
                        if parsed.is_empty() {
                            continue;
                        }
                        let command = &parsed[0];
                        let args = &parsed[1..];

                        match command.as_str() {
                            "whoami" => {
                                println!("{}", get_username());
                            }
                            "ls" => {
                                let mut hidden = false;
                                let mut dir_path = ".".to_string();
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
                                for entry in entries.flatten() {
                                    let file_name = entry.file_name().to_string_lossy().to_string();
                                    if !hidden && file_name.starts_with('.') {
                                        continue;
                                    }
                                    print!("{}  ", file_name);
                                }
                                println!();
                            }
                            "touch" => {
                                if args.is_empty() {
                                    eprintln!("touch: missing file operand");
                                } else {
                                    for file in args {
                                        match File::create(file) {
                                            Ok(_) => {}
                                            Err(e) => eprintln!("touch: file creating failed '{}': {}", file, e),
                                        }
                                    }
                                }
                            }
                            "rmdir" => {
                                if args.is_empty() {
                                    eprintln!("rmdir: missing operand");
                                } else {
                                    for dir in args {
                                        match fs::remove_dir(dir) {
                                            Ok(_) => {}
                                            Err(e) => eprintln!("rmdir: failed to remove '{}': {}", dir, e),
                                        }
                                    }
                                }
                            }
                            "pwd" => match get_current_dir() {
                                Ok(path) => println!("{}", path.display()),
                                Err(e) => eprintln!("pwd: error retrieving current directory: {}", e),
                            },
                            "cd" => {
                                let new_dir = args.get(0).map(|s| s.as_str());
                                let target_dir = match new_dir {
                                    Some(path) => Path::new(path).to_path_buf(),
                                    None => dirs::home_dir().expect("Could not get home directory"),
                                };

                                if let Err(e) = env::set_current_dir(&target_dir) {
                                    eprintln!("{}", e);
                                }
                                previous_command = None;
                            }
                            "mkdir" => {
                                let mut recursive = false;
                                let mut targets: Vec<&str> = Vec::new();
                                let mut args_iter = args.iter();
                                while let Some(arg) = args_iter.next() {
                                    if arg == "-p" {
                                        recursive = true;
                                    } else {
                                        targets.push(arg);
                                    }
                                }
                                for dir in targets {
                                    let path = Path::new(dir);
                                    let result = if recursive {
                                        fs::create_dir_all(path)
                                    } else {
                                        fs::create_dir(path)
                                    };
                                    if let Err(e) = result {
                                        eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
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
                                    }
                                    None => Stdio::inherit(),
                                };
                                let stdout = if commands.peek().is_some() {
                                    Stdio::piped()
                                } else {
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
                                    }
                                    Err(_) => {
                                        previous_command = None;
                                        eprintln!("nexish: command not found: {}", command);
                                    }
                                }
                            }
                        }
                    }
                    if let Some(ref mut final_command) = previous_command {
                        final_command.wait().unwrap();
                    }
                }
                reedline::Signal::CtrlD | reedline::Signal::CtrlC => {
                    println!("Exiting...");
                    break;
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
