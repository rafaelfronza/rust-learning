fn borrowing_references() {
    fn to_main() {
        let rocket_fuel = String::from("RP-1");
        let (rocket_fuel, length) = process_fuel(rocket_fuel);
        println!("rocket_fuel is {rocket_fuel} and length is {length}");
    }

    fn process_fuel(propellant: String) -> (String, usize) {
        println!("processing propellant {propellant}...");
        let length = propellant.len();
        (propellant, length)
    }

    to_main();
    /*
    Borrowing:
        - Access data without taking ownership of it
        - Create references using the borrow operator: &
    */
    fn to_main_two() {
        let rocket_fuel = String::from("RP-1");
        let length = process_fuel_two(&rocket_fuel);
        println!("rocket_fuel is {rocket_fuel} and length is {length}");
    }

    fn process_fuel_two(propellant: &String) -> usize {
        println!("processing propellant {propellant}...");
        let length = propellant.len();
        length
    }

    to_main_two();
}

fn mutable_references() {
    fn to_main_two() {
        let mut rocket_fuel = String::from("RP-1");
        let length = process_fuel_two(&mut rocket_fuel);
        println!("rocket_fuel is {rocket_fuel} and length is {length}");
    }

    fn process_fuel_two(propellant: &mut String) -> usize {
        println!("processing propellant {propellant}...");
        propellant.push_str(" is highly flammable!");
        let length = propellant.len();
        length
    }

    to_main_two();
}

fn dangling_references() {
    fn to_main() {
        let rocket_fuel = process_fuel();
        println!("rocket_fuel is {rocket_fuel}");
    }

    // fn process_fuel() -> &String {
    fn process_fuel() -> String{
        let new_fuel = String::from("RP-1");
        // &new_fuel
        new_fuel
    }

    to_main();
}

fn slices() {
    /*
        - Reference to a contiguous section of a collection
        - Commonly encoutered as the string slice data type: &str
        - String literals are slices
    */
    let message = String::from("Greetings from Earth!");
    println!("message is {message}");

    let last_word = &message[15..15+5]; // 15 --> pointer | 15+5 --> chars to read
    println!("last word is {last_word}");
    /*
        - Length is in bytes
        - Range indices must occur at valid UTF-8 character boundaries
    */

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    let outer_planets: &[i32] = &planets[4..];
    println!("inner planets are {:?}", inner_planets);
    println!("outer planets are {:?}", outer_planets);
}

fn slices_as_function_params() {
    fn to_main() {
        let message = String::from("Greetings from Earth!");
        let first_word = get_first_word(&message);
        // let first_word_slice = get_first_word_slice(&message[10..]);
        let first_word_slice = get_first_word_slice(&message);
        println!("first_word is {first_word}");
        println!("first_word_slice is {first_word_slice}");
    }

    fn get_first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index]; // found a space
            }
        }

        &s // no spaces found; input is a single word
    }

    fn get_first_word_slice(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (index, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..index]; // found a space
            }
        }

        &s // no spaces found; input is a single word
    }

    to_main();
    /*
        --> &String != &str
            * &String (string data on the heap)
            * &str (slice borrowing the data on the heap to the pointer)
    */
}

fn challenge() {
    fn trim_spaces(s: &str) -> &str {
        let mut start = 0;
        for (index, character) in s.chars().enumerate() {
            if character != ' ' {
                start = index;
                break;
            }
        }

        let mut end = 0;
        for (index, character) in s.chars().rev().enumerate() {
            if character != ' ' {
                end = s.len() - index;
                break;
            }
        }

        &s[start..end]
    }

    let no_remove = "No remove.";
    let lead_remove = " Remove first.";
    let trail_remove = "Remove last. ";

    let no_remove_ans = trim_spaces(no_remove);
    let lead_remove_ans = trim_spaces(lead_remove);
    let trail_remove_ans = trim_spaces(trail_remove);

    println!("{no_remove_ans}");
    println!("{lead_remove_ans}");
    println!("{trail_remove_ans}");

    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn main() {
    borrowing_references();
    mutable_references();
    dangling_references();
    slices();
    slices_as_function_params();
    challenge();
}