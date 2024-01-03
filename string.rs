fn main() {
    let string: &str = "fixed length string
    can be multiline";

    println!("{}", string);

    let raw_str = r"You don't need to escape the \ character";
    println!("{}", raw_str);

    // String from constructor
    let mut name = String::new();
    name.push_str("Johnny");
    println!("{}", name);

    let mut surname = String::from("Deep");
    println!("{}", surname);

    // concatenation of strings
    let mut full_name = name + " " + &surname;

    println!("{}", full_name);

    let name = String::from(full_name);
    let parts: Vec<&str> = name.split_whitespace().collect();

    println!("{:?}", parts);
    println!("{}", parts[0]);
}
