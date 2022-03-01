fn main() {
    println!("Hello, world!");
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let max = max(&numbers);
    let min = min(&numbers);
    let mean = mean(&numbers);
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests Passed!");
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