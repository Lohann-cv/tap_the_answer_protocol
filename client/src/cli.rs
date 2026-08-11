//mod cli_struct;
use crate::cli_struct::{StreamType, IOResult, ClientEnvironement};
use colored::Colorize;
use std::error::Error;
use std::io::{self, BufRead, ErrorKind, Write};
// use std::path::Path;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/*To handle handle error gracefully make Enum*/

pub async fn engage_conection(env: &mut ClientEnvironement) -> Result<(StreamType, StreamType), Box<dyn Error>> {
    let (mut reader, mut writer) = setup_client(&env).await?;
    /*read for proto connection*/
    handle_tcp_message(&mut reader, &env).await?;
    /*write for the name*/
    loop {
        let res = handle_tcp_message(&mut writer, &env).await?;
        match handle_tcp_message(&mut reader, &env).await? {
            IOResult::Succes(name) => {
                env.set_name(name.to_string());
                break;
            }
            IOResult::Error(err_desc) => {
                println!("{}", err_desc);
            }
        }
    }
    env.is_authenticate = true;
    Ok((reader, writer))
}

pub async fn setup_client(env: &ClientEnvironement) -> Result<(StreamType, StreamType), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let socket = dotenvy::var("SOCKET")?;
    let stream = TcpStream::connect(socket).await?;
    let (mut reader, mut writer) = stream.into_split();
    Ok((StreamType::Read(reader), StreamType::Write(writer)))
}

pub async fn handle_tcp_message(stream: &mut StreamType, env: &ClientEnvironement) -> Result<IOResult, Box<dyn Error>> {
    match stream {
        StreamType::Read(ref mut reader) => {
            let mut buffer = vec![0; 8192];
            let n = reader.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("TMP : readed {}", &response);
            if response.contains("ERR ") {
                return Ok(IOResult::Error(response.to_string()));
            } else {
                return Ok(IOResult::Succes(response.to_string()));
            }
        }
        StreamType::Write(ref mut writer) => {
            prompt_user(env);
            let user_message = read_user_input(&mut io::stdin().lock());
            /*Error hande*/
            println!("TMP : writed {}", &user_message);
            writer.write_all(&user_message.as_bytes());
            return Ok(IOResult::Succes(user_message));
        }
    }
}

pub fn read_user_input<R: BufRead>(inputer: &mut R) -> String {
    let mut usr_input = String::new();

    match inputer.read_line(&mut usr_input) {
        Ok(_) => usr_input,
        Err(e) => match e.kind() {
            ErrorKind::InvalidData => String::from("The prompt is not UTF-8 !"),
            ErrorKind::Interrupted => String::from("The opperation was interrupted"),
            _ => String::from("I/O error"),
        },
    }
}

pub fn prompt_user(env: &ClientEnvironement) {
    if env.is_authenticate {
        let message = format!("{:?} please enter your command !\n>>> ", env.name);
        print!("{}", message.bold());
    } else {
        print!("{}", "Please set your identity !\n>>> ".bold());
    }
    io::stdout().flush().expect("flush error");
}

pub fn display_banner() {
    let banner = r#"
     _________     ________     ________ 
    /         \   /  ____  \   /   ___  \
    \___   ___/  /  |    |  \  |  |   | |
       |   |     |  |____|  |  |  |___| |
       |   |     |   ____   |  |   _____/
       |   |     |  |    |  |  |  |
       |   |     |  |    |  |  |  |
       |   |     |  |    |  |  |  |
       \___/     \__/    \__/  \__/
    "#;
    println!("{}", banner.cyan().bold());
    println!("\t{}", "====> Made by Bombus.corp <====".yellow().blink());
    println!(
        "{}",
        "Copyright (c) 2026 Author. All Rights Reserved."
            .red()
            .dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_user_input() {
        let mut test_buf = Cursor::new(b"Bum Rocker\n");
        let result = read_user_input(&mut test_buf);
        assert_eq!(result, "Bum Rocker\n");
    }

    #[test]
    fn test_read_user_input_utf8_error() {
        let invalid_bytes = vec![0xff, 0xff, 0xff];
        let mut test_buf = Cursor::new(invalid_bytes);
        let result = read_user_input(&mut test_buf);
        assert_eq!(result, "The prompt is not UTF-8 !")
    }
}
