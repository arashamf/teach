use std::io::stdin;
use std::io;

fn main() {
    let mut numb1:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut numb2:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut numb3:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut buf:f64;

    if numb1 > numb2{
        buf = numb2;
        numb2 = numb1;
        numb1 = buf;
    }
    if numb2 > numb3 {
        buf = numb3;
        numb3 = numb2;
        numb2 = buf;

         if numb1 > numb2{
            buf = numb2;
            numb2 = numb1;
            numb1 = buf;
        }
    }
    println!("{:.1},{:.1},{:.1}", numb1, numb2, numb3);

    let coin:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let note:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if (coin == 1) || (coin == 2) || (coin == 5) || (coin == 10)
    {   println!("Принята монета номинала {}", coin); }
    else        {   println!("Монеты такого номинала не принимаются"); }

    if (note == 5) || (note == 10) || (note == 50) || (note == 100) || (note == 200) 
    || (note == 500) || (note == 1000) || (note == 2000) || (note == 5000)
    {   println!("Принята купюра номинала {}", note); }
    else        {   println!("Купюры такого номинала не принимаются"); }

    let mut number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut buf :i32;
    let mut major_radix :i32 = number/100;
    let mut middle_radix :i32 = (number%100)/10;
    let mut junior_radix :i32 = number%10;

    if major_radix < middle_radix   { 
        buf = major_radix;
        major_radix = middle_radix;
        middle_radix = buf;
    }

   if   middle_radix < junior_radix    {
        buf = middle_radix;
        middle_radix = junior_radix;
        junior_radix = buf;
        if major_radix < middle_radix   { 
            buf = major_radix;
            major_radix = middle_radix;
            middle_radix = buf;
        }
    }    
    number = (major_radix*100) + (middle_radix * 10) + junior_radix;        
    println!("{}",number);

    let mut numb1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut major_radix :i32 = numb1/100;
    let mut middle_radix :i32 = (numb1%100)/10;
    let mut junior_radix :i32 = numb1%10;

    if major_radix >  middle_radix { 
        if  middle_radix != 0 
        {(major_radix, middle_radix) = (middle_radix, major_radix); } //Обмен через кортеж (Rust-идиоматичный способ)
    }
    if major_radix >  junior_radix { 
        if  junior_radix != 0 
        {   (major_radix, junior_radix) = (junior_radix, major_radix); }
    }
    if middle_radix > junior_radix { 
        (middle_radix,junior_radix) = (junior_radix, middle_radix); 
    }

    numb1 = (major_radix*100) + (middle_radix * 10) + junior_radix;        
    println!("{}",numb1);

    let mut in_str1 = String::new();
    io::stdin().read_line(&mut in_str1).expect("Ошибка при чтении строки"); // Обработка ошибки
    in_str1 = in_str1.trim().to_string();
    let city_name1 = String::from("Четный");
    let city_name2 = String::from("Нечетный");
    let test_numb1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let test_numb2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if in_str1 == city_name1 {
        if test_numb1 % 2 > 0   { println!("{} в город {} вход запрещен", test_numb1, in_str1); }
        else                    { println!("{} в город {} вход разрешен", test_numb1, in_str1); }

        if test_numb2 % 2 > 0   { println!("{} в город {} вход запрещен", test_numb2, in_str1); }
        else                    { println!("{} в город {} вход разрешен", test_numb2, in_str1); }
    }

        if in_str1 == city_name2 {
        if test_numb1 % 2 > 0   { println!("{} в город {} вход разрешен", test_numb1, in_str1); }
        else                    { println!("{} в город {} вход запрещен", test_numb1, in_str1); }

        if test_numb2 % 2 > 0   { println!("{} в город {} вход разрешен", test_numb2, in_str1); }
        else                    { println!("{} в город {} вход запрещен", test_numb2, in_str1); }
    }
    let age:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let sys_press:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let dia_press:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    const JUN_LOW_SYS: u32 = 90; const JUN_HIGH_SYS: u32 = 139;
    const JUN_LOW_DIA: u32 = 60; const JUN_HIGH_DIA: u32 = 89;
    const MID_LOW_SYS: u32 = 91; const MID_HIGH_SYS: u32 = 149;
    const MID_LOW_DIA: u32 = 61; const MID_HIGH_DIA: u32 = 91;
    const OLD_LOW_SYS: u32 = 91; const OLD_HIGH_SYS: u32 = 159;
    const OLD_LOW_DIA: u32 = 61; const OLD_HIGH_DIA: u32 = 91;

    if (age >= 18) && (age <= 39) {
        if (sys_press >= JUN_LOW_SYS) && (sys_press <= JUN_HIGH_SYS) && (dia_press >= JUN_LOW_DIA) && (dia_press <= JUN_HIGH_DIA)
        {   println!("Систолическое и Диастолическое АД в норме");  }
        else {  
            if sys_press < JUN_LOW_SYS { println!("Систолическое АД {} ниже нормы на {}", sys_press, JUN_LOW_SYS-sys_press); }
            else    {
                if sys_press > JUN_HIGH_SYS {   println!("Систолическое АД {} выше нормы на {}", sys_press, sys_press - JUN_HIGH_SYS); } 
                else { println!("Систолическое АД в норме"); }       
            }
            if dia_press < JUN_LOW_DIA {
                 println!("Диастолическое АД {} ниже нормы на {}", dia_press, JUN_LOW_DIA-dia_press);  
            } 
            else {
                if dia_press > JUN_HIGH_DIA { println!("Диастолическое АД {} выше нормы на {}", dia_press, dia_press - JUN_HIGH_DIA);  }
                else { println!("Диастолическое АД в норме"); }  
            }   
        }
    }

    if (age >= 40) && (age <= 59) {
        if (sys_press >=  MID_LOW_SYS) && (sys_press <= MID_HIGH_SYS) && (dia_press >= MID_LOW_DIA) && (dia_press <= MID_HIGH_DIA)
        {   println!("Систолическое и Диастолическое АД в норме");  }
        else {
            if sys_press < MID_LOW_SYS { println!("Систолическое АД {} ниже нормы на {}", sys_press, MID_LOW_SYS-sys_press); }
            else    {
                if sys_press > MID_HIGH_SYS {   println!("Систолическое АД {} выше нормы на {}", sys_press, sys_press - MID_HIGH_SYS); } 
                else { println!("Систолическое АД в норме"); }       
            }
            if dia_press < MID_LOW_DIA {
                println!("Диастолическое АД {} ниже нормы на {}", dia_press, MID_LOW_DIA-dia_press);  
            } 
            else {
                if dia_press > MID_HIGH_DIA { println!("Диастолическое АД {} выше нормы на {}", dia_press, dia_press - MID_HIGH_DIA);  }
                else { println!("Диастолическое АД в норме"); } 
            }
        }   
    }

    if age > 59 {
        if (sys_press >=  OLD_LOW_SYS) && (sys_press <= OLD_HIGH_SYS) && (dia_press >= OLD_LOW_DIA) && (dia_press <= OLD_HIGH_DIA)
        {   println!("Систолическое и Диастолическое АД в норме");  }
        else {
            if sys_press < OLD_LOW_SYS { println!("Систолическое АД {} ниже нормы на {}", sys_press, OLD_LOW_SYS-sys_press); }
            else    {
                if sys_press > OLD_HIGH_SYS {   println!("Систолическое АД {} выше нормы на {}", sys_press, sys_press - OLD_HIGH_SYS); } 
                else { println!("Систолическое АД в норме"); }       
            }
            if dia_press < OLD_LOW_DIA {
                println!("Диастолическое АД {} ниже нормы на {}", dia_press, OLD_LOW_DIA-dia_press);  
            } 
            else {
                if dia_press > OLD_HIGH_DIA { println!("Диастолическое АД {} выше нормы на {}", dia_press, dia_press - OLD_HIGH_DIA);  }
                else { println!("Диастолическое АД в норме"); }  
            }
        }   
    }
    let mut s0 = String::new();
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s0).expect("Input error!");
    std::io::stdin().read_line(&mut s1).expect("Input error!");
    std::io::stdin().read_line(&mut s2).expect("Input error!");

    let user_bit:u32 = s0.trim().parse().expect(format!("Unexpected: {}", s0).as_ref()); 
    let group_bit:u32 = s1.trim().parse().expect(format!("Unexpected: {}", s0).as_ref()); 
    let other_bit:u32 = s2.trim().parse().expect(format!("Unexpected: {}", s0).as_ref()); 

    let mut read:bool = false; let mut writes:bool = false; let mut execute:bool = false; let mut fool_access:bool = true;
    let mut tmp_bit :u32;
    if user_bit > 0 {
        read = false; writes = false; execute = false; fool_access = true;
        tmp_bit = user_bit&1;
        if tmp_bit > 0 { execute = true; } else {  fool_access = false; }
        tmp_bit = (user_bit >> 1) & 1;
        if tmp_bit > 0 { writes = true;  }  else {  fool_access = false; }
         tmp_bit = (user_bit >> 2) & 1;
        if tmp_bit > 0 { read = true; }  else {  fool_access = false; }
    }
    else { fool_access = false;  }

    if  user_bit > 0 {
        if fool_access == true {  println!("Other (full access):"); }
        else {  println!("Other:"); }
        if read == true {
            if (writes == false) && (execute == false)  {  print!("    - read only\r\n");  }
           else    {  println!("    - read"); }
        }     
        if  writes == true {
            if (read == false) && (execute == false)  {  print!("    - write only\r\n");  }
            else {  println!("    - write"); }
        }
        if  execute == true  {  
            if (read == false) &&  (writes == false)    {   print!("    - execute only\r\n");  }
            else {  print!("    - execute\r\n"); }           
        }
    }
    else {  println!("User (no access).");  }

    if group_bit > 0 {
        read = false; writes = false; execute = false; fool_access = true;
        tmp_bit = group_bit&1;
        if tmp_bit > 0 { execute = true; } else {  fool_access = false; }
        tmp_bit = (group_bit >> 1) & 1;
        if tmp_bit > 0 { writes = true;  }  else {  fool_access = false; }
         tmp_bit = (group_bit >> 2) & 1;
        if tmp_bit > 0 { read = true; }  else {  fool_access = false; }
    }
    else {  fool_access = false;  }

    if  group_bit > 0 {
        if fool_access == true {  println!("Other (full access):"); }
        else {  println!("Group:"); }
        if read == true {
            if (writes == false) && (execute == false)  {  print!("    - read only\r\n");  }
           else    {  println!("    - read"); }
        }     
        if writes == true {
            if (read == false) && (execute == false)  {  print!("    - write only\r\n");  }
            else {  println!("    - write"); }
        }
        if  execute == true  {  
            if (read == false) &&  (writes == false)    {   print!("    - execute only\r\n");  }
            else {  print!("    - execute\r\n"); }           
        }
    }
    else {  println!("Group (no access).");  }

    if  other_bit > 0 {
        read = false; writes = false; execute = false; fool_access = true;
        tmp_bit = other_bit&1;
        if tmp_bit > 0 { execute = true; } else {  fool_access = false; }
        tmp_bit = (other_bit >> 1) & 1;
        if tmp_bit > 0 { writes = true;  }  else {  fool_access = false; }
        tmp_bit = (other_bit >> 2) & 1;
        if tmp_bit > 0 { read = true; }  else {  fool_access = false; }
    }

    if  other_bit > 0 {
        if fool_access == true {  println!("Other (full access):"); }
        else {  println!("Other:"); }
        if read == true {
            if (writes == false) && (execute == false)  {  print!("    - read only\r\n");  }
           else    {  println!("    - read"); }
        }      
        if writes == true {
            if (read == false) && (execute == false)  {  print!("    - write only\r\n");  }
            else {  println!("    - write"); }
        }
        if  execute == true  {  
            if (read == false) &&  (writes == false)    {   print!("    - execute only\r\n");  }
            else {  print!("    - execute\r\n"); }           
        }
    }
    else {  println!("Other (no access).");  }
}