use std::io::stdin;

fn main() {
    let mut number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let radix:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    println!("x до установки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    number |= 1 << radix;
    println!("");
    println!("x после установки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);

    println!("x до очистки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    number &= !(1 << radix);
    println!("");
    println!("x после очистки {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);

    println!("x до переключения {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    if (number & (1 << radix)) != 0 {   number &= !(1 << radix); }
    else {  number |= 1 << radix; }
    println!("");
    println!("x после переключения {}-го бита", radix);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);

    let number:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let radix:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    println!("{}-й бит числа {:08b} равен {}", radix, number, (number&(1<<radix))>>radix );

    let mut number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut radix:u8 = 0;
    if  number != 0  {
        for count in 0..8   {
            if number & (-128  >> count) != 0 {
                radix = count;
                break;
            }
        }
        println!("x до очистки старшего установленного бита");
        println!("в двоичной записи: {:08b}", number);
        println!("в десятичной записи: {}", number);
        println!("");  
        number &= !(-128 >> radix);  
        println!("x после очистки старшего установленного бита");
        println!("в двоичной записи: {:08b}", number);
        println!("в десятичной записи: {}", number);
    }
    else {  println!("у числа {:08b} нет установленных бит", number);   }

    let number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut radix:i8 = 0;
    if  number != 0  {
        for count in 0..8   {
            if number & (0x01 << count) != 0 {
                radix |= 1<<count;
                break;
            }
        }
        println!("извлеченный из {} младший установленный бит", number);
        println!("в двоичной записи: {:08b}", radix);
        println!("в десятичной записи: {}", radix);
        println!("");  
    }
    else {  println!("у числа {:08b} нет установленных бит", number);   }
    let mut shift:i8 = 0;
    if  number != 0  {
        for count in 0..8   {
            if number & (-128 >> count) != 0 {
                radix = 7 - count;
                shift |= 1<<radix;
                break;
            }
        }
        println!("извлеченный из {} старший установленный бит", number);
        println!("в двоичной записи: {:08b}", shift);
        println!("в десятичной записи: {}", shift);
        println!("");  
    }
    else {  println!("у числа {:08b} нет установленных бит", number);   }
}   