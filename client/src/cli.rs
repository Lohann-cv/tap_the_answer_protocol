use crate::cli_struct::{ClientEnvironement, IOResult, StreamType};
use colored::Colorize;
use std::error::Error;
// use std::io::{self, BufRead, ErrorKind, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/*To handle handle error gracefully make Enum*/

pub async fn engage_conection(
    env: &mut ClientEnvironement,
) -> Result<(StreamType, StreamType), Box<dyn Error>> {
    let (mut reader, mut writer) = setup_client().await?;
    handle_tcp_message(&mut reader, env).await?;
    loop {
        let _ = handle_tcp_message(&mut writer, env).await?;
        match handle_tcp_message(&mut reader, env).await? {
            IOResult::Succes(_) => {
                break;
            }
            IOResult::Error(err_desc) => {
                if err_desc.contains("ERR 902 CONNECCTION_LOST") {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::ConnectionAborted,
                        "Connection lost",
                    )));
                }
                println!("{}", err_desc);
            }
        }
    }
    env.is_authenticate = true;
    Ok((reader, writer))
}

pub async fn setup_client() -> Result<(StreamType, StreamType), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let socket = dotenvy::var("SOCKET")?;
    let stream = TcpStream::connect(socket).await?;
    let (reader, writer) = stream.into_split();
    Ok((StreamType::Read(reader), StreamType::Write(writer)))
}

pub async fn handle_tcp_message(
    stream: &mut StreamType,
    env: &ClientEnvironement,
) -> Result<IOResult, Box<dyn Error>> {
    match stream {
        StreamType::Read(ref mut reader) => {
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                Ok(IOResult::Error(String::from("ERR 902 CONNECCTION_LOST")))
            } else if line.contains("ERR") {
                Ok(IOResult::Error(line.to_string()))
            } else {
                println!("{}", line);
                Ok(IOResult::Succes(line.to_string()))
            }
        }
        StreamType::Write(ref mut writer) => {
            prompt_user(env).await?;
            // let user_message = read_user_input(&mut io::stdin().lock());
            // let _ = writer.write_all(user_message.as_bytes()).await;
            let input = tokio::io::stdin();
            let mut reader = BufReader::new(input);
            let mut user_message = String::new();
            reader.read_line(&mut user_message).await?;
            let _ = writer.write_all(user_message.as_bytes()).await;
            Ok(IOResult::Succes(user_message.to_string()))
        }
    }
}

// pub async fn read_user_input<R: BufRead>(inputer: &mut R) -> String {
//     let mut usr_input = String::new();

//     match inputer.read_line(&mut usr_input) {
//         Ok(_) => usr_input,
//         Err(e) => match e.kind() {
//             ErrorKind::InvalidData => String::from("The prompt is not UTF-8 !"),
//             ErrorKind::Interrupted => String::from("The opperation was interrupted"),
//             _ => String::from("I/O error"),
//         },
//     }
// }

pub async fn prompt_user(env: &ClientEnvironement) -> Result<(), Box<dyn Error>> {
    let mut message = String::new();
    if env.is_authenticate {
        message.push_str(&format!("{}", "Please enter your command !\n>>> ".bold()));
    } else {
        message.push_str(&format!("{}", "Please set your identity !\n>>> ".bold()));
    }
    let mut output = tokio::io::stdout();
    output.write_all(message.as_bytes()).await?;
    output.flush().await?;
    Ok(())
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::Cursor;

//     #[test]
//     fn test_read_user_input() {
//         let mut test_buf = Cursor::new(b"Bum Rocker\n");
//         let result = read_user_input(&mut test_buf);
//         assert_eq!(result, "Bum Rocker\n");
//     }

//     #[test]
//     fn test_read_user_input_utf8_error() {
//         let invalid_bytes = vec![0xff, 0xff, 0xff];
//         let mut test_buf = Cursor::new(invalid_bytes);
//         let result = read_user_input(&mut test_buf);
//         assert_eq!(result, "The prompt is not UTF-8 !")
//     }
// }
