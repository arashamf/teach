
use std::io::stdin;

fn main() {
   //Считывание целого числа (usize) и вывод элемента массива по этому индексу
    let arr = [-1, 0, 1, 2, 30, 4, 500];
    let numb: usize;
    let mut input_string1 = String::new();
    io::stdin().read_line(&mut input_string1).expect("Ошибка при чтении строки"); // Обработка ошибки
    numb = input_string1.trim() .parse().expect("Введенное значение не является числом"); // Обработка ошибки парсинга
    if numb < 7
    {   println!("{}", arr[numb]);  }

    //считывание 5 вещественных значений и 1 целое число (usize), запись первые пять значений в массив и вывод (до 2 знаков) элементов коллекции по индексу последнего считанного числа
    const ARR_LEN:usize = 6; // длина массива
    let mut arr = [0f64 ; ARR_LEN];  // иницилизация нулями массива
    for _i in 0..ARR_LEN {
        let ind:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
        arr[_i] = ind;    //заполнение массива введенными данными 
    }
    let ptr = arr[ARR_LEN - 1] as usize;
    if ptr < (ARR_LEN - 1) 
    {   println!("{:.2}",arr[ptr]); }

    //считывание 5 целых чисел, запись их в массив и вывод элементов коллекции по индексу считанных чисел
    const ARR2_LEN:usize = 5; // длина массива
    let mut arr2 = [0i32 ; ARR2_LEN];  // иницилизация нулями массива
    for _i in 0..ARR2_LEN {
        let ind:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
        arr2[_i] = ind;    //заполнение массива введенными данными 
    }

    let mut index;
    for ptr2 in 0..ARR2_LEN     {
        if arr2[ptr2] as usize <= ARR2_LEN   {
            index =  arr2[ptr2] as usize;
            if ptr2 < (ARR2_LEN-1) {
                print!("{}, ", arr2[index]); 
            }
            else    {
                println!("{}", arr2[index]);
            }
        }
    }

    // инициализация массива нулей и замена элемента в массиве по указанному индексу (первое число) на значение второго числа
    let mut arr3 = [0i32 ; 10];  // иницилизация нулями массива
    let mut index2: usize = 0;
    for _i in 0..2 {
        let ind:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
        if _i == 0  {
            if ind < 10   {
                index2 = ind as usize;
            }
        }
        if _i == 1    {
            arr3[index2] = ind;    //заполнение массива введенными данными 
        }
    }
    println!("{:?}", arr3);

    let emp_arr: [i32; 0] = [];
    let emp_arr2 = [0u8; 0];
    let emp_arr3: [i32; 0]; emp_arr3 = [];

    //Деструктуризация массива
    let arr = [0, 1, 1, 2, 3, 5];
    let [null, first, second, third, fourth, fifth] = arr; // деструктуризация
    println!("{null}, {first}, {second}, {third}, {fourth}, {fifth}");
    
    //Считывание целого числа и вывод суммы, разности, произведения предыдущего и следующего элемента по этому индексу
    let arr2 = [-5, 1, 8, 2, 30, 4000, 500000];
    let _index:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if _index < 6 {
       println!("{}", (arr2[_index-1]+arr2[_index+1]));
       println!("{}", (arr2[_index-1]-arr2[_index+1]));
       println!("{}", (arr2[_index-1]*arr2[_index+1]));
    }

    //Считывания двух целых чисел. Замена местами значения по этим индексам, а затем вывод (до 9 знаков) отредактированного массива
    let mut arr3 = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];
    let buf_numb:f64;
    let _ind1:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let _ind2:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    if (_ind1<7) && (_ind2<7) {
        buf_numb = arr3[_ind1];
        arr3[_ind1] = arr3[_ind2];
        arr3[_ind2] = buf_numb;
        println!("{:.9?}",arr3);
    }

}