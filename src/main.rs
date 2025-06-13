extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    
    println!("Вы зашли в игру Угадайка");

    let secret_number = rand::rng().random_range(0..100);

    loop {
        
        println!("Введите число или цифру.");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .ok()
            .expect("Ошибка получения ввода!");

        let choice: i32 = choice.trim().parse()
            .ok()
            .expect("Вы вели не число!");

        match choice.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Число мало"),
            std::cmp::Ordering::Equal => {
                println!("Вы выиграли!");
                println!("Выход из игры...");
                break;
            }

            std::cmp::Ordering::Greater => println!("Число велико!"),
        }
    }

}