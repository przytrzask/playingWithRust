fn main() {
    // basic loop
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("count is {:?}", count);
    //     if count == 10 {
    //         break;
    //     }
    // }

    // expression loop
    let mut starting_age = 0;
    let age = loop {
        starting_age += 1;
        if starting_age == 10 {
            break starting_age;
        }
    };
    println!("age is {:?}", age);

    // while loop
    // let mut counter = 0;
    // while counter < 10 {
    //     println!("counter is {:?}", counter);
    //     counter += 1;
    // }

    // for in loop

    // for number in 1..10 {
    for number in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        println!("number is {:?}", number);
    }
}
