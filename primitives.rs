// primitives
fn main() {
    let price = 129;
    let tax = 23.22;
    // let total = price as f64 + tax;

    // cast to float64 in rust
    // let total = price as f64 + tax;

    let total = f64::from(price) + tax;
    println!("Total: {} + {} = {}", price, tax, total)
}
