// used for read args
use std::env;

fn main() {
    let config = rlox::parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(64);
    });
    if let Err(err) = rlox::run(config) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

}