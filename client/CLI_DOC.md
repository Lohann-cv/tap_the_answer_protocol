# COMMAND LINE INTERFACE

*The CLI as well as the GUI is supposed to be held by the client that wants to conect to the game.*

## Tech Stack Used

> Crossterm
Crossterm is a low level rust crate, it's used to use to create CLI that is communicating with a server.
The power of Crossterm is that it's able to separate the terminal in one or more space, such as a space for the event message and a space for the user message. Moreover it is compatible with Tokio.

> Ratatui
Ratatui is used with Crossterm to create a flashy Terminal User Interface. It's able to make layouts to cut the terminal in one or more space, widgets that are already made. The layouts space are regenerate at runtime to handle the comming message or use's modifications.
