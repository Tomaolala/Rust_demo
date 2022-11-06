use std::{io, cmp::Ordering};

use rand::{Rng};

fn main() {
    println!("猜数游戏");
    println!("输入猜的数字");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    
    let scret_number = rand::thread_rng().gen_range(0, 101);
    let guess:u32  =guess.trim().parse().expect("please type a number");
    println!("你猜测的数字是 {}",guess);
    
    println!("神秘数字是 {}",scret_number);
        match guess.cmp(&scret_number) {
            Ordering:: Less=>println!("Too small"),
            Ordering::Greater=>println!("TOO big"),
            Ordering::Equal=>println!("Equal")
        }
}
