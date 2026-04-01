use std::io::stdin;

fn main() {
    /* 
   let mut number:u16=123;

   while number < 322 {
        let i = number % 10;
        let j = (number / 10)%10;
        let k = (number / 100)%10;
        if (i != j) &&  (j != k) &&  (i != k) && ((i == 1) || (i==2) || (i==3)) && 
        ((j == 1) || (j==2) || (j==3)) && ((k == 1) || (k==2) || (k==3)) {
            println! ("{number}");
            number += 9;
        }
        else {  number += 1;    }
   }
 
    let iter:usize = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number:f32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut res:f32 = 1.0;  
    for i in 0..=iter {
        res += number/(2.0*number+1.0);
    }
    println! ("{:.2}",res);
    for i in 0..=iter {
        res *= (number+1.0)/((number*number)/2.0+4.0);
    }
    println! ("{:.5}",res);
    let mut res:f64 = 1.0;  
    for i in 2..=iter   {
        if i%2==0   {   res -= 1.0/(i as f64);    }
        else        {   res += 1.0/(i as f64);    }
    }
    println! ("{:.*}",iter,res);

    let numb_try:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let chance:f32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let success_try:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut succes_ch:f32 = 1.0;
    let mut fail_ch:f32 = 1.0;
    for _ in 0..(success_try as usize)   {
        succes_ch *= chance;
    }
    for _ in 0..((numb_try - success_try) as usize)   {
        fail_ch *= 1.0-chance;
    }
    let series_chance = fail_ch * succes_ch;
    println! ("{:.2}",series_chance);

    let sum:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let term:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mut chance:u8 = 0;
    let mut success_ch:u8 = 0;
    let end:u8;
    if sum > 6  {    end = 6;    }
    else        {    end = sum-1;   }
    for i in 1..=end   {
        let buf = sum - i; 
        if (buf > 0) && (buf <= 6)  {
            chance += 1;
            if (i == term) || (buf == term)   {
                success_ch += 1;
            }
        }
    }
    println! ("{:.2}", (success_ch as f32)/(chance as f32));*/
    let mut x:i8;
    let mut y:i8;
    let mut z:i8;
    println!(" x  y  z");
    for i in -10..=10 {
        x = i;
        for j in -3..=3 {
            y = j;
            for k in 2..=6   {
                z=k;
                if (4*x-(2*y)+(3*z)) == 20  {
                    if x*y*z<15 {
                        if (x*x)+(y*y)+(z*z) > 8  {
                            println!("{:2} {:2} {:2}", x,y,z);
                        }
                    }
                }
            }
        }
    }
}

