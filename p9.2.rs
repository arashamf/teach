use std::io::stdin;

fn main() {
    let mut array:[i32; 5] = [1, 2, 3, 4, 5];
    let number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    array[0] += number;
    array[1] -= number;
    array[2] *= number;
    array[3] /= number;
    array[4] %= number;
    println!("{:?}", array);
    let numb1:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb2:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    println!("{:08b} :a", numb1);
    println!("{:08b} :b", numb2);
    println!("{:08b} :a & b", numb1&numb2);

    let numb1:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let numb2:i8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    println!("{:08b} :a", numb1);
    println!("{:08b} :b", numb2);
    println!("{:08b} :a | b", numb1|numb2);

    let number:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    if number >= 0 {
        println!(" {}: {:032b} :a", number, number);
        println!("{}: {:032b} :!a", !number, !number);
    }
    else {
        if number < 0 {
            println!("{}: {:032b} :a", number, number);
            println!(" {}: {:032b} :!a", !number, !number);
        }
    }
    let mut numb1:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let mut numb2:i32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    numb1^=numb2;
    numb2^=numb1;
    println!("{}", numb1^numb2);
    println!("{}", numb2);

    let mut number:u64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let radix:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    number *= 1 << radix;
    println!("{}", number);

    let mut number:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let radix:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    number /= 1 << radix;
    println!("{}", number);
}   