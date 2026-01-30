use std::io::stdin;
//use std::io;

fn main() {
    let mut hours:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut over_work_hours = 0;
    const RATE:u32 = 1500;
    const NORMAL_WORK_HOURS:u8 = 40;
    if hours > NORMAL_WORK_HOURS    {
        over_work_hours = hours - NORMAL_WORK_HOURS;
        hours = NORMAL_WORK_HOURS;
    }
    let dirty_pay:f64 = (hours as f64) * (RATE as f64) + ((over_work_hours as f64)*(RATE as f64) * 1.5 );
    let clean_pay:f64 = dirty_pay * 0.87;
    let tax:f64 = dirty_pay - clean_pay;

    println!("Заработная плата до вычетов: {:.2} руб", dirty_pay);
    println!("Сумма налогов: {:.2} руб", tax);
    println!("Заработная плата после вычетов: {:.2} руб", clean_pay);

    let number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut senior_limit: i32 = 0;
    let mut jun_limit: i32 = 0;
    if number1 > number2 {
        senior_limit = number1;
        jun_limit = number2;
    }
    else {
        senior_limit = number2;
        jun_limit= number1;
    }
    if (number >= jun_limit) && (number <= senior_limit)
    {   println!("Значение {} принадлежит отрезку [{}, {}]", number, jun_limit, senior_limit); }
    else 
    {   println!("Значение {} не принадлежит отрезку [{}, {}]", number, jun_limit, senior_limit); }

    let number:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number1:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number2:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut senior_limit: f64 = 0.0;
    let mut jun_limit: f64 = 0.0;
    if number1 > number2 {
        senior_limit = number1;
        jun_limit = number2;
    }
    else {
        senior_limit = number2;
        jun_limit= number1;
    }
    if (number > jun_limit) && (number < senior_limit)
    {   println!("Значение {:.1} принадлежит интервалу ({:.1}, {:.1})", number, jun_limit, senior_limit); }
    else 
    {   println!("Значение {:.1} не принадлежит интервалу ({:.1}, {:.1})", number, jun_limit, senior_limit); }

    let coord_X_p1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let coord_Y_p1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let coord_X_p2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let coord_Y_p2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let coord_X_p3:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let coord_Y_p3:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    let vector1:[i32; 2] = [coord_X_p2-coord_X_p1, coord_Y_p2-coord_Y_p1];
    let vector2:[i32; 2] = [coord_X_p3-coord_X_p2, coord_Y_p3-coord_Y_p2];


    if vector2[0] != 0 && vector2[1] != 0 {
        if ((vector1[0]/vector2[0]) == (vector1[1]/vector2[1])) && ((vector1[0]%vector2[0]) == (vector1[1]%vector2[1]))
        {   println!("Точки коллинеарны");   }
        else    {   println!("Точки не коллинеарны");   }
    }
    else    {   
        if  (vector1[0] == vector2[0]) && (vector1[1] == vector2[1])
        {   println!("Точки коллинеарны ");  }
        else    {  println!("Точки не коллинеарны"); }
    }
    let point_x:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let point_y:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if point_x > 0.0 {
        if point_y > 0.0
        {   println!("I - четверть");   }
        else 
        {   println!("IV - четверть"); }    
    }
    else {
        if point_y > 0.0
        {   println!("II - четверть");   }
        else 
        {   println!("III - четверть"); }  
    }
    let a:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let b:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let c:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if (a == b) && (b == c) {
      println!("Числа {}, {} и {} образуют равносторонний треугольник", a, b , c);  
    }
    else {
        if (a + b > c) && (a + c > b) && (b + c > a) {
            if (a == b) || (a == c) || (b == c) {
                println!("Числа {}, {} и {} образуют равнобедренный треугольник", a, b , c);
            }
            else {
                println!("Числа {}, {} и {} образуют разносторонний треугольник", a, b , c);
            }
        }
        else {
            println!("Числа {}, {} и {} не образуют треугольник", a, b , c);
        }
    }
    let n_series:i64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut sum: i64 = 0;
    for count in 1..n_series+1 {
        if count % 2 != 0 {
            sum += count;
        }
        else {   sum -= count;   }
    }
    println!("{}", sum);
    let factorial:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut multi:i32 = 1;
    if factorial > 4
    {   println!("Для {}! последняя цифра равна 0", factorial);    }
    else {
        for count in 1..factorial+1
        {   multi *= count; }
        println!("Для {}! последняя цифра равна {}", factorial, multi%10);
    }
}   