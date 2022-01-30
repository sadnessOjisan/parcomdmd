fn main() {
    let orig = String::from("1");
    let (f, s) = orig.split_at(1);
    println!("{}/{}", f, s);
}
