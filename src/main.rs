use std::os::unix::prelude::*;
use std::io::{self, Read, Write};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command};

fn read_command(stack: &Vec<String>) -> String {
    let mut input = String::new();
    
    for byte in io::stdin().lock().bytes() {
        let b = byte.unwrap();
        println!("{}", b);

        input.push(b as char);
    }

    input
}

// read char
// if it is whitespace, append current buffer to vec,
// if it is a char, append to current buffer 
// if it is a enter, run command, append vec to stack, and clear vec
// if last four chars in buffer are up arrow, delete buffer, 
//      replace with last item in stack


fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut current_dir: PathBuf;
    let mut stack: Vec<String> = Vec::new();
    
    loop {
        let s = read_command(&stack);
        current_dir = std::env::current_dir()?;
        println!("{} >", current_dir.display());
        
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut args: Vec<&str> = buffer.split_ascii_whitespace().collect();
        
        stack.push(buffer.to_string());

        if args[0] == "exit" {
            break;
        }

        if args[0] == "cd" {
            let p = Path::new(args[1]).canonicalize()?;
            let cd = env::set_current_dir(p);
            
            continue;
        }
        

        let output = Command::new(args.remove(0))
                        .args(args)
                        .output()
                        .expect("failed to run command");
        

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    Ok(())
}
