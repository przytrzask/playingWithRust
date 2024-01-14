use std::collections::HashMap;

struct Person<'a> {
    name: &'a str,
    location: &'a str,
}

// struct Person<'a> {
//     name: String,
//     location: &'a String,
// }

fn main() {
    let airport_codes = HashMap::from([
        ("WAW", "Warsaw"),
        ("WMI", "Modlin"),
        ("PMI", "Palma de Mallorca"),
    ]);

    let tomek = Person {
        name: "Tomek",
        location: "WMI",
    };

    fn get_airport_name<'a>(code: &'a str, airport_codes: &'a HashMap<&str, &str>) -> &'a str {
        airport_codes
            .get(code)
            .expect("No airport found for that code.")
    }

    println!(
        "Welcome, {} from {}!",
        tomek.name,
        get_airport_name(&tomek.location, &airport_codes)
    );
}

// fn main() {
//     let london = String::from("London");
//     let me = Person {
//         name: String::from("Sid"),
//         location: &london,
//     };

//     let jon = Person {
//         name: String::from("Jon"),
//         location: &london,
//     };

//     println!("Hello, {} from {}!", me.name, me.location);
//     println!("Hello, {} from {}!", jon.name, jon.location);
// }
