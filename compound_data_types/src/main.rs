fn arrays() {
    println!("== arrays ==");

    let letters = ['a', 'b', 'b'];
    let first_letter = letters[0];
    println!("first_letter is {first_letter}");

    let mut letters_2 = ['a', 'b', 'b'];
    letters_2[0] = 'x';
    let first_letter_2 = letters_2[0];
    println!("first_letter_2 is {first_letter_2}");

    let numbers: [i32; 5];
    numbers = [0; 5]; // implement 5 copies of the number 0
    println!("last number is {}", numbers[4]);

    // let new_numbers: [i32; 5];
    // new_numbers = [0; 5]; // implement 5 copies of the number 0
    // let index: usize = new_numbers.len();
    // println!("new last number is {}", new_numbers[index]);
}

fn multidimensional_arrays() {
    println!("\n== multidimensional_arrays ==");

    let parking_lot = [[1, 2, 3],
                                      [4, 5, 6]];
    // the interdimensions must be the same for the arrays
    let number = parking_lot[1][2];
    println!("number is {number}");

    // let garage: [[[i32; 100]; 20]; 5];
}

fn tuples() {
    println!("\n== tuples ==");

    let stuff = (10, 3.14, 'x');
    let first_item = stuff.0;
    println!("first_item is {first_item}");

    let mut stuff_2: (u8, f32, char) = (10, 3.14, 'x');
    stuff_2.0 += 3;
    let first_item_2 = stuff_2.0;
    println!("first_item_2 is {first_item_2}");

    let (a, b, c) = stuff_2;
    println!("b is {b}");
}

fn main() {
    arrays();
    multidimensional_arrays();
    tuples();
}
