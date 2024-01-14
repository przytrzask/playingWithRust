fn farewell(name: &String) {
    println!("So long, {}!", name);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Matt");
    greet(&name);
    farewell(&name);
}
