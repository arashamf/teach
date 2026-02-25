use std::io::stdin;
use std::io;
fn main() {
    let mut input = String::new();
    let mut stop_word = String::new();
    stdin().read_line(&mut stop_word).expect("Ошибка при считывании");
    let mut numb: f64;
    let mut sum: f64 = 0.0;
    let mut flag:bool = true;

    while flag {
        stdin().read_line(&mut input).expect("Ошибка при считывании");
        if  stop_word.trim() == input.trim() {   flag = false;  }
        else    {
            numb = input.trim().parse().expect("Введенное значение не является числом"); 
            sum += numb;
        }
        println!("{}", input.trim());
        input.clear();
    }
    println!("{:.1}", sum);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Ошибка при считывании");
    let mut counter:u32 = 0;
    print! ("Число {} состоит из цифр:", input.trim());
    for ch in input.trim().chars() {
        if ch.is_numeric() == false {   break;  }
        else {  
            print! (" {}", ch);
            counter += 1;   
        }
    }
    print! ("\n");
    println! ("Число {} является {counter} значным", input.trim());

    let iter:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut counter:u8 = 0;
    for count in 0..8  {
        if (iter >> count) & 0x01 == 0x01 { counter+=1; }
    }
    println! ("Количество установленных битов в {:08b} равно {counter}",iter);

    let number:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut buf:u32 = number;
    let mut flag:bool = true;
    let mut cnt:u8 = 0;
    let jun_div:u32 = 10;
    let mut sen_div:u32 = 1;

    while buf != 0  { 
        buf /= 10;
        cnt += 1;  
        sen_div *= 10;
    }
    sen_div /= 10;
    buf = number;

    for _ in 0..cnt/2  {
        if (buf/sen_div) != (buf%jun_div) {
            flag = false;
            break
        }
        else { 
            buf %= sen_div; buf /= jun_div;
            sen_div /= 100;          
        }
    }

    if flag == true {   println!("Число {} является палиндромом", number);  }
    else            {   println! ("Число {} не является палиндромом", number); }

    let mut prev:u32;
    let mut curr :u32;
    if buf / 10 > 0 {
        prev = buf % 10;
        buf /= 10;
        while buf != 0  { 
            curr = buf%10;
            if curr != prev {               
                flag = false;
                break;
            }
            buf /= 10;  
            prev = curr;
        }
    }
    if  flag == true    {   println!("Все цифры числа {number} равны");  }
    else                {   println!("Цифры числа {number} неодинаковые");   }
    let log:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    const EXPONENT:u32 = 2;
    let mut mult = EXPONENT;
    let mut step = 0;
    while mult <= log {
        mult *= EXPONENT;
        step += 1;
    }
    println!("{step}");

    let log:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    const EXPONENT:u32 = 2;
    let mut mult = EXPONENT;
    let mut step = 0;
    while mult <= log {
        mult *= EXPONENT;
        step += 1;
    }
    println!("{step}");

    let mut series = vec![false; 9];
    let mut input = String::new();
    let mut numb:usize = 0;
    let mut count = 0; 
0;
   loop   {
        stdin().read_line(&mut input).expect("Ошибка при считывании");
        numb = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        input.clear();
        if (numb < 10) && (numb > 0) {  
            series[numb-1] = true; 
            count += 1;
        }        
    }
    for i in 0..=count {
        if series[i] == false {
            println!("Пропущено число {}",i+1);
        }
    }
    let number:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let step:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut res:u32 = number;
    let mut div:u32 = 1;
    let mut cnt:u32 = 0;   
    while res > 0 {
          res  /= 10;
          cnt +=1;
    }
    if cnt > step {   
        for _ in 0..(cnt-step) {
            div*=10;
        }
        println!("{}", number%div);
    }
    else {  println!("k >= n");  }
} 