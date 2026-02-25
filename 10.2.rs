use std::io::stdin;
use std::io;
fn main() {
    let mut in_str = String::new();
    for _ in 0..10 {

        stdin().read_line(&mut in_str).expect("");
        println!("Вы ввели: {}", in_str.trim());
        in_str.clear();
    }
    let numb:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut in_str = String::new();
    let mut sum:f64 = 0.0;
    let mut buf:f64 = 0.0;
    for _ in 0..numb {
        stdin().read_line(&mut in_str).expect("Ошибка при чтении строки");
        buf = in_str.trim().parse().expect("Ошибка преобразования типа"); 
        in_str.clear();
        sum += buf;
    }
    println!("Сумма считанных чисел равна: {:.1}", sum);
    let numb:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut curr:i32 = 0;
    let mut last:i32 = 0;
    let mut in_str = String::new();
    for count in 0..numb {
        stdin().read_line(&mut in_str).expect("Ошибка при чтении строки");
        curr = in_str.trim().parse().expect("Ошибка преобразования типа");
        in_str.clear();
        if count > 0 {
            if curr > last  {   println!("{curr} > {last}");    }
            else {
                if curr == last  {   println!("{curr} == {last}");    }
                else    {
                    if curr < last  {   println!("{curr} < {last}");    }
                }
            }
        }
        (curr, last) = (last, curr);         
    }
    let start:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let end:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let step:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut div:f64 = 1.0;
    for count in (start..=end).step_by(step) {
        div *= count as f64;
    }
    println!("{:.1}", div);
    for count in (start..=end).step_by(step) {
        if count%2 == 0 {
            println!("{}",count);
        }
    }    
    for count in (start..=end).rev() {
        println!("{}",count);
    } 
}  