use std::io::stdin;
use std::io;

fn main() {

   let numb_month:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let year = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if numb_month < 13  && numb_month > 0
    {  println!("{}", year[numb_month-1]);  }

    let months = ["Январь", "Февраль", "Март", "Апрель", "Май", "Июнь", "Июль", "Август", 
                            "Сентябрь", "Октябрь", "Ноябрь", "Декабрь"];
    let season = ["Зима", "Весна", "Лето", "Осень"];
   // println!("{}", season[numb_month % 12 / 3])
     match numb_month {
        1 | 2 | 12 => println!("{}",season[0]),
        3 | 4 | 5 => println!("{}",season[1]),
        6 | 7 | 8 => println!("{}",season[2]),
        9 | 10 | 11 => println!("{}",season[3]),
        _ => println!("Недопустимое значение"),
    }     
    let tup = (1, 3.14, -12.3, -50, 100, 250, -4, 7.6);
    let (number1, number2, number3, number4, number5, number6, number7, number8) = tup;
    let mut sum: f64 = 0.0;
    if (number1 as f64) > 0.0 { sum += number1 as f64; }
    if (number2 as f64) > 0.0 { sum += number2 as f64; }
    if (number3 as f64) > 0.0 { sum += number3 as f64; }
    if (number4 as f64) > 0.0 { sum += number4 as f64; }
    if (number5 as f64) > 0.0 { sum += number5 as f64; }
    if (number6 as f64) > 0.0 { sum += number6 as f64; }
    if (number7 as f64) > 0.0 { sum += number7 as f64; }
    if (number8 as f64) > 0.0 { sum += number8 as f64; }
    println!("{}", sum);
    let array = [3, 1, 0, -5, -1, 300, 4, 2];
    let mut max:i32 = array[0];
    let mut min:i32 = array[0];
    for counter in array {
        if max < counter {  max = counter;  }
        if min > counter {  min = counter;  }
    }
    let min_index:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let max_index:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if min == array[min_index] { println!("Считанный мин.индекс корректный"); }
    else {  println!("Считанный мин.индекс некорректный");    }
    if max == array[max_index] { println!("Считанный макс.индекс корректный"); }
    else {   println!("Считанный макс.индекс некорректный");   }
    let mut array = [5, -2, 8, -1, 3];
    for i in 0.. 4 {
        for j in (i + 1).. 5 {
            if array[i] > array[j]  { 
                (array[i], array[j]) = (array[j], array[i]);  
            }
        }
    }
    println!("{:?}",array);
}