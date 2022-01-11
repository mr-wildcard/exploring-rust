/*
fn main() {
    let is_cool: bool = true;
    let initial: char = 'S';
    let age: u64 = 30;
    let net_worth: f32 = 1.5; // In Bitcoin of course
}
*/

fn main() {
    let price = 129;
    let tax = 23.22;
    let total = f64::from(price) + tax;

    println!("{}", total);
}
