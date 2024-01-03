fn sum(x: u64, y: u64) -> u64 {
    x + y
}

fn main() {
    let x = 10;
    let y = 20;

    let result = sum(x, y);
    let anotherSum = |x, y| x + y;
    let secondResult = anotherSum(20, 20);

    println!("{:?}  {:?}", result, secondResult)
}
