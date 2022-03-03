use std::io;
use std::io::Write;
use std::fs;
use std::env;
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    assert_eq!(numbers_test(&numbers), true);
    println!("Numbers Tests Passed!");

    let char_to_remove = " ";
    assert_eq!(str_test(char_to_remove), true);
    println!("String Tests Passed!");

    assert_eq!(random_game(), true);
    println!("Random Game Passed!"); 

    if env::args().len() <= 2 {
        println!("Need 2 arguments! <file path> <name>");
        return;
    }
    let filename: String = env::args().nth(1).unwrap();
    let person: String = env::args().nth(2).unwrap();
    assert_eq!(roster_test(&filename, &person), true);
    println!("Roster Test Passed!");
}

fn numbers_test(numbers: &[i32]) -> bool {
    let max = max(&numbers);
    let min = min(&numbers);
    let mean = mean(&numbers);
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    return true
}

fn max(numbers: &[i32]) -> i32 {
    let mut max_num = numbers[0];
    for number in numbers {
        if number > &max_num {
            max_num = *number
        }
    };
    return max_num
}

fn min(numbers: &[i32]) -> i32 {
    let mut min_num = numbers[0];
    for number in numbers {
        if number < &min_num{
            min_num = *number
        }
    };
    return min_num
}

fn mean(numbers: &[i32]) -> f64 {
    let mut mean: f64 = 0.0;
    for number in numbers {
        mean += *number as f64
    }
    mean /= numbers.len() as f64;
    return mean
}

fn str_test(remove: &str) -> bool {
    let str_to_remove = remove;
    let str_test1 = "We need more space.";
    assert_eq!(trim_char(str_test1, str_to_remove), "We need more space.");
    let str_test2 = String::from("There's space in front.");
    assert_eq!(trim_char(&str_test2, str_to_remove), "There's space in front.");
    let str_test3 = String::from("There's space to the rear.    ");
    assert_eq!(trim_char(&str_test3, str_to_remove), "There's space to the rear.");
    let str_test4 = "    We're surrounded by space!    ";
    assert_eq!(trim_char(str_test4, str_to_remove), "We're surrounded by space!");
    let str_test5 = "    ";
    assert_eq!(trim_char(str_test5, str_to_remove), "");
    let str_test6 = "";
    assert_eq!(trim_char(str_test6, str_to_remove), "");
    let str_test7 = " 🚀 ";
    assert_eq!(trim_char(str_test7, str_to_remove), "🚀");
    return true
}

fn trim_char<'a>(str: &'a str, str_to_remove: &'a str) -> &'a str {
    let blank_string = "";
    let str_bytes = str.as_bytes();
    let str_to_remove_bytes = str_to_remove.as_bytes();

    // check length
    if str_bytes.len() < 1 {
        println!("String does not have any characters.");
        return blank_string
    }

    for remove in str_to_remove_bytes.iter() {
        // check if all spaces
        let mut all_char = true;
        for item in str_bytes {
            if *item != *remove {
                all_char = false;
                break
            }
        }
        if all_char == true {
            println!("String only contained \"{}\".", str_to_remove);
            return blank_string
        }

        let mut first_index = 0;
        let mut last_index = str_bytes.len() - 1;

        if str_bytes[0] == *remove {
            println!("Found \"{}\" in front", str_to_remove);
            for (index, item) in str_bytes.iter().enumerate() {
                if *item != *remove {
                    first_index = index;
                    break;
                }
            }
        }
        if str_bytes[last_index] == *remove {
            println!("Found \"{}\" in rear", str_to_remove);
            for (index, item) in str_bytes.iter().enumerate().rev() {
                if *item != *remove {
                    last_index = index;
                    break;
                }
            }
        }  
    return &str[first_index..last_index + 1]
    }
    return &str
}

fn random_game() -> bool {
    loop {
        let number: u32 = rand::thread_rng().gen_range(1..100);
        let mut correct_guess = false;
        let mut number_of_guesses = 0;
        let max_guesses = 5;
        while correct_guess == false && number_of_guesses < max_guesses {
            let mut buffer = String::new();
            println!("Enter a number between 0 and 100");
            io::stdin().read_line(&mut buffer).expect("Could not read input!");
            let guess: u32 = buffer.trim().parse().expect("Did not input a number between 0 and 100!");
            number_of_guesses += 1;
            if guess == number {
                println!("You correctly guessed: {} in {} trys!", number, number_of_guesses);
                correct_guess = true
            } else if guess < number {
                println!("{} was too low!", guess);
            } else {
                println!("{} was too high!", guess)
            }
        }
        if number_of_guesses == max_guesses && correct_guess != true {
            println!("You did not guess the number after {} trys!", number_of_guesses);
        }
        println!("Do you want to play again? (y or n)");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Could not read input!");
        if buffer != "y\n" {
            break;
        }
    }
    return true
}

fn roster_test(filename: &str, person: &str) -> bool {
    let file_contents = fs::read_to_string(filename).unwrap();
    let mut in_file = false;
    for (index, content) in file_contents.lines().enumerate() {
        if content == person {
            in_file = true;
            println!("{} is number {} in {}", person, index + 1, filename);
        }
    }
    if in_file == false {
        println!("{} was not in {}", person, filename);
        let mut file = fs::OpenOptions::new().append(true).open(filename).unwrap();
        file.write(b"\n").expect("Error writing file!");
        file.write(person.as_bytes()).expect("Error writing file!");
        println!("{} is now in {}!", person, filename);
    };
    return true
}