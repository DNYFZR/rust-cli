// CLI Module
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use std::str::SplitWhitespace;

fn help() {
    println!("    command  |  action  ");
    println!("----------------------------------------------");
    println!("- cd         |    navigate to a directory, use ../ to move up");
    println!("- ls         |    list files & directories in current location");
    println!("- exit       |    terminate the interface");
    println!("- help       |    print the terminal command guide");
    println!("- newdir     |    create a new directory");
    println!("- newfile    |    create a new file of specified ext. type");
    println!("- openfile   |    open a text based file and print the content to the terminal");
    println!("- searchfile |    search a text based file and print highlighted content to the terminal if found");
    println!("");
}

fn get_cwd() -> PathBuf {
   return std::env::current_dir().ok().unwrap();
}

fn parse_cwd(cwd: &PathBuf) -> String {
    return cwd.display().to_string();    
}

fn move_cwd(args: SplitWhitespace<'_>) {
    let new_dir = args
        .peekable()
        .peek()
        .map_or("/", |x| *x);
    
    let root = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let new_dir = if new_dir == "-" { root } else { new_dir.to_string() };
    
    if let Err(e) = std::env::set_current_dir(&new_dir) {
        eprintln!("{}", e)
    }
}

fn list_cwd(cwd:&PathBuf) {
    let cwd_content = fs::read_dir(cwd.clone()).unwrap();
    
    for path in cwd_content {
        let item = path
            .unwrap()
            .path()
            .display()
            .to_string();

        let item = item
            .split(&parse_cwd(&cwd))
            .collect::<Vec<_>>();
        
        println!("{:?}", item.get(1).unwrap().replace("\\", ""));
    }
    println!("");
}

fn make_directory(path: &str) {
    fs::create_dir(path).ok();
}

fn make_file(path: &str) {
    fs::File::create_new(path).ok();
}

fn open_file(path: &str) {
    let mut f = fs::File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok();

    println!("{}", buf);
    println!("");
}

fn search_file(path: &str, find: &str) {
    let mut f = fs::File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok();

    // Create matching case strings
    let buf_matcher = &buf.to_lowercase();
    let search_term = &find.to_lowercase();

    // Run search 
    if buf_matcher.contains(&find.to_lowercase()) {
        let output: Vec<&str> = buf_matcher.split_inclusive(search_term).collect();
        let output_last = output.clone().pop().unwrap();

        println!("--- START OF FILE ---");

        for part in output {
            println!("{}", part);
            
            if part == output_last {
                println!("--- END OF FILE ---");
            } else {
                println!("----------------");
            }
        }
        return;
    }

    // Handle no-match results
    println!("{find} not in {path}");

}

pub fn run() {
    loop {
        // Get current dir
        let cwd = get_cwd();
        let cwd_str:String = parse_cwd(&cwd).replace("\\", "/");
        print!("{cwd_str}>_");

        io::stdout().flush().unwrap();

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input error");

        // Parse user input
        let mut args = input.trim().split_whitespace();
        let command = args.next().unwrap_or("");

        // Execute Command
        match command {
            "exit" => break,
            "help" => help(),
            "cd" => move_cwd(args),
            "ls" => list_cwd(&cwd),
            "newdir" => make_directory(args.next().unwrap()),
            "newfile" => make_file(args.next().unwrap()),
            "openfile" => open_file(args.next().unwrap()),
            "searchfile" => {
                let path = args.next().unwrap();
                let find = args.next();

                if find != None {
                    search_file(path, find.unwrap());
                } 
                
                else {
                    println!("Please enter a search term : searchfile [path] [search term]");
                }
            },

            command => {
                let child = Command::new(command)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .spawn();

                match child {
                    Ok(mut child) => {
                        child.wait().expect("command execution error");
                    },
                    Err(e) => eprintln!("failed to execute command : {}", e),
                }
            }
        }
    }
}
