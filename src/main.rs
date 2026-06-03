use std::io;

// BÀI 1: IN RA SỐ LỚN NHẤT VỚI a,b,c NHẬP TỪ BÀN PHÍM
// fn nhap(msg: &str) -> f64 {
//     let mut input = String::new();
//     println!("{}",msg);
//     io::stdin().read_line(&mut input).unwrap();
//     input.trim().parse().unwrap()
// }

// fn main() {
//     let a = nhap ("Nhap a : ");
//     let b = nhap("nhap b : ");
//     let c = nhap("nhap c : ");
//     let mut max = a;
//     if b > max {
//         max = b;
//     }
//     if c > max  {
//         max = c;
//     }
//     println!("SO LON NHAT LA : {}", max);
// }

//BÀI 2 : GIẢI PHƯƠNG TRÌNH BẬC HAI Ax^2 + Bx + C = 0
// fn nhap_gia_tri(msg: &str) -> f64 {
//     let mut s = String::new();
//     println!("{}", msg);
//     io::stdin().read_line(&mut s).unwrap();
//     s.trim().parse().unwrap()
// }
// fn main() {
//     let a = nhap_gia_tri("NHAP a : ");
//     let b = nhap_gia_tri("NHAP b : ");
//     let c = nhap_gia_tri("NHAP c : ");

//     if a == 0.0 {
//         if b == 0.0 {
//             if c == 0.0 {
//                 println!("PHUONG TRINH CO VO SO NGHIEM");
//             } else {println!("PHUONG TRINH VO NGHIEM");}
//         } else { println!("PHUONG TRINH CO NGHIEM LA : {}", -1.0*c/b);}
//     } else {
//         let delta = b*b - 4.0*a*c;
//         if delta < 0.0 {
//             println!("PHUONG TRINH VO NGHIEM");
//         }
//         else if delta == 0.0 {
//             println!("PHUONG TRINH CO NGHIEM KEP LA : {}", -b/(2.0*a))
//         }
//         else {
//             let x1 = (-b + delta.sqrt()) / (2.0*a);
//             let x2 = (-b - delta.sqrt()) / (2.0*a);
//             println!("PHUONG TRINH CO HAI NGHIEM PHAN BIET LA ");
//             println!("X1 = {}",x1);
//             println!("X2 = {}",x2);
//         }
//     }
// }

//
//CÁCH 2 : VIẾT THEO KIỂU CLASS

struct HinhChuNhat {
    dai: f64,
    rong: f64,
}

impl HinhChuNhat {
    fn input() -> Self {
        let dai = nhap("NHAP CHIEU DAI : ");
        let rong = nhap("NHAP CHIEU RONG : ");
        Self { dai, rong }
    }

    fn tinh_dien_tich(&self) -> f64 {
        self.dai * self.rong
    }

    fn xuat(&self) {
        println!("DIEN TICH HINH CHU NHAT LA : {}", self.tinh_dien_tich());
    }
}

fn nhap(msg: &str) -> f64 {
    let mut s = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let h = HinhChuNhat::input();
    h.xuat();
}
