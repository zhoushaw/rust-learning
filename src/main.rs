use std::io; // prelude
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1,101); // i32 u32 i64

    // println!("神秘数字是: {}",secret_number);

    loop {
        println!("猜一个数!");
    
        // mut 关键词表示其可变
        // :: 关键词表示是其静态方法
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // io:: Result Ok，Err 
    
        // 可以覆盖上面的变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你猜测的数是：{}",guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("To big !"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
