 use std::io::stdin;

fn main() {
    let number_questions:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let unknown_quest:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let probability:f64 = 1.0 - (unknown_quest as f64)/(number_questions as f64);
    println!("Вероятность попадания выученного вопроса: {:.3}", probability);

    let reliability_1year:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let reliability_2year:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    println!("Вероятность того, что чайник прослужит меньше двух лет, но больше года равна: {:.2}",  (reliability_1year - reliability_2year));

    let beetroot_masses:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let sugar_mass:f64 = (beetroot_masses*1000.0)/100.0*18.0;
    println!("Из {:.3} тонн(ы) получится {:.3} кг сахара", beetroot_masses, sugar_mass);
   
    let sum:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");     
    let percent:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    println!("{:.3}", (sum *(percent /100.0)));

    let lamps:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");     
    let percent:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    println!("Количество привезенных лампочек: {}", (((lamps as f64)/(percent /100.0))as u32));

    let pre_mass:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");     
    let post_mass:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");   
    let dried_fruits:f64 = (post_mass/pre_mass)*100.0;
    println!("Доля сухофруктов относительно свежих фруктов составляет: {:.3}%", dried_fruits);
    println!("Процент массы, потерянный при сушке: {:.3}%", 100.0-dried_fruits);
}