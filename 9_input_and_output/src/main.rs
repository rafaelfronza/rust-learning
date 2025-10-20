use std::env;
use std::fs;
use::std::io::Write;

// fn cmd_line_arguments() {
//     println!("=== cmd_line_arguments ===");
//     if env::args().len() <= 2 {
//         println!("Program requires at least 2 arguments.");
//         return;
//     }

//     for (index, argument) in env::args().enumerate() {
//         println!("argument {} is {}", index, argument);
//     }

//     let arg2 = env::args().nth(2).unwrap();
//     // let str_arg2 = env::args().nth(2).unwrap(); // always as string
//     // let arg2: u32 = str_arg2.trim().parse().unwrap();
//     println!("argument arg2 is {}", arg2);
// }

// fn reading_from_files() {
//     println!("\n=== reading_from_files ===");
//     let contents = fs::read_to_string("planets.txt").unwrap();
//     println!("contents from file: \n{}", contents);

//     for line in contents.lines() {
//         println!("line is {}", line);
//     }

//     let contents = fs::read("planets.txt").unwrap();
//     println!("contents is {:?}", contents);
// }

// fn write_to_file() {
//     println!("\n=== write_to_file ===");

//     let mut speech = String::new();
//     speech.push_str("We choose to go to the moon this decade\n");
//     speech.push_str("and do the other things,\n");
//     speech.push_str("not becaue they are easy,\n");
//     speech.push_str("but because they are hard.");

//     fs::write("speech.txt", speech);

//     let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
//     file.write(b"\nPluto");
// }

fn challenge() {
    /*
        * program to check if a specific person exists in a list of names
        * print a message if the name was or wasn't found
        * 2 arguments:
                file name --> file.txt 
                name --> Jason
    */
    if env::args().len() < 3 {
        println!("Program requires 2 arguments.");
        return;
    } else if env::args().len() > 3 {
        println!("Program requires only 2 arguments.");
        return;
    }

    let arg_file = env::args().nth(1).unwrap();
    let arg_name = env::args().nth(2).unwrap();

    let file = fs::read_to_string(&arg_file).unwrap();

    let mut not_found = false;

    for name in file.lines() {
        if arg_name == name {
            println!("Found {} in the name list.", arg_name);
            return;
        } else {
            not_found = true;
        }
    }

    if not_found == true {
        println!("Could not find {} in the name list.", arg_name);
    }
}

fn main() {
    // cmd_line_arguments();
    // reading_from_files();
    // write_to_file();
    challenge();
}
