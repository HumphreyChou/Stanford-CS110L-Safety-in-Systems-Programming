fn main() {
    let mut s = String::from("hello");
    let refs = &s;
    s = String::from("world");
    println!("{}", refs.to_uppercase());
}