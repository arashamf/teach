
use std::io;

fn main() {
    let safe = ('🍏','🪙','🍐','🪙','🍊','🪙','🍓','🪙','🍒','🪙');

    println!("{}, {}, {}, {}, {}", safe.0, safe.2, safe.4, safe.6, safe.8);

        let tup = (
        "Воскресенье",
        "Понедельник",
        "Вторник",
        "Среда",
        "Четверг",
        "Пятница",
        "Суббота",
    );

    println!("{}, {}, {}", tup.1, tup.3, tup.5);

    /*let mut in_str1 = String::new();
     io::stdin()
        .read_line(&mut in_str1)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим

    let mut in_str2 = String::new();
     io::stdin()
        .read_line(&mut in_str2)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим

    let mut in_str3 = String::new();
     io::stdin()
        .read_line(&mut in_str3)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим

    let mut in_str4 = String::new();
     io::stdin()
        .read_line(&mut in_str4)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим

        let mut in_str5 = String::new();
     io::stdin()
        .read_line(&mut in_str5)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    // Удаляем лишние пробельные символы и переводы строк, затем парсим

     let str_tup = (in_str1, in_str2, in_str3, in_str4, in_str5);
     println!("{:?}",str_tup);*/

    let mut numb: f64;
    let mut input_string1 = String::new();
    io::stdin()
        .read_line(&mut input_string1)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string1.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let a = numb as i32;

    let mut input_string2 = String::new();
         io::stdin()
        .read_line(&mut input_string2)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string2.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let b = numb as i32;

    let mut input_string3 = String::new();
         io::stdin()
        .read_line(&mut input_string3)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string3.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let c = numb as i32;

    let mut input_string4 = String::new();
         io::stdin()
        .read_line(&mut input_string4)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string4.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let d = numb as i32;

    let mut input_string5 = String::new();
         io::stdin()
        .read_line(&mut input_string5)
        .expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string5.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let e = numb as i32;

    let numb_tup = (a, b, c, d, e, 0);
    println!("{}, {}, {}, {}, {}, {}", numb_tup.0, numb_tup.1,  numb_tup.2,  numb_tup.3,  numb_tup.4,  numb_tup.5);

    let tup = ("До", "деструктуризации", 3, 2, 1, 0, "пуск!");
    let (i_str, j_str, a, b, c, d, k_str) = tup; // деструктуризация
     println!("{i_str}, {j_str}, {a}, {b}, {c}, {d}, {k_str}");

    let tup_p = (3, 0.1, 0.04, 0.001, 0.0005, 0.00009, 0.000002, 0.0000006);

    let numb_p:f64;
    numb_p= (tup_p.0 as f64) + tup_p.1 + tup_p.2 + tup_p.3 + tup_p.4 + tup_p.5 + tup_p.6 + tup_p.7;
    println!("{}", numb_p);

}