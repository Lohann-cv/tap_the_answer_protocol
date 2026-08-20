use std::error::Error;

pub async fn look(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn move_command(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn quit(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("QUIT") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn chat(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn who(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn group_create(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn group_invite(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn group_join(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn group_leave(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn take(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn drop_command(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn inventory(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn talk(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn attack(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn status(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn quest(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}

pub async fn quests(message: String) -> Result<String, Box<dyn Error + Send>> {
    if message.contains("LOOK") {
        Ok(message)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid message",
        )))
    }
}
