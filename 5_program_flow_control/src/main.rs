fn coditional_execution() {
    println!("== conditional_execution ==");
    let x = 3;
    if x == 3 {
        println!("x is 3.");
    }

    let y = 4;
    if y != 3 {
        println!("y is NOT 3. y is {y}");
    }
    if y + 1 != 3 {
        println!("y + 1 is NOT 3, y + 1 is {}", y + 1);
    }
}

fn multiple_conditions() {
    println!("\n== multiple_conditions ==");
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y.");
    } else {
        if x < y {
            println!("x is less than y.");
        } else {
            println!("x is equal to y.");
        }
    }

    if x > y {
        println!("x is greater than y.");
    } else if x < y {
        println!("x is less than y.");
    } else {
        println!("x is equal to y.");
    }
}

fn conditional_assignment() {
    println!("\n== conditional_assignment ==");
    let make_x_odd = true;
    let x = if make_x_odd {1} else {2};
    // let x = if make_x_odd {1} else {2.0}; -- this resolves to compilation error because 'if' and 'else' are incompatible types
    // if make_x_odd {
    //     x = 1;
    // } else {
    //     x = 2;
    // }

    println!("x is {x}");
}

fn loops() {
    println!("\n== loops ==");
    let mut count = 0;

    // loop {
    //     if count == 10 {
    //         break;
    //     }
    //     count += 1;
    //     println!("count is {count}");
    // }

    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {count}");
    };

    println!("After the loop.");
    println!("result is {result}");
}

fn loop_while() {
    println!("\n== while ==");
    let mut count_one = 0;
    let letters = ['a', 'b', 'c'];

    while count_one < 10 {
        count_one += 1;
        println!("count is {count_one}");
    }

    println!("");

    let mut count_two = 0;
    while count_two < letters.len() {
        println!("letter is {}", letters[count_two]);
        count_two += 1;
    }
}

fn loop_for() {
    println!("\n== for ==");
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("item is {item}");
    }

    println!("");

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    println!("");

    for number in 0..5 {
        println!("number is {number}");
    }
}

fn nested_loops() {
    println!("\n== nested loops ==");
    let matrix = [[1, 2, 3],
                  [4, 5, 6],
                  [7, 8, 9]];

    for (row_index, row) in matrix.iter().enumerate() {
        for num in row.iter() {
            print!("{}\t", num);
        }
        println!("row #{} is {row:?}", row_index);
    }

    println!("");

    let mut matrix_new = [[1, 2, 3],
                          [4, 5, 6],
                          [7, 8, 9]];

    for row in matrix_new.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }
}

fn challenge() {
    println!("\n== challenge ==");
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let avg: f64;
    
    max = 0;
    min = 0;
    let mut sum = 0;

    for num in numbers {
        sum += num;
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
    }

    avg = sum as f64 / numbers.len() as f64;

    println!("max is {max}");
    println!("min is {min}");
    println!("avg is {}", avg as f64);

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(avg, 12.5);
    println!("Passed!");

    // let my_array = [[1, 2], [3, 4]];
    
    // for sub_array in my_array.iter() {
    //     for element in sub_array.iter() {
    //         println!("{}", element);
    //     }
    // }

    // let x = 1;
    // let y = x;

    // if x > y {
    //     print!("1");
    // } else if x < y {
    //     if x == y {
    //         print!("2");
    //     }
    //     print!("3");
    // } else {
    //     print!("4");
    // }

    // let my_array = [1.0, 2.0, 3.0];
    // for element in my_array.iter() {
    //     *element += 1.0;
    // }
}

fn main() {
    coditional_execution();
    multiple_conditions();
    conditional_assignment();
    loops();
    loop_while();
    loop_for();
    nested_loops();
    challenge();
}
/* 
using:
    loop:
        - repeat a block of code forever
        - need the loop to return a value (e.g.: let result = loop {<expr>})
    while:
        - continue repeating a block of code as long as a condition is true
    for:
        - iterate ofver each item in a collection
        - repeat a block of code N times -> iterate over range 0..N
*/