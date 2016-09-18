#[macro_use] extern crate matches;

extern crate rustyline;

mod lexer;

fn main() {
    repl();
}

fn repl() {
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => evaluate_line( line ),
            Err(_)   => return(),
        }
    }
}

fn evaluate_line( line: String ) {
    let mut tokens = lexer::tokens( &line );
    loop {
        match tokens.next() {
            Some(token) => println!( "Token: {:?}", token ),
            None        => return(),
        }
    }
}

