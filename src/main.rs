use std::io;
use rand::Rng;

#[derive(PartialEq)]
enum Moves {
    Rock,
    Scissors,
    Paper
}

// Получение хода от бота
fn get_bot_step() -> Moves {
    use Moves::*;
    let num = rand::thread_rng().gen_range(1..4);
    let bot_step = {
        if num == 1 {
            Rock
        } else if num == 2 {
            Scissors
        } else {
            Scissors
        }
    };
    bot_step
}

// Получение номера хода от пользователя
fn get_user_num() -> i32 {
    loop {
        println!("Ваш ход: ");
        let mut str_u_num = String::new();
        let _ = io::stdin().read_line(&mut str_u_num);  // Получаем данные 
        let u_step: i32 = str_u_num.trim().parse().unwrap();

        if !(u_step > 0 && u_step < 4) {
            println!("Нет такого хода :(");
            continue;
        } else {
            return u_step;
        }
    }
}

// Определение победителя
fn who_win(u_step: &Moves, b_step: &Moves) {
    use Moves::*;
    if u_step == b_step {
        println!("НИЧЬЯ!");
    } else {
        match u_step {
            Rock => {
                let mes = if b_step == &Scissors {String::from("ВЫИГРАЛ")} else {String::from("ПРОИГРАЛ")};
                println!("\nТЫ {}!", mes);
            },
            Scissors => {
                let mes = if b_step == &Paper {String::from("ВЫИГРАЛ")} else {String::from("ПРОИГРАЛ")};
                println!("\nТЫ {}!", mes);
            },
            Paper => {
                let mes = if b_step == &Rock {String::from("ВЫИГРАЛ")} else {String::from("ПРОИГРАЛ")};
                println!("\nТЫ {}!", mes);
            }
        }
    }
}

// Хочет ли игрок продолжить игру
fn the_end() -> i8 {
    let mut str_choice = String::new();
    
    println!("\n==========================");
    println!("    Желаете закончить?\n[1] - Да\t[2] - Нет");
    println!("==========================");

    println!("Ваш выбор: ");
    let _ = io::stdin().read_line(&mut str_choice);
    let choice: i32 = str_choice.trim().parse().unwrap();

    if choice == 1 {1} else {2}
}

fn main() {
    println!("Игра началась!");
    loop {
        println!("=================");    
        println!("[1] - Камень\n[2] - Ножницы\n[3] - Бумага");
        println!("=================\n"); 

        // Ход игрока
        let u_step = {
            let u_num = get_user_num();
            if u_num == 1 {
                println!("\n- Ход игрока:  Камень");
                Moves::Rock
            } else if u_num == 2 {
                println!("\n- Ход игрока:  Ножницы");
                Moves::Scissors
            } else {
                println!("\n- Ход игрока:  Бумага");
                Moves::Paper
            }
        };

        // Ход бота
        let b_step = get_bot_step();
        use Moves::*;
        match b_step {
            Rock => println!("- Ход оппонента:  Камень"),
            Scissors => println!("- Ход оппонента:  Ножницы"),
            Paper => println!("- Ход оппонента:  Бумага"),
        }

        who_win(&u_step, &b_step);

        let end = the_end();
        if end == 1{
            break;
        } else {
            continue;
        }
    }
}
