use std::io::stdin;

fn main() {
    let mut number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let radix:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
   /*  println!("x до установки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    number |= 1 << radix;
    println!("");
    println!("x после установки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);*/

    println!("x до очистки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    number &= !(1 << radix);
    println!("");
    println!("x после очистки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
}   