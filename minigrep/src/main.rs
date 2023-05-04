use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problemen beim Parsing der Argumenten: {err}");
        process::exit(1);

        //unwrap_or_else is defined on Result<T,E>
        //it allows to define custom error handling
        //calls the closure. passes inner value of Err
    });

    minigrep::inialize();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}