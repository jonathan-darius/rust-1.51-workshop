fn main() {
    let (sum_of_nums, product_of_nums, average_of_nums) = {
        let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
        // ====================================================================
        // Option 1 Answer
        // ====================================================================
        // (sum(numbers.clone()), product(numbers.clone()), average(numbers.clone()))
        // ====================================================================

        // ====================================================================
        // Personal Answer
        // ====================================================================
        (sum(&numbers), product(&numbers), average(&numbers))
        // ====================================================================
    };

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

// ====================================================================
// Option 1 Answer
// ====================================================================
// fn sum(numbers: Vec<i64>) -> i64 {
//     let mut total = 0;
//     for num in numbers.iter() {
//         total += num;
//     }
//     total
// }

// fn product(numbers: Vec<i64>) -> i64 {
//     let mut total = 1;
//     for num in numbers.iter() {
//         total *= num;
//     }
//     total
// }

// fn average(numbers: Vec<i64>) -> i64 {
//     let length = numbers.len() as i64;
//     sum(numbers) / length
// }
// ====================================================================

// ====================================================================
// Personal Answer
// ====================================================================
fn sum(numbers: &Vec<i64>) -> i64 {
    let mut total = 0;
    for num in numbers.iter() {
        total += num;
    }
    total
}

fn product(numbers: &Vec<i64>) -> i64 {
    let mut total = 1;
    for num in numbers.iter() {
        total *= num;
    }
    total
}

fn average(numbers: &Vec<i64>) -> i64 {
    let length = numbers.len() as i64;
    sum(numbers) / length
}
// ====================================================================