#[macro_use] extern crate matches;

extern crate rustyline;
extern crate clap;

use clap::App;

mod lexer;
mod eval;

fn main() {
    let matches = App::new("plic")
        .version("0.1.0")
        .author("Nicolas Bonnefon <nicolas@bonnefon.org>")
        .about("A tiny LISP interpreter.")
        .args_from_usage(
            "-e, --eval=[EXPRESSION] 'Evaluate the passed expression'
             -v...                   'Sets the level of verbosity'
             [INPUT]                 'Evaluate the content of this file'")
        .get_matches();

    if let Some( eval ) = matches.value_of("eval") {
        println!("Using expression: {}", eval);
        match eval::evaluate_line( eval.to_string() ) {
            Ok(n) => println!("{}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    else if let Some( file ) = matches.value_of("INPUT") {
        println!("Using input file: {}", matches.value_of("INPUT").unwrap());
    }
    else {
        repl();
    }
}

fn repl() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        let result = match readline {
            Ok(line) => eval::evaluate_line( line ),
            Err(_)   => return(),
        };
        match result {
            Ok(n) => println!("Result: {}", n),
            Err(e) => println!("Error: {}", e),
        };
    }
}
