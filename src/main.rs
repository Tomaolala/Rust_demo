use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("猜数游戏");
    let scret_number = rand::thread_rng().gen_range(0, 101);
    loop {
        println!("输入猜的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜测的数字是 {}", guess);
        // match处理错误  
        let guess: u32 =match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("请输入数字");
                continue;
            }
        };
        match guess.cmp(&scret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("TOO big"),
            Ordering::Equal => println!("Equal"),
        }
    }






}
