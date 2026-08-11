mod cli;
mod cli_struct;
//use cli_struct::{StreamType, IOResult, ClientEnvironement};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut env = cli_struct::ClientEnvironement::new();
    cli::display_banner();
    let (mut reader, mut writer) = cli::engage_conection(&mut env).await?;
    let env_copy = env.clone();

    let reader_handle = tokio::spawn(async move {
        loop {
            cli::handle_tcp_message(&mut reader, &env_copy);
        }
    });

    let writer_handle = tokio::spawn(async move {
        loop {
            cli::handle_tcp_message(&mut writer, &env);
        }
    });
    let reader_result = reader_handle.await;
    let writer_result = writer_handle.await;
    Ok(())
}
