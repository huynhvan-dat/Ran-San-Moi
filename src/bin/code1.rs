// BÀI 2 : GIẢI PHƯƠNG TRÌNH BẬC HAI Ax^2 + Bx + C = 0

use std::io;
fn nhap_gia_tri(msg: &str) -> f64 {
    let mut s = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
fn main() {
    let a = nhap_gia_tri("NHAP a : ");
    let b = nhap_gia_tri("NHAP b : ");
    let c = nhap_gia_tri("NHAP c : ");

    if a == 0.0 {
        if b == 0.0 {
            if c == 0.0 {
                println!("PHUONG TRINH CO VO SO NGHIEM");
            } else {
                println!("PHUONG TRINH VO NGHIEM");
            }
        } else {
            println!("PHUONG TRINH CO NGHIEM LA : {}", -1.0 * c / b);
        }
    } else {
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            println!("PHUONG TRINH VO NGHIEM");
        } else if delta == 0.0 {
            println!("PHUONG TRINH CO NGHIEM KEP LA : {}", -b / (2.0 * a))
        } else {
            let x1 = (-b + delta.sqrt()) / (2.0 * a);
            let x2 = (-b - delta.sqrt()) / (2.0 * a);
            println!("PHUONG TRINH CO HAI NGHIEM PHAN BIET LA ");
            println!("X1 = {}", x1);
            println!("X2 = {}", x2);
        }
    }
}
