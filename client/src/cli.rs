use colored::Colorize;
use std::io::{self, BufRead, ErrorKind, Write};

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

pub fn prompt_user() {
    print!("{}", "Please enter your command !\n>>> ".bold());
    io::stdout().flush().expect("Flush error");
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
