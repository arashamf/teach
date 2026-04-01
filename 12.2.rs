use std::io::stdin;
use std::io;

fn input() {
    let mut one_tkt_cst = [0u32; 3];
    for i in 0..3 {
        one_tkt_cst[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    }
    let mut num_tks_sld = [0u16; 3]; 
    for i in 0..3 {
        num_tks_sld[i] = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input"); 
    }
    calc_profit(one_tkt_cst, num_tks_sld);
}

fn calc_profit(one_tkt_cst: [u32; 3], num_tks_sld: [u16; 3]) {
    print_profit((one_tkt_cst[0]*(num_tks_sld[0] as u32))+(one_tkt_cst[1]*(num_tks_sld[1] as u32))+(one_tkt_cst[2]*(num_tks_sld[2] as u32)));
}

fn print_profit(profit: u32) {
    println!("{}",profit); 
}

fn main() {
    input();
}

/*fn input() {
    let tax_rate:f64 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let veh_pow:u16 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    let period:u8 = stdin().lines().next().unwrap().unwrap().parse().expect("wrong input");  
    calc_tax(tax_rate,veh_pow,period);
}*/

fn calc_tax(tax_rate: f64,veh_pow: u16,own_per: u8) {
    print_tax(tax_rate*(veh_pow as f64)*(own_per as f64/12.0));
}
fn print_tax(tax: f64) {
    println!("{:.2}",tax); 
}