mod cli;
mod cli_struct;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut env = cli_struct::ClientEnvironement::new();
    cli::display_banner();
    let (mut reader, mut writer) = cli::engage_conection(&mut env).await?;
    let env_copy = env.clone();

    let reader_handle = tokio::spawn(async move {
        loop {
            cli::handle_tcp_message(&mut reader, &env_copy).await;
        }
    });

    let writer_handle = tokio::spawn(async move {
        loop {
            cli::handle_tcp_message(&mut writer, &env).await;
        }
    });
    let reader_result = reader_handle.await;
    let writer_result = writer_handle.await;
    Ok(())
}
