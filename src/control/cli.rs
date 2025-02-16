use std::io::{stderr, stdin, stdout, Stderr, Stdin, Stdout, Write};

const PROMPT: &str = "Command> ";

#[derive(Debug, Clone, Copy)]
enum Commands {
    Init,
    Status,
    New,
}

pub fn run_cli() {
    let sin: Stdin = stdin();
    let mut sout: Stdout = stdout();
    let mut serr: Stderr = stderr();

    loop {
        let _ = sout.lock();
        let _ = sout.write(PROMPT.as_bytes());
        let _ = sout.flush();

        let mut user_input_str = String::new();

        let _ = sin.read_line(&mut user_input_str);

        println!("{}", user_input_str);
    }
}
