 use std::io::stdin;

fn main() {
    let levels:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let aparts_on_level:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let number_apart:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 

    let entrance = (number_apart -1)/(levels*aparts_on_level) + 1;
    let level = ((number_apart -1)/aparts_on_level)%levels + 1;
    println!("Квартира с номером {} находится в подъезде {} на {} этаже", number_apart, entrance, level);

    let numb_pages:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let lines_on_page:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let symbol_on_page:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    let kbyte_size = (numb_pages*lines_on_page*symbol_on_page*2)/1024;
    println!("Информационный объем статьи в Кбайтах: {}", kbyte_size);

    let bitrate:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let bit_depth:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let sample_rate:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let duration:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    let transmission_time = (2* bit_depth*sample_rate*duration)/bitrate;
    println!("Время передачи голосового сообщения: {} секунд", transmission_time);

    let sample_rate:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let  bitrate:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");
    let compression:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");

    let comp_div:f64 = 1.00 - (compression as f64)/100.00;
    let max_coding_depth:f64 = ((bitrate as f64)*(1024.00*8.00)) / (2.00*1000.00*(sample_rate as f64)*comp_div);

    println!("Максимальная глубина кодирования: {}", (max_coding_depth.round()) as i32);

    let file_size:u32 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    let mono_file_size:u32 = (file_size*2*2)/(3*4);

    println!("Размер файла при повторной записи: {} Мбайт", mono_file_size);
}