use parcomdmd::control::fmap;
fn main() {
    fmap();
    let orig = String::from("1");
    let (f, s) = orig.split_at(1);
    println!("{}/{}", f, s);
}
