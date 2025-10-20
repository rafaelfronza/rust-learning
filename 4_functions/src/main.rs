/* Functions */
fn number_main() {
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {sum}");
}

/* Functions return values */
fn square(x: i32) -> i32 {
    println!("squaring {x}");
    x * x
    // This will return the value of x * x
}

fn square_two(x: i32) -> i32 {
    println!("squaring again {x}");
    return x * x;
    println!("End of function"); // This statement is not executed
}

fn square_three(x: i32) -> (i32, i32) {
    println!("squaring again {x}");
    return (x, x * x);
}

/* Challenge */
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return (1.8 * celsius) + 32.0;
}

fn main() {
    println!("== Functions ==");
    number_main();

    println!("\n== Functions return values ==");
    let result = square(13);
    println!("result is {result}");
    let result_two = square_two(13);
    println!("result is {result_two}");
    let result_three = square_three(13);
    println!("result is the tuple {:?}", result_three); // :? special debug formatting

    println!("\n== Challenge ==");
    let celsius = 23.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{} Celsius to Fahrenheit is: {}", celsius, fahrenheit);

    assert_eq!(fahrenheit, 73.4);
    println!("Test passed!");
}

/*
Statement ends with semicolon ;
Statement does not return a value, it's just an action:
y = x + 1; ---> this is a statement

Expressions does not end with a semicolon
Expressions evaluate to a resulting value
1 + 2 ---> this is an expression
1 + 2; ---> this is a statement

e.g.:
let sum = a + b;

expression => a + b
statement  => let sum = a + b;
*/