extern crate rustyline;

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
    let mut tokens = code_tokens( &line );
    loop {
        match tokens.next() {
            Some(token) => println!( "Token: {}", token ),
            None        => return(),
        }
    }
}

fn code_tokens<'a>( code: &'a String ) -> PelicCodeTokens<'a> {
    PelicCodeTokens { iter: code.chars() }
}

struct PelicCodeTokens<'a> {
    iter: std::str::Chars<'a>,
}

impl<'a> Iterator for PelicCodeTokens<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let c = self.iter.next();
        match c {
            Some(car) => Some(car.to_string()),
            None      => None
        }
    }
}
