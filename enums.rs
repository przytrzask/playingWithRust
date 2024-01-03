#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Black,
    Rgb(u8, u8, u8),
}

fn main() {
    let colour = Colour::Rgb(255, 255, 255);

    println!("{:?}", colour)
}
