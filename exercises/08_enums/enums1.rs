// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit =1;
    Echo =2;
    Move=3;
    ChangeColor=4;
}

fn main() {
    println!("{:?}", Message::Quit as u8);
    println!("{:?}", Message::Echo as u8);
    println!("{:?}", Message::Move as u8);
    println!("{:?}", Message::ChangeColor as u8);
}
