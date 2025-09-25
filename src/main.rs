
use std::io;

fn main() {
    println!("Введите действительное число");
    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим
    let a: f64 = input_string
        .trim() // Удаляем пробелы
        .parse() // Преобразуем в i32 (Rust сам выведет тип)
        .expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    
    println!("Введите целое число");
    let mut input_string2 = String::new();
    
    io::stdin()
        .read_line(&mut input_string2)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
        
    let b: i32 = input_string2
        .trim() // Удаляем пробелы
        .parse() // Преобразуем в i32 (Rust сам выведет тип)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    
    println!("Введите целое число");
    let mut input_string3 = String::new();
    
    io::stdin()
        .read_line(&mut input_string3)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
        
    let d: f64 = input_string3
        .trim() // Удаляем пробелы
        .parse() // Преобразуем в i32 (Rust сам выведет тип)
        .expect("Ошибка при чтении строки"); // Обработка ошибки

    let c: usize = b as usize; //преоброзование в тип usize

    println!("введёное число {3}, необходим разрядов после запятой {2}, получено {0:.1$}",  a, c, c, a); //количество выводимых цифр после запятой числа a
    println!("сумма {:.1}",  (a + (b as f64)));

    let  integer_part = a as i64;
    let fractional_part = a - integer_part as f64;
   
    println!("Целая часть: {}", integer_part);
    println!("Дробная часть, 3 символа после запятой: {:.3}", fractional_part);

    println!("{0} + ({1}) = {2}", a as i32, d as i32, (a+d) as i32);
    println!("{0} - ({1}) = {2}", a as i32, d as i32, (a-d) as i32);
    println!("{0} * ({1}) = {2}", a as i32, d as i32, (a*d) as i32);
    if (d as i32) != 0
    {
        println!("{0} / ({1}) = {2:.3}", a as i32, d as i32, (a/d) );
    }
    println!("{0} % ({1}) = {2:.3}", a as i32, d as i32, (a%d) );

    println!("{:.E}", a); //запись в экспоненциальной форме

    println!("{0} kg = {1:.3} lbs",  a , (a*2.205)); //запись в экспоненциальной форме
    println!("{0} lbs = {1:.3} kg",  d , (d*0.454)); //запись в экспоненциальной форме
}
