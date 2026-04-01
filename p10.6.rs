use std::io::stdin;
use std::io;
fn main() {
    let mut number:f64 =0.0;
    loop {
        number = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
        if number <= 0.0 {  continue;  }
        else {  
            println!("{number}");
            break;  
        }
    }
    let number:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let mut tmp:u32 = number;
    let mut handle:usize = 0;
    let mut prev:u32;
    let mut flag:bool = true;
    let mut series = vec![0; 15];
    while tmp > 0 {
        series[handle] = tmp % 10;
        tmp /= 10;
        handle += 1;
    }
    prev = series[0];
    for count in 1..handle {
        if prev <= series[count] {
            flag = false;
            break;
        }
        else {  prev = series[count]; }
    }

    if flag == true {    println!("Цифры числа {} расположены по возрастанию", number);      }
    else            {    println!("Цифры числа {} не расположены по возрастанию", number);   }

    let mut number:i32 = 1;
    let mut sum_plus = 0;
    let mut sum_minus = 0;

    while number != 0  { 
        number = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
        if number >0 {  sum_plus += number; }
        else { sum_minus += number; }
    }
    println!("Сумма отрицательных чисел: {}", sum_minus);
    println!("Сумма положительных чисел: {}", sum_plus);
    let mut input = String::new();
    let divizor: i32 = loop {
        input.clear();
        stdin().read_line(&mut input).expect("Ошибка ввода");
        match input.trim().parse() {
            Ok(num) => break num,
            _ => continue,
        }
    }; 
    let size: usize = loop {
        input.clear();
        stdin().read_line(&mut input).expect("Ошибка ввода");
        match input.trim().parse() {
            Ok(num) if num > 0 => break num,
            _ => continue,
        }
    };
    let mut series = vec![0; size];
    for count in 0..size {
        series[count] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    }
    for count in 0..size {
        if  series[count] % divizor == 0 {   println!("{}",series[count]);   }
        else {  continue;   }
    }
    let mut command = String::new();
    command.clear();
    stdin().read_line(&mut command).expect("Ошибка ввода");
    let mut input = String::new();
    let mut temp:u32;
    
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Ошибка при чтении ввода.");
        if input.trim() != command.trim() {
            temp = match input.trim().parse() {
                Ok(num)  => num,
                Err(_) => continue,
            };
            if temp > 90 {println!("Термостат открыт");}
            else         {println!("Термостат закрыт");}
        }
        else {break;}
    }

    let mut input = String::new();
    let mut passwd= String::new();
    let size:u32;
    let mut count = 0;

    passwd.clear();
    stdin().read_line(&mut passwd).expect("Ошибка при чтении ввода.");
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Ошибка при чтении ввода.");
        size = match input.trim().parse() {
            Ok(num)  => num ,
            Err(_) => continue,
        };
        break;
    }
    loop { 
        input.clear();
        stdin().read_line(&mut input).expect("Ошибка при чтении ввода.");
        if input.trim() == passwd.trim() {
            println!("Доступ предоставлен");
            break;
        }
        else {
            count += 1;
            println!("Неверный пароль");
            if count >= size {         
                println!("Слишком много попыток, пожалуйста повторите позже");
                break;
            }
        }
    }      
} 
