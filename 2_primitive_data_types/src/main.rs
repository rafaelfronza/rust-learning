fn integer_data_types() {
    println!("== integer_data_types ==");
    let x = 10;
    println!("X is {}", x);
    // x = 20;
    // println!("X is {}", x);
    let y: u8 = 255;
    // y = y + 1;
    println!("Y is {}", y);
    /*
        i = Signed (can go to negatives)
        u = Unsigned (only positives)
    */
}

fn floating_data_types() {
    println!("\n== floating_data_types ==");
    /*
        Float: only 2 floating points
            - f32
            - f64
    */
    let z: f32 = 10.123456789123456789;
    println!("z is {}", z);
}

fn arithmetic_operations() {
    println!("\n== arithmetic_operations ==");
    let a = 10;
    let b = 3.0;
    // let c = a + b;
    // let c = a * b;
    // let c = a as f64 / b;
    let c = a as f64 / (b + 1.0);
    // let c = a % b;
    println!("c is {}", c)
}

fn formatting_print_statements() {
    println!("== formatting_print_statements ==");
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    println!("c is {:.3}", c);
    println!("c is {:8.3}", c);
    println!("c is {:08.3}", c);
    println!("c is {:08.3}\na is {}", c, a);
    // print! macro does not add automatically the break-line in the end of the string to be printed
    print!("c is {0:08.3}\na is {1}\nc is once again is {0}\n\n", c, a);
    println!("a is {a}");
}

fn bitwise_operations() {
    println!("\n== bitwise_operations ==");
    let mut value = 0b1111_0101u8;
    println!("value is {value}");
    println!("value is {value:08b}");
    /*
    Bitwise Operators:
        - Logical operations on patterns of bits at the individual bit level
            -- NOT
            -- AND
            -- OR
            -- XOR
            -- SHIFT
    */
    value = !value; // NOT
    println!("value is {value:08b}");

    value = value & 0b1111_0111; // AND
    println!("value is {value:08b}");
    let bit_six = value & 0b0100_0000;
    println!("bit 6 is {bit_six}");

    value = value | 0b0100_0000; // OR
    println!("value is {value:08b}");

    value = value ^ 0b0101_0101; // XOR
    println!("value is {value:08b}");

    value = value << 4; // Left SHIFT
    println!("value is {value:08b}");

    value = value >> 4; // Right SHIFT
    println!("value is {value:08b}");
}

fn boolean_operations() {
    println!("\n== booldan_operations ==");
    let a = true;
    let b = false;

    println!("a is {a} and b is {b}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let c = (a ^ b) | (a & b);
    println!("c is {c}");

    // Short-circuiting Logical Operation: &&, ||
    // This evaluates the first operator of the expression and can determine wheter it's false or true
    let d = (a ^ b) || (a & b);
    println!("c is {d}");

    let e = (a ^ b) || panic!(); // Won't crash
    println!("c is {e}");

    // let e = (a ^ b) && panic!(); // Will crash
    // println!("c is {e}");
}

fn comparison_operations() {
    println!("\n== comparison_operations ==");
    let a = 1;
    let b = 2;

    println!("a is {a} and b is {b}");
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);

    let a = true;
    let b = false;

    println!("a is {a} and b is {b}");
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
}

fn char_data_types() {
    println!("\n== char_data_types ==");
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}'; // unicode char for a finger pointing up (4bytes)

    println!("Letter: {}\nNumber: {}\nFinger: {}", letter, number, finger);
}

fn challenge() {
    println!("\n== challenge ==");
    let a = 13; // int
    let b = 2.3; // float 64bit
    let c: f32 = 120.0; // float 32bit

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Passed");
}

fn tests() {
    println!("\n== tests ==");
    let a: u8 = 19;
    let b: u8 = 5;
    let c = a / b;
    println!("{c}");

    let mut x = 0b1111_0101u8;
    x = !x;
    println!("{x:04b}");

    let y: f64 = 10.287628736;
    println!("{y:04.2}");

    let A = true;
    let B = false;
    let C = (A | B) & (A ^ B);
    println!("{C}");
}

fn main() {
    integer_data_types();
    floating_data_types();
    arithmetic_operations();
    formatting_print_statements();
    bitwise_operations();
    boolean_operations();
    comparison_operations();
    char_data_types();
    challenge();
    tests();
}
