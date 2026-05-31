#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}
fn describe(d: Direction) {
    match d {
        Direction::North => println!("Going up"),
        Direction::South => println!("Going down"),
        Direction::East => println!("Going right"),
        Direction::West => println!("Going left"),
    }
}
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(text) => println!("{}", text),
        Message::Color(r, g, b) => {
            println!("Color: ({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    let d1 = Direction::North;
    let d2 = Direction::South;
    let d3 = Direction::East;
    let d4 = Direction::West;

    println!("{:?}", d1);

    describe(d1);
    describe(d2);
    describe(d3);
    describe(d4);

    process(Message::Quit);

    process(Message::Move {
        x: 100,
        y: 200,
    });

    process(Message::Write(
        String::from("Hello Rust")
    ));

    process(Message::Color(
        255,
        0,
        0,
    ));
}