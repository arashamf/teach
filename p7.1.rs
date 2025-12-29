use std::io::stdin;
//use std::io;

fn main() {
    let float_digital:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    if float_digital > (0 as f64) { println!("Число {:.1} является положительным", float_digital);    }
    else {
        if float_digital < (0 as f64) { println!("Число {:.1} является отрицательным", float_digital);    }
    }
    let float_dig1:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let float_dig2:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let mut max_dig3:f64 = 0.0;
    if float_dig1 > float_dig2 { max_dig3 = float_dig1; }  
    else { 
        if float_dig1 < float_dig2 { max_dig3 = float_dig2; } 
    }  

    println!("Из {:.3} и {:.3} больше {:.3}", float_dig1, float_dig2, max_dig3);    

     let int_dig1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let int_dig2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  

    if int_dig1%2 == 0 { println!("Число {} является четным", int_dig1); }
    else {  println!("Число {} является нечетным", int_dig1); }

    if int_dig2%2 == 0 { println!("Число {} является четным", int_dig2); }
    else {  println!("Число {} является нечетным", int_dig2); }

    let xs= [-2.5, 4.2, 9.1, 22.5, 30.0, 1445.123, 1000000.0, 0.001, 0.5, -0.127];
    let len_arr:usize = xs.len();
    let cell_arr:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if cell_arr < 0 {  println!("Отрицательный индекс приводит к панике");     }
    else {
        if  (cell_arr as usize) >= len_arr {   println!("Попытка выхода за пределы массива");  } 
        else {  println!("Элемент с индексом {} равен {:.3}", cell_arr,  xs[cell_arr as usize]);     }   
    }

    let temp:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    const MIN_CUTOFF: f64 = 24.0;
    const MAX_CUTOFF: f64 = 28.0;
    if temp < MIN_CUTOFF { println!("Температура {:.1}°C ниже нормы {:.1}°C, включаю отопление", temp,  MIN_CUTOFF); }
    else {
        if temp >  MAX_CUTOFF { println!("Температура {:.1}°C выше нормы {:.1}°C, отключаю отопление", temp,  MAX_CUTOFF); }
        else {   println!("Температура {:.1}°C в пределах нормы", temp);    }
    }

    let numb1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb3:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if ((numb1 % 10) == (numb2 % 10)) && ((numb2 % 10) == (numb3 % 10)) 
    {   println!("Числа неразличны"); }
    else {
        if  ((numb1 % 100) == (numb2 % 100)) && ((numb2 % 100) == (numb3 % 100)) 
        {   println!("Числа неразличны"); }
        else 
        {   println!("Числа различны"); }
    }

    let year:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if ((year%4) == 0 && (year%100) != 0) || (year % 400 == 0)   {
             println!("{} является високосным годом", year);
    }   else {  println!("{} не является високосным годом", year); }
}