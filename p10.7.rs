use std::io::stdin;
use std::io;
fn main() {
    let num_str:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    for  count in 0..num_str    {
        for _ in 0..num_str-1-count   { print!(" ");    }
     for _ in 0..1+count   {    print!("*");    }
        for _ in 0..count   {  print!("*");    }
        println!("");
    }
    let num:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let block:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let end=num/block + num%block;
    let mut number:u32=0;
    for  _ in 0..end    {
        for count in 0..block    { 
            number += 1;
            if number <= (num as u32)    {print!("{}", number);}
            else {  break;  }
            if count < block-1 {    print!(" ");    }
        }
        println!("");
    }
    const SIZE:usize = 10;
    let mut series = vec![0; SIZE];
    let mut longest = vec![0; SIZE];               // длина наибольшей возрастающей подпоследовательности, заканчивающейся на i
    let mut max_start = 0;
    let mut max_len = 1;
    let mut current_start = 0;
    let mut current_len = 1;

    for count in 0..SIZE    {   series[count]   = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  }

    for i in 1..series.len() {
        if series[i] > series[i - 1]    {
            current_len += 1;
            if current_len > max_len    {
                max_len = current_len;
                max_start = current_start;
            }
        } 
        else {
            current_start = i;
            current_len = 1;
        }
    }
    longest = series[max_start..max_start + max_len].to_vec();
    for count in 0..max_len {
        print!("{} ", longest[count]);
    }
    println!(""); 

    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    let num: usize = user_input.trim().parse().unwrap();

    for count in 1..=num {
        for _ in 1..=num-count {
            print!(" ");
        }
        let mut value:u64 = 1;
        print!("{value}");
        for j in 1..count {
            value *= count as u64 - j as u64;
            value /= j as u64;
            print!(" {}", value);
        }
        println!("");
    }
    let num_pass:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut announce = String::from("");
    let mut input = String::new();
    let mut series:Vec<u32> = vec![0; num_pass];
    let mut counter:u32= 0;
    let mut flag:bool = false;
    let mut str_buf:String = String::new();

    for count in 0..num_pass    {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        series[count] = input.trim().parse().unwrap();
    }
    loop {
        counter += 1;
        for i in 0..num_pass {
            let pass: u32 = (series[i] >> 16)&0xFFFF;
            if pass==counter  {
                if flag == true {   str_buf =  format!(", {}", series[i]&0xFFFF); }
                else {   str_buf =  format!("{}", series[i]&0xFFFF); }
                announce.push_str(&str_buf);
                flag = true;
                str_buf.clear();
                series[i] = 0;
            }
        }
        //println!("Поезд прибыл на Станцию № {}!", counter);
        if flag == true {
            println!("Поезд прибыл на Станцию № {}!", counter);
            println!("Просим на выход пассажиров с номером(ами):");
            println!("{announce}");
            announce.clear();
            flag = false;
        }
        if series[num_pass-1] == 0 {break;  }
    }

    let number:u64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut buf:u64 = number;
    let mut numb_double:usize=0;
    let mut numb_double_cnt:usize=0;
    let mut count = 0;
    let mut end = 0;
    let mut flag:bool = true;
    let mut ptr = 0;
    let mut cnt = 0;

    while buf != 0 {
        count += 1;
        buf /= 10;
    }

    //инициализация массива
    let mut series:Vec<u32> = vec![0; count];
    buf = number;
    for i in 0..count {
        series[i] = (buf % 10) as u32;
        buf /= 10;
    }


    //сортировка по возрастанию
    for i in 0..count {
        for j in i+1..count     {
            if series[i] < series[j]   {  (series[j], series[i]) = (series[i], series[j]);    }
        }
    }
   //поиск дублей
    while cnt < count-1 {
        ptr = cnt + 1;
        while series[cnt] == series[ptr]   {   
            numb_double_cnt += 1;   
            if ptr < cnt {    ptr +=1;    }
            else { break;   }
        }  
        if  numb_double_cnt != 0 {
            if numb_double_cnt % 2 != 0 {   numb_double += numb_double_cnt;   }
            else {numb_double += numb_double_cnt-1;}
            cnt = ptr+1;
        }
        else {cnt +=1;}
        numb_double_cnt = 0;
    }
    
    for i in 0..count-1 {
        for j in i+1..count     {
            if series[i] == series[j]   {  
                (series[count-1-i], series[i]) = (series[i], series[count-1-i]);    
                break;
            }
        }
    }

    if count%2 == 0 {   
        end = count/2-1;
        if numb_double != (end + 1) {   flag = false;   }    
    }
    else            {   
        end = count/2;  
        if numb_double != end  {   flag = false;   }           
    
    }

    if flag == true {
        //сортировка по убыванию
        for i in 0..end+1 {
            for j in 0..i+1 {
                if series[i] > series[j]   {  (series[i], series[j]) = (series[j], series[i]);    }
            }
        }

        //сортировка по возрастанию
        for i in end+1..count {
            for j in end+1..i+1     {
                if series[i] < series[j]   {  (series[j], series[i]) = (series[i], series[j]);    }
            }
        }

        if count%2 != 0 {
            for i in 0..end+1   {
                for j in end+1..count {
                    if series[i] == series[j]   {break;}
                    else {
                        if j == count-1 {
                            if i != end { (series[i], series[end]) = (series[end], series[i]); }
                        }
                    }
                }

            }
        }
        for i in 0..count { print!("{}", series[i]); }
        println!(""); 
    }
    else    {    println!("Число {number} не образует палиндром");  }
}
