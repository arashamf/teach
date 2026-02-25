use std::io::stdin;
//use std::io;
fn main() {
    let numb:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
   let mut array = vec![0; numb as usize];
    let mut aver_value:f32 = 0.0;
    for count in 0..numb  {
        array[count] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
        aver_value += array[count];
    }
    println!("Среднее значение: {:.1}", (aver_value/numb as f32)  );
    let mut numb_max:i32 = 0;
    let mut numb_min:i32 = 0;
    for count in 0..numb  {
        array[count] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
        if count == 0 {  numb_max = array[count]; numb_min = array[count];  }
        else {
            if array[count] > numb_max {  numb_max = array[count]; }
            else {
                if array[count] < numb_min {  numb_min = array[count]; }
            }
        }
    }
    println!("Максимальное число: {}",numb_max  );
    println!("Минимальное число: {}", numb_min  );
    
    let mut in_str = String::new();
    const AR_SIZE:usize = 10;
    let mut array = [0; AR_SIZE];
    for count in 0..AR_SIZE  {
        stdin().read_line(&mut in_str).expect("Ошибка при чтении строки");
        array[count] = in_str.trim().parse().expect("Ошибка преобразования типа");
        in_str.clear();
    }
    (array[numb-1], array[numb+1]) = (array[numb+1], array[numb-1]); 
    println!("{:?}", array);

    let number:u64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut buf:u64 = number % 10;
    let mut sum:u64 = buf;
    let mut divider:u64 = 1;
    for count in 1..16  {
        divider *= 10;
        buf = number % (10*divider) / divider;
        if count%2 != 0 {  
            buf *= 2; 
            if buf > 9 {    buf -= 9;   }  
        }
        sum += buf;
    }
    let mut card_number: [u32; 4] = [0; 4];
    divider = 1;
    for count in 0..4  {
        card_number[count] = (number % (10000*divider) / divider) as u32;
        divider *= 10000;
    }
    if sum %10 == 0  {   println!("Карта с номером {:04} {:04} {:04} {:04} действительна",card_number[3],card_number[2],card_number[1],card_number[0]); }
    else {   println!("Карты с номером {:04} {:04} {:04} {:04} не существует",card_number[3],card_number[2],card_number[1],card_number[0]);  }
    let start:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let end:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    for count in start..end  {
        print!("{} | ", count);
    }
    println!("{}", end);

    let number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let size:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut in_number:i32;
    let mut counter:u32 = 0;
    for _ in 0..size  {
        in_number = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");    
        if in_number > number { counter += 1;  }
    }
    println!("Количество элементов больших {number}: {counter}");

    const SIZE:usize = 10;
    let mut arr1 = [0; SIZE];
    let mut arr2 = [0; SIZE];
    let mut flag:bool = true;

    for i in 0..arr1.len() {
        arr1[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    }
    for i in 0..arr2.len() {
        arr2[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    }
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] { 
            flag = false; 
            break;
        }
    }
    if flag == true {    println!("Массивы {:?} и {:?} равны", arr1, arr2);   }
    else {  println!("Массивы {:?} и {:?} не равны", arr1, arr2);   }   

    let mut flag:bool = true;
    let iter:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut number:u32 = 0;
    let end:u32 = 1 << iter; 

    while number < end {    
        if number != (end-1) {flag = false;}
        else {  flag = true;  }
        println!("{:0iter$b} | {}", number, flag as u32);
        number += 1;   
    }
}  