enum Command {
    Quit,
    Move(i32,i32),
    AlsoMove(i32,i32),
    Write(String),
    Dance
}

fn main() {
    let mut position = (0, 0);
    let mut message = String::new();

    execute_command(&mut position, &mut message, Command::Move(1,2));
    assert_eq!(position, (1, 2));

    execute_command(&mut position, &mut message, Command::AlsoMove(5,0));
    assert_eq!(position, (6, 2));

    execute_command(&mut position, &mut message, Command::Write(String::from("hello, world!")));
    assert_eq!(message, String::from("hello, world!"));
}

fn execute_command(position: &mut (i32, i32), message: &mut String, command: Command) {
    match command {

    }
}
