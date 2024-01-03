fn main() {
    let birthYear = 1986;
    let age = 2021 - birthYear;

    if birthYear < 1990 {
        println!("Ok boomer")
    } else if birthYear > 1990 && birthYear < 2000 {
        println!("Millenial")
    } else {
        println!("Gen Z")
    }

    let childGen = if birthYear < 1990 {
        "Ok boomer"
    } else if birthYear > 1990 && birthYear < 2000 {
        "Millenial"
    } else {
        "Gen Z"
    };

    println!("childGen {:?}", childGen);
}
