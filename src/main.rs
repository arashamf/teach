use std::io::stdin;
use std::io;
fn main() {
    /*let mut number:f64 =0.0;
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
    else            {    println!("Цифры числа {} не расположены по возрастанию", number);   }*/

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
} 