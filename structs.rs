#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Celsius(i16);
struct Temp(i16, i16);

fn main() {
    let name = String::from("Sidhartha");
    let age = 30;

    let me = Person { name, age };

    let boiling_temp = Celsius(100);
    let _freezing_temp = Celsius(0);

    let me_next_year = Person { age: 31, ..me };

    // println!("{:?}", me);
    println!("boiling temp is {:?}", boiling_temp.0);
    println!("me next year {:?}", me_next_year);
}
