// Pattern matching

// fn main() {
//     enum Status {
//         Connected,
//         Disconnected,
//         Failure(String),
//     }

//     let lan = Status::Failure(String::from("Could not contact DHCP server"));
//     match lan {
//         Status::Connected => {
//             println!("Connection established")
//         }
//         Status::Disconnected => {
//             println!("Connection lost")
//         }
//         Status::Failure(error) => {
//             println!("Error: {}", error)
//         }
//     }
// }

fn main() {
    enum Status {
        Connected,
        Disconnected,
        Failure(String),
    }

    let status = Status::Failure(String::from("Sever not found"));

    if let Status::Failure(error) = status {
        println!("Error: {}", error)
    }
}
