use std::io;
fn nhap(msg: &str) -> f64 {
    let mut input = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main() {
    let a = nhap("Nhap a : ");
    let b = nhap("nhap b : ");
    let c = nhap("nhap c : ");
    let mut max = a;
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }
    println!("SO LON NHAT LA : {}", max);
}
