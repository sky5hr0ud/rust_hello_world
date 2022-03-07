use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::fs;
use std::env;
use std::mem;
use std::ops::Add;
use std::fmt;
use rand::prelude::*;
use std::collections::HashMap;

// Need to provide two args when ran <filepath> <name>
fn main() {
    println!("Hello, world!");
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    assert_eq!(numbers_test(&numbers), true);
    println!("Numbers Tests Passed!");

    let char_to_remove = " ";
    assert_eq!(str_test(char_to_remove), true);
    println!("String Tests Passed!");

    //assert_eq!(random_game(), true);
    //println!("Random Game Passed!"); 

    if env::args().len() <= 3 {
        println!("Need 2 arguments! <file path> <name> <book_name>");
        return;
    }
    let filename: String = env::args().nth(1).unwrap();
    let person: String = env::args().nth(2).unwrap();
    assert_eq!(roster_test(&filename, &person), true);
    println!("Roster Test Passed!");

    assert_eq!(shape_test(1.0, 2.0, 3.0), true);
    println!("Shape Test Passed!");

    assert_eq!(*sum_boxes(1, 2), 3);
    assert_eq!(*sum_boxes(3.14159, 2.71828), 5.85987);
    println!("Boxes Test Passed!");

    assert_eq!(satellite_test(String::from("Hubble Telescope"), 2),
               "Hubble Telescope moving at 2 miles per second!");
    assert_eq!(satellite_test(String::from("Voyager"), -5000000.231423),
               "Voyager moving at -5000000.231423 miles per second!");
    satellite_test("Super Secret Spy Satellite".to_string(), "negative five");
    println!("Satellite Tests Passed!");

    assert_eq!(address(), true);
    println!("Address Test Passed!");

    let book_name: String = env::args().nth(3).unwrap();
    for index in 1..18 {
        println!("Searching for {} character long words!", index);
        if book_test(&book_name, index) {
            continue;
        } else {
            panic!("Error with book function!");
        }
    }
    println!("Unique words in book test passed!");
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

fn trim_char<'a, 'b>(str: &'a str, str_to_remove: &'b str) -> &'a str {
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
            let guess = user_number_input(0, 100);
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
        let next_game = user_input();
        let mut play_next = false;
        let valid_next_games = ["y\n", "yes\n", "Y\n", "YES\n"];
        for valid in valid_next_games {
            if next_game == valid {
                play_next = true;
            }
        }
        if play_next == false {
            break;
        }
    }
    return true
}

fn user_number_input(lower_limit: u32, upper_limit: u32) -> u32 {
    let mut loop_counter = 0;
    let max_loops = 100;
    loop {
        println!("Enter a number between {} and {}", lower_limit, upper_limit);
        let parse_message = user_input().trim().parse::<u32>();
        let parsed_message_success = match parse_message {
            Ok(number) => (true, number),
            Err(_) => (false, 0)
            };
        if parsed_message_success.0 {
            if parsed_message_success.1 > lower_limit &&
               parsed_message_success.1 < upper_limit {
                return parsed_message_success.1
               } else {
                loop_counter += 1;
                println!("Input not between bounds of {} and {}", lower_limit, upper_limit);
               }
        } else {
            loop_counter += 1;
            println!("Input not an integer between bounds of {} and {}", lower_limit, upper_limit);
        }
        if loop_counter >= max_loops {
            panic!("Error: Incorrect input supplied too many times!");
        }
    }
}

fn user_input() -> String {
    let mut loop_counter = 0;
    let max_loops = 100;
    loop {
        let mut buffer = String::new();
        let input = io::stdin().read_line(&mut buffer);
        let user_output = match input {
            Ok(_) => (true, String::from(buffer)),
            Err(_) => (false, String::from("Input error! Please try again!"))
        };
        if user_output.0 {
            return user_output.1
        } else {
            loop_counter += 1;
            println!("{}", user_output.1);
        }
        if loop_counter >= max_loops {
            panic!("Error: Incorrect input supplied too many times!");
        }
    }
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

#[derive(Debug)]
struct Cuboid {
    length: f64,
    width: f64,
    height: f64
}

impl Cuboid {
    fn new(length: f64, width: f64, height: f64) -> Cuboid {
        Cuboid {
            length: length,
            width: width,
            height: height
        }
    }
    fn get_length(&self) -> f64 {
        return self.length
    }
    fn set_length(&mut self, new_length: f64) {
        self.length = new_length
    }
    fn get_width(&self) -> f64 {
        return self.width
    }
    fn set_width(&mut self, new_width: f64) {
        self.width = new_width
    }
    fn get_height(&self) -> f64 {
        return self.height
    }
    fn set_height(&mut self, new_height: f64) {
        self.height = new_height
    }
    fn get_xyz(&self) -> (f64, f64, f64) {
        return (self.get_length(), self.get_width(), self.get_height())
    }
    fn surface_area(&self) -> f64 {
        return 2.0 * (self.get_length() * self.get_width() + 
                      self.get_length() * self.get_height() +
                      self.get_height() * self.get_width())
    }
    fn volume(&self) -> f64 {
        return self.get_length() * self.get_width() * self.get_height()
    }
    fn scale(&mut self, scale_factor: f64) {
        self.set_length(self.get_length() * scale_factor);
        self.set_width(self.get_width() * scale_factor);
        self.set_height(self.get_height() * scale_factor);
    }
}

fn shape_test(length: f64, width: f64, height: f64) -> bool {
    let mut cuboid1: Box<Cuboid> = Box::new(Cuboid::new(length, width, height));
    println!("Cuboid has dimensions: {:?}, surface area: {:?}, volume: {:?}", 
        cuboid1.get_xyz(), cuboid1.surface_area(), cuboid1.volume());
    cuboid1.scale(100.0);
    println!("Cuboid has dimensions: {:?}, surface area: {:?}, volume: {:?}", 
        cuboid1.get_xyz(), cuboid1.surface_area(), cuboid1.volume());
    println!("Memory size of Cuboid pointer: {:?} bytes!", mem::size_of_val(&cuboid1));
    println!("Memory size of Cuboid: {:?} bytes!", mem::size_of_val(&*cuboid1));
    return true
}

fn sum_boxes<T: std::ops::Add + Add<Output = T> + std::fmt::Debug>(num1: T, num2: T) -> Box<T> {
    let number1 = Box::new(num1);
    let number2 = Box::new(num2);
    let number3 = Box::new(*number1 + *number2);
    return number3
}

struct Satellite<T> {
    name: String,
    velocity: T // miles per second
}

trait Description {
    fn describe(&self) -> String;
}

impl<T: std::fmt::Display> Description for Satellite<T> {
    fn describe(&self) -> String {
        format!("{} moving at {} miles per second!", self.name, self.velocity)
    }
}

impl<T> Satellite<T>
    where T: std::cmp::PartialOrd + std::fmt::Display {    
    fn compare(&self, satellite2: Satellite<T>) {
        if self.velocity > satellite2.velocity {
            println!("{}", &self.describe());
        } else if self.velocity < satellite2.velocity {
            println!("{}", &satellite2.describe());
        } else {
            println!("{} == {}", &self.describe(), &satellite2.describe());
        }
    }
    fn new(name: String, initial_velocity: T) ->
    Satellite<T> {
        Satellite {
            name: name,
            velocity: initial_velocity
        }
    }
}

fn satellite_test<T>(name: String, initial_velocity: T) -> String
    where T: fmt::Display + PartialOrd {
    let satellite1 = Satellite::new(name, initial_velocity);
    println!("{}", satellite1.describe());
    satellite_test2();
    return satellite1.describe();
}

fn satellite_test2() -> bool {
    let satellite1 = Satellite::new("Hubble2".to_string(), 1000.0);
    let satellite2 = Satellite::new("James Web".to_string(), 500.0);
    satellite1.compare(satellite2);
    let satellite1 = Satellite::new("Hubble2".to_string(), 1000.0);
    let satellite2 = Satellite::new("James Web".to_string(), 5000);
    compare_satellite(satellite1, satellite2);
    return true
}

// how do you put this in an impl?
fn compare_satellite<T,U>(satellite1: Satellite<T>, satellite2: Satellite<U>)
    where T: std::cmp::PartialOrd + std::fmt::Display + From<U>,
          U: std::cmp::PartialOrd + std::fmt::Display + Copy {
    if &satellite1.velocity > &T::from(satellite2.velocity) {
        println!("{}", &satellite1.describe());
    } else if &satellite1.velocity < &T::from(satellite2.velocity) {
        println!("{}", &satellite2.describe());
    } else {
        println!("{} == {}", &satellite1.describe(), &satellite2.describe());
    }
}

enum Location {
    Unknown,
    Anon,
    Known(f64, f64) // latitude, longitude
}

trait Display {
    fn display(&self) -> String;
}

impl Display for Location {
    fn display(&self) -> String {
        match *self {
            Location::Unknown => format!("Location not known \u{1F622}"),
            Location::Anon => format!("Location request refused \u{1F620}"),
            Location::Known(lat, lon) => format!("Address is at location {} {} \u{1F911}", lat, lon)
        }
    }
}

fn address() -> bool {
    let address = Location::Unknown;
    println!("{}", address.display());
    let address = Location::Anon;
    println!("{}", address.display());
    let address = Location::Known(28.608295, -80.604177);
    println!("{}", address.display());
    return true
}

fn book_test(book_name: &str, word_length: i32) -> bool {
    let mut book_contents;
    let mut read_book = file_reader(&book_name);
    if read_book.0 {
        //println!("Successfully read book!");
        book_contents = read_book.1;
    } else {
        // let the user input a path to the book
        println!("{} File: {}", read_book.1, book_name);
        while read_book.0 == false {
            println!("Input path to book!");
            let input = &user_input();
            read_book = file_reader(input.trim());
            if read_book.0 {
                //println!("Successfully read book!");
            } else {
                println!("{} File: {}", read_book.1, input.trim());
            }
        }
        book_contents = read_book.1;
    }
    let punctuations = [".", ",", ";", ":", "'", "\"", "!"];
    for punctuation in punctuations {
        book_contents = book_contents.replace(punctuation, "");
    }
    book_contents = book_contents.to_ascii_lowercase();
    let mut book_contents_vec: Vec<&str> = book_contents.split_whitespace().collect();
    book_contents_vec.retain(|element| element.chars().count() == word_length.try_into().unwrap());
    book_contents_vec.retain(|element| element.chars().all(char::is_alphabetic));
    book_contents_vec.retain(|element| element.starts_with("www") == false);
    let mut unique_words = HashMap::new();
    for element in &book_contents_vec {
        let word = unique_words.entry(element).or_insert(1);
        *word += 1
    }
    let mut highest_value = 0;
    let mut highest_value_word: Vec<(&str, i32)> = Vec::new();
    let max_top_words = 5;
    for element in &book_contents_vec {
        let output =  unique_words.get_key_value(&element).expect("Not Found!");
        if output.1 > &highest_value {
            highest_value = *output.1;
            if highest_value_word.len() >= max_top_words {
                highest_value_word.clear();
            }
            highest_value_word.push((element, *output.1));
        } else if output.1 == &highest_value {
            highest_value_word.push((element, *output.1));
            highest_value_word.sort();
            highest_value_word.dedup();
        }
    }
    println!("The top word occurs {} times!", highest_value);
    println!("The top {} words are:", highest_value_word.len());
    for word in highest_value_word {
        println!("The word {} which occurs {} times.", word.0, word.1);
    } 
    return true
}

fn file_reader(file: &str) -> (bool, String) {
    let file_to_be_read = file;
    let contents = fs::read_to_string(file_to_be_read);
    let content = match contents {
        Ok(message) => (true, message),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => (false, String::from("File not found!")),
            io::ErrorKind::PermissionDenied => (false, String::from("No file access permissions!")),
            _ => (false, String::from("Error accessing file!"))
        }
    };
    return content
}