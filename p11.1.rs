use std::io::stdin;
//use std::io;
fn main() {
    let number:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let mut count:u32 = 0;
    while count < number    {
        print!("{} {}", count, count+1);
        println!("");
        count +=2;
    }
    let number:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    for i in 0..number {
        for j in 1..=(i+1) {
            if j == i+1 {print!("{j}");}
            else        {print!("{j} ");}
        }
        println!("");
    }
    let size:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    // Верхняя половина ромба (включая среднюю линию)
    for i in 0..size {
        for _ in 0..(size-i-1)  {   print!(" ");    }
        for _ in 0..(2*i+1)     {   print!("*");    }
        println!(""); 
    }
     // Нижняя половина ромба (исключая среднюю линию)
       for i in (0..size-1).rev() {
        for _ in 0..(size-i-1)  {    print!(" ");    }       // Пробелы перед звёздочками
        for _ in 0..(2*i+1)     {   print!("*");     }       // Звёздочки
        println!();
    }
    let number:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let mut start:usize = 1;
    if number == 2  {   start = 1;    }
    else            {   start = 2;    }
    for i in start..number {
        if (number%i == 0) && (i!=1) {
            println!("Число {} не является простым", number);
            break;
        }
        else {
            if i>=(number-1)  {
                println!("Число {} является простым", number);
            }
        }
    }
    let mut numb1:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut numb2:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut tmp = 0; 
    while numb1 != 0 {
        tmp = numb1 & numb2;    // Вычисляем биты, где нужно сделать перенос (a & b) 
        numb2 = numb1^numb2;   // Вычисляем сумму без учета переносов (a ^ b)
        numb1 = tmp << 1; // Сдвигаем переносы на один бит влево для следующей итерации
    }
    println!("Результат: {:b} (dec: {})", numb2, numb2);  

    const SIZE:usize = 10;
    let mut series = vec![0; SIZE];
    let numb1:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let numb2:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut pos1 = 0;
    let mut pos2 = 0;
    for i in 0..SIZE {
        series[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
        if series[i] == numb1  {    pos1 = i;   } 
        if series[i] == numb2  {    pos2 = i;   }            
    }
    (series[pos1], series[pos2]) = (series[pos2], series[pos1]);    
    println!("{:?}", series);

    let a:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let b:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut big:i32 = 0;

    // Проверка условия пересечения интервалов
    if a <= b    {
        if b-a <= 1 {
            println!("Решения в целых числах нет");
        }    
        else {
            big = b - 1;
            println!("{big}");
        }    
    }
    else {
        println!("A и B не пересекаются");
    }
    const SIZE:usize = 10;
    let mut series:Vec<i32> = vec![0; SIZE];  //инициализация массива
    let mut flag:bool = false;
    let mut buf:i32 = 0;
    for i in 0..SIZE {
        series[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    }
    series.sort();
    buf = series[SIZE-1]+1;
    for i in 0..SIZE-1 {
        if series[i] == series [i+1] {  
            if buf != series[i]    {   print! ("{} ", series[i]);  }  
            buf = series[i];
            flag = true;
        }
    }
    if flag == true {   println! ("");  }
    else            {   println! ("Повторяющихся чисел нет");  }

    let numb1:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let numb2:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut cnt:u8 = 0;
    for i in 0..8  {
        if  (numb1 >> i)&0x01 != (numb2 >> i)&0x01 {
            cnt += 1;
        }
    }
    println!("{cnt}")  ;
}

