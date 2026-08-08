mod cli;
use std::io;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    cli::display_banner();
    cli::engage_conection().await?;
    loop {
        cli::prompt_user();
        let usr_input = cli::read_user_input(&mut io::stdin().lock());
        println!("You entered : {}", usr_input);
    }
}
