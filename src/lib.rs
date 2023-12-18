use std::error::Error;
use std::io::{self, Write, Read};
use std::fs::File;

mod vm;
mod object;

// struct config is reserved for futher extension
pub struct Config {
    path: String,
}

impl Config {
    fn new(path: String) -> Self {
        Self {
            path,
        }
    }
}

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next(); // skip the first arg, which is the program name
    let path = args.next().unwrap_or_default();

    let config = Config::new(path);

    if !args.next().is_none() {
        return Err("Usage: rlox [path/to/lox/script]");
    }
    
    Ok(config) 
} 

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.path.is_empty() {
        return run_prompt();
    } else {
        return run_file(&config.path);
    }
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    println!("run lox from prompt");
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        vm::interpret(&buffer); 
        buffer.clear();
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    println!("run lox from script: {}", path);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    match vm::interpret(&content) {
        vm::InterpreterResult::InterpreterOk => Ok(()),
        vm::InterpreterResult::InterpreterCompileError => Err("lox compile error".into()),
        // vm::InterpreterResult::InterpreterRuntimeError => Err("lox runtime error".into()),
        // vm::InterpreterResult::InterpreterDebug => Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[test]
//     // fn test_parse_args() {
//     //     let args = vec!["rlox".to_string(), "path/to/lox/script".to_string()];
//     //     let config = parse_args(args.into_iter()).unwrap();
//     //     assert_eq!(config.path, "path/to/lox/script");
//     // }

//     // #[test]
//     // fn test_parse_args_no_args() {
//     //     let args = vec!["rlox".to_string()];
//     //     let config = parse_args(args.into_iter()).unwrap();
//     //     assert_eq!(config.path, "");
//     // }

//     // #[test]
//     // fn test_parse_args_too_many_args() {
//     //     let args = vec!["rlox".to_string(), "path/to/lox/script".to_string(), "extra".to_string()];
//     //     let config = parse_args(args.into_iter());
//     //     assert!(config.is_err());
//     // }

//     #[test]
//     fn test() {
        
//     }
// }