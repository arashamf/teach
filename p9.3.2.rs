use std::io::stdin;

fn main() {
    let number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut radix:u8 = 0;
    if  number != 0  {
        for count in 0..8   {
            if number & (0x01  << count) != 0   {
                radix = count;
                break;
            }
        }
        println!("в числе {:08b} позиция младшего установленного бита равна {}", number, radix);
    }
    else {  println!("у числа {:08b} нет установленных бит", number);   }
    if  number != 0  {
        for count in 0..8   {
            if number & (-128  >> count) != 0   {
                radix = 7-count;
                break;
            }
        }
        println!("в числе {:08b} позиция старшего установленного бита равна {}", number, radix);
    }
    else {  println!("у числа {:08b} нет установленных бит", number);   }
    let mut number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let count:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    println!("x до очистки от 0 до {}-го бита", count-1);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    println!("");
    if (count > 0) && (count < 8)  {
        match count {
            1 => number &= !(0x01),
            2 => number &= !(0x03),
            3 => number &= !(0x07),
            4 => number &= !(0x0F),
            5 => number &= !(0x1F),
            6 => number &= !(0x3F),
            7 => number &= !(0x7F),
            8 => number &= 0x00,
            _ => panic!(),
        }
    }
    println!("x после очистки от 0 до {}-го бита", count-1);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);

    println!("x до очистки от 7 до {}-го бита", count);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);
    println!("");
    number &= ((1u8 << count) - 1u8) as i8;
    println!("x после очистки от 7 до {}-го бита", count);
    println!("в двоичной записи: {:08b}", number);
    println!("в десятичной записи: {}", number);

    let  number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if number & !(0xFEu8 as i8) > 0 
    {     println!("число {} является нечетным", number);   }
    else 
    {   println!("число {} является четным", number);   }
    
    let number:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if (number > 0) && (number & !number == 0) {
        println!("число {} является степенью 2",number);
    }
    else {
        println!("число {} не является степенью 2",number);
    }

    let numb1:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb2:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if (numb1 ^ numb2) >= 0 {
        println!("числа {} и {} имеют одинаковые знаки",numb1,numb2);
    }
    else {
        println!("числа {} и {} имеют разные знаки",numb1,numb2);
    }
}   