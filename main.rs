fn main() {
    let a = 10; // immutable
    let aref1 = &a; // ref
    let aref2 = &a;
    println!("{},{},{}", a, aref1, aref2)
}
