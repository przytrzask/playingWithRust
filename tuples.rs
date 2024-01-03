// arrays
fn main() {
    let tuple = ("Iphone 12", 1299, "black");

    let (price, ..) = tuple;

    println!("tuple debug {:?} {:?}", tuple, price)
}
