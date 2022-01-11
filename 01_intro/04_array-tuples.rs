fn main() {
    let tuple = ("ok", true, 2);
    let (_, total, ..) = tuple;

    println!("{}", total);
}
