// fn main() {
//     let y = "Hiya universe";

//     let hiya = y.split(" ").nth(0).unwrap();

//     println!("{}", hiya);
// }

fn main() {
    let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let y: Vec<i32> = x.iter().copied().filter(|a| a % 2 == 0).collect();

    let z = &y[0..3];

    println!("{:?}", x);

    println!("{:?}", y);

    println!("{:?}", z);
}
