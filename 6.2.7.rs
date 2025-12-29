use std::io::stdin;
use std::io;

fn main() {
    let auth_capital:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");     
    let percent:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");     
    let fee_rustam:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");       
    let share_soltan:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let profit:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   

    let share_timur:f64 = 1.0 - (((percent as f64)/100.0) + ((fee_rustam as f64)/auth_capital) + share_soltan);
    let profit_timur = profit * share_timur;

    println!("От прибыли {} рублей Тимуру причитается {:.3} рублей", profit, profit_timur);

    let mut in_str1 = String::new();
    io::stdin().read_line(&mut in_str1).expect("Ошибка при чтении строки"); // Обработка ошибки
    let size:f64 = in_str1.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга

    let mut in_str2 = String::new();
    io::stdin().read_line(&mut in_str2).expect("Ошибка при чтении строки"); // Обработка ошибки
    let percent:u32 = in_str2.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
   
    let mut in_str3 = String::new();
    io::stdin().read_line(&mut in_str3).expect("Ошибка при чтении строки"); // Обработка ошибки
    let add_size:f64 = in_str3.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга

    let incr_vol:f64 = (size+add_size )/size;
    
    let new_per:f64 = (percent as f64)/incr_vol;
    println!("Концентрация получившегося раствора: {:.3}%", new_per);

    let mass_raisin:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let grape_water:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let raisin_water:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  

    let mass_grape:f64 = ((100.0-raisin_water)/100.0*mass_raisin)/((100.0-grape_water)/100.0);
    println!("Для получения {:.3} кг изюма необходимо {:.3} кг винограда",  mass_raisin, mass_grape);

    let length:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let first_lenght_day:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let day:u16 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 

    let last_lenght_day:f64 = ((2.0 * length)/(day as f64)) - first_lenght_day;
    println!("В последний день рабочие проложили {:.3} метр(ов)", last_lenght_day);

    let first_lenght_day:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let day:u16 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let numbers_day:u16 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let length:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   

    let diff_lenght:f64 = ((2.0*length)-(2.0*first_lenght_day*(numbers_day as f64))) / (((numbers_day - 1)*numbers_day) as f64);
    let m_lenght_day:f64 = first_lenght_day + (((day - 1) as f64)*diff_lenght);

     println!("За {} день турист прошел {:.3} км", day, m_lenght_day);
}