use std::io::stdin;
//use std::io;

fn main() {
   /* let mut number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut buf_numb:i32 = 0;
    let mut div:i32 = 1;
    for n in 0..3 {
            if n < 2 {
                if number < 0   { div = -1;  } 
            }
            else {  div = 1;    }
            buf_numb += (number % 10)*div;
            number = number/10;
        } 
    println!("{}",buf_numb);

    let div:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb3:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if (numb1%div == 0) && (numb2%div == 0) && (numb3%div == 0) {   println!("{} является делителем чисел {}, {}, {}", div, numb1, numb2, numb3);   }
    else {  println!("{} не является делителем всех чисел", div);   }

    let number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let code: [char; 4] = ['О', 'Н', 'Г', 'И'];
    println! ("{}{}{}{}", code[(number/1000-1) as usize], code[((number/100)%10-1) as usize], code[((number%100)/10-1) as usize], code[(number%10-1) as usize]);

    let expon:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut number:i64 = 9;
    for expon in 1..expon {
        number *= 9;
        println! ("{}",number);
    }
    println! ("Последняя цифра 9 в степени {} равна {}", expon, number%10);

    let expon:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut number:i64 = 2;
    for expon in 1..expon {
        number *= 2;
        println! ("{}",number);
    }
    println! ("Последняя цифра 2 в степени {} равна {}", expon, number%10);

    let expon1:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let expon2:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut sum: u32 = 0;
    if (expon1 % 2) == 0 { sum += 1; } else {  sum += 9; }
    if (expon2 % 2) == 0 { sum += 6; } else {  sum += 4; }

    println! ("Последняя цифра суммы равна {}", sum%10);
    let north: String = String::from("Север");
    let west: String = String::from("Запад");
    let south: String = String::from("Юг");
    let east: String = String::from("Восток");
    let dir:[String; 4] = [north, east, south, west];
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Ошибка при чтении строки"); // Обработка ошибки
    input_str = input_str.trim().parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    let direct:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut curr_direct = 4;
    if input_str == dir[0]  {   curr_direct = 0;   }
    else {
        if input_str == dir[1] {   curr_direct = 1;   }
        else {
            if input_str == dir[2] {   curr_direct = 2;   }
            else {  
                if input_str == dir[3] {   curr_direct = 3;   } 
            }
        }
    }
    if direct != 0  {   
        if direct == 1  {   curr_direct = (curr_direct + 3) % 4 ; }
        else {
            if direct == 2 {    curr_direct =  (curr_direct + 1) % 4;   }
        }
    }
    print! ("Направление лунохода после выполнения команды: {}", dir[curr_direct as usize]);*/
    let noleap_year:[u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap_year:[u32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let day:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let month:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let year:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if (month >= 1) && (month <= 12)    {
        if (year >= 1) && (year <= 2024)    {
            if ((year % 4 == 0) || (year % 400 == 0)) && (year % 100 != 0)   {
                if day <= leap_year[month as usize]
                {   println! ("Дата корректна!");    }
                else 
                {   println! ("Дата некорректна!"); }
            }
            else    {
                if day <= noleap_year[month as usize]
                {   println! ("Дата корректна!");    }
                else 
                {   println! ("Дата некорректна!"); }
            }
        }
    }
}   