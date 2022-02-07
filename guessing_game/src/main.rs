extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 봅시다!"); // Game Start

    let secret_number = rand::thread_rng().gen_range(1..101); // Secret number generate

    loop {
        println!("추측하는 값을 입력해주세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("라인을 읽어오지 못 했습니다.");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; // String convert to u32 type
        
        println!("입력한 숫자: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("숫자가 너무 작습니다."),
            Ordering::Greater => println!("숫자가 너무 큽니다."),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            },
        }
    }

}
