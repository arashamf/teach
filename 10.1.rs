use std::io::stdin;
use std::io;
fn main() {
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).expect("");
    let number:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    for count in 0..=number   {
        println!("{},{}",input_string.trim(), count);       
    }
    let mut in_str = String::new();
    std::io::stdin().read_line(&mut in_str).expect("");
    let start:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let end:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    for _ in start..end {
        println!("{}", in_str.trim());
    }
    let end:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    for count in 0..end {
        println!("{}", count);
    }

    let mut in_str = String::new();
    std::io::stdin().read_line(&mut in_str).expect("");
    let end:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    for count in 0..end {
        println!("{}: {}", count,in_str.trim());
    }
    let mut start:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut end:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if start > end {    (start, end) = (end, start);    } //Деструктуризация кортежа 
    let mut sum: i32 = 0;
     for count in start..end {
        println!("{} * 2 = {}", count, count<<1);
    }
    for count in start..end {
        sum +=count;
    }
    println!("{}", sum);
     for count in start..=end {
        println!("{}", count);
    }
    for count in start..=end {
        println!("{}", count*count);
    }
    let numb:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut factorial:u32 = 1;
    for count in 1..=numb {
        factorial *= count;
    }
    println!("Факториал числа {} равен: {}", numb, factorial);
    let mut prev_fib:u32 = 0;
    let mut curr_fib:u32 = 1;
    println!("Последовательность Фибоначчи для {} элементов:",numb);
    if numb > 0   {
        for _ in 0..numb {
            println!("{}",prev_fib);
            (prev_fib, curr_fib) = (curr_fib, prev_fib+curr_fib); 
        }
    }
}  