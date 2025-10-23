
use std::io::stdin;

fn main() {
    /*let arr = [-1, 0, 1, 2, 30, 4, 500];
    let numb: usize;
    let mut input_string1 = String::new();
    io::stdin().read_line(&mut input_string1).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string1.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    if numb < 7
    {   println!("{}", arr[numb]);  }*/

    const ARR_LEN:usize = 6; // длина массива
    let mut arr = [0f64 ; ARR_LEN];  // иницилизация нулями массива
    for _i in 0..ARR_LEN {
    let ind:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    arr[_i] = ind;    //заполненеи массива введенными данными 
    }

    let ptr = arr[ARR_LEN - 1] as usize;
    if ptr < (ARR_LEN - 1) 
    {   println!("{:.2}",arr[ptr]); }
}