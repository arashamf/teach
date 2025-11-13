
use std::io;

const KM_MILE: f64 = 0.621371;
const KM_YARD: f64 = 1093.61;
const KM_FOOT: f64 = 3280.84;
const KM_INCH: f64 = 39370.1;

const SEC_IN_HOUR: u64 = 3600;
const SEC_IN_MIN: u64 = 60;

fn main() {
    let numb: u32;
    let mut input_string1 = String::new();
    io::stdin().read_line(&mut input_string1).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string1.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    if numb >9 && numb < 100{
        println!("{}", numb/10);
        println!("{}", numb%10);
    }

    let numb2: f64;
    let mut input_string2 = String::new();
    io::stdin().read_line(&mut input_string2).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb2 = input_string2.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let mut fractional_part: f64 = numb2 % 1.0; //десятичная часть
    fractional_part = (fractional_part*(1000 as f64)).round(); //0.1236->124
    fractional_part =  fractional_part/(1000 as f64); //124->0.124
    println!("{}", fractional_part);

    let numb3: i32;
    let mut in_str1 = String::new();
    io::stdin().read_line(&mut in_str1).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb3 = in_str1.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let one_pl: i32 = numb3 % 10; //1 разряд
    let tens_pl: i32 = (numb3 % 100)/10; //2 разряд
    let hund_pl: i32 = numb3 /100; //3 разряд
    println!("{}", one_pl+tens_pl+hund_pl);

    let numb4: i32;
    let mut in_str2 = String::new();
    io::stdin().read_line(&mut in_str2).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb4 = in_str2.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let one_pl2: i32 = numb4 % 10; //1 разряд
    let tens_pl2: i32 = (numb4 % 100)/10; //2 разряд
    let hund_pl2: i32 = (numb4 % 1000)/100; //3 разряд
    let thous_pl2: i32 = numb4 /1000; //4 разряд
    println!("Последняя цифра: {}", one_pl2);
    println!("Третья цифра: {}", tens_pl2);
    println!("Вторая цифра: {}", hund_pl2);
    println!("Первая цифра: {}", thous_pl2);

    let distance: f64;
    let mut in_str3 = String::new();
    io::stdin().read_line(&mut in_str3).expect("Ошибка при чтении строки"); // Обработка ошибки
    distance = in_str3.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    println!("{} км = {:.3} миль", distance, distance*KM_MILE);
    println!("{} км = {:.3} ярдов", distance, distance*KM_YARD);
    println!("{} км = {:.3} футов", distance, distance*KM_FOOT);
    println!("{} км = {:.3} дюймов", distance, distance*KM_INCH);

    let time: u64;
    let mut in_str4 = String::new();
    io::stdin().read_line(&mut in_str4).expect("Ошибка при чтении строки"); // Обработка ошибки
    time = in_str4.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let hour: u64 = time/SEC_IN_HOUR;
    let min: u64 = (time - hour*SEC_IN_HOUR)/SEC_IN_MIN;
    let sec: u64 = time - hour*SEC_IN_HOUR - min*SEC_IN_MIN;
    println!("{} сек = {} час {} минут {} секунд", time, hour, min, sec);
}