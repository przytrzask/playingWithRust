mod shapes;

use crate::shapes::{area::Area, circle::Circle, rect::Rect};

// derives debug

// fn main() {
//     let list: Vec<_> = vec![1, 2, 3].iter().map(|x| x + 1).collect();

//     println!("list {:?}", list)
// }

// fn main() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     file.lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_id, line)| println!("{}", line))
// or
// .for_each(|line| println!("{}", line.1)) (enumerate creates tuple)
// }

// ## ENUMS
// enum Colour {
//     Red,
//     Green,
//     Blue,
// }

// impl Colour {
//     fn is_green(&self) -> bool {
//         if let Colour::Green = self {
//             return true;
//         }
//         return false;
//     }

//     fn is_green_parts(&self) -> bool {
//         match self {
//             Colour::Red => false,
//             Colour::Green => true,
//             Colour::Blue => true,
//         }
//     }
// }

// fn print_color(colour: Colour) {
//     match colour {
//         Colour::Red => {
//             println!("{}", "Red")
//         }

//         Colour::Green => {
//             println!("{}", "Green")
//         }

//         Colour::Blue => {
//             println!("{}", "Blue")
//         }
//     }
// }

// fn main() {
//     print_color(Colour::Red);

//     let colour = Colour::Red;

//     println!("{}", colour.is_green_parts());
// }

// create a method append to take in a list of Items and push in the string "Hello Fem!"

// create an Items array (doesn't matter if its empty or not)

// pass it to append

// create a Vec<usize>

// try to pass it to append

// struct Custom {
//     age: usize,
//     name: String,
// }

// enum Item {
//     Number(usize),
//     String(String),
//     MyCustom(Custom),
// }

// fn append(items: &mut Vec<Item>) {
//     items.push(Item::String("some sring".to_string()));
// }

// fn main() {
//     let mut items: Vec<Item> = vec![];
//     let mut numbers: Vec<usize> = vec![];

//     append(&mut items);
//     append(&mut numbers);
// }

// Options
//
// fn main() {
//     fn multiply(value: Option<usize>) -> usize {
//         match value {
//             Option::None => 0,
//             Option::Some(num) => num * 5,
//         }
//     }

//     // create uzise value of 5
//     let five = Option::Some(5);

//     let result = multiply(five);
//     let result2 = multiply(Option::None);

//     println!("result is {:?}", result);
//     println!("result2 is {:?}", result2);
//     println!("result2 is {:?}", result2);
// }

// fn practice(list: Vec<usize>, idx: usize) -> usize {
//     return list.index(idx) * 5;
// }

// fn main() {
//     let file_name = std::env::args().nth(1).expect("File name should be passed");

//     let file = std::fs::read_to_string(file_name);

//     match file {
//         Ok(x) => {
//             return x.lines().enumerate().for_each(|x| println!("{}", x.1));
//         }
//         Err(e) => {
//             println!("{}", e);
//         }
//     }
// }

#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    // return item.count.add_assign(1);
    item.count += 1;
}

fn print_all_items(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item)
    }
}

// fn main() {
//     let mut item = Item { count: 1 };
//     println!("{:?}", item);
//     add_one(&mut item);
//     println!("{:?}", item);
// }
//
// fn main() {
//     let mut items = vec![Item { count: 1 }];
//     let item = items.get_mut(0).unwrap();
//     println!("{:?}", item);
//     print_all_items(&items);
//     // println!("{:?}", item);
// }

// fn main() {
//     let items = vec![1, 2, 3];
//     let sum = items.iter().map(|x| x + 1);

//     println!("{:?}", items);
// }

fn main() {
    let circle = Circle {
        x: 0f64,
        y: 0f64,
        radius: 4f64,
    };

    let rect = Rect::default();

    let mappedRect = rect:Re

    println!("rect area: {}", rect.area());
    println!("circle area: {}", circle.area());
    println!("f64 area: {}", 6.9.area());
    println!("rect : {}", rect);

    // let rec = Circle {
    //     x:f64(1),
    //     y:f64(2),
    //     radius:f64(4),
    // }
}
