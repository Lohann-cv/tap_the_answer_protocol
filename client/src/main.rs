mod cli;
use std::io;

fn main() {
    cli::display_banner();
    loop {
        cli::prompt_user();
        let usr_input = cli::read_user_input(&mut io::stdin().lock());
        println!("You entered : {}", usr_input);
    }
}
