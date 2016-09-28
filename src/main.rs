#[macro_use] extern crate matches;

extern crate rustyline;

mod lexer;
mod eval;

fn main() {
    repl();
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
