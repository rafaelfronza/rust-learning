use std::mem;

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * &self.width + 2 * &self.height
    }
}

fn generic_struct_definitions() {
    println!(" == generic_struct_definitions == ");
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };
    println!("rect is {:?}", rect);
}

fn generic_method_definitions() {
    println!(" == generic_method_definitions == ");
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
        // height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());

    let rect_two = Rectangle {
        width: 1u8,
        height: 3u8
    };
    println!("perimeter is {}", rect_two.get_perimeter());
}

fn generic_function_definitions() {
    fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }

    println!("biggest is {}", get_biggest(1, 2));
}

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn box_data_type() {
    /*
        Box<T>:
            - Store data on the heap instead of on the stack
            - Considered as Smart Pointer because provides additional functionality beyond references
            - Box<T> has ownership of the data it points to
            - When Box<T> goes out of scope it deallocates the heap memory

        Use Cases:
            - Stpre a type whose size cannot be known at compile time
            - Example: Recursive types
            - Transfer ownership of data rather than copy it on the stack
            - Avoid copying large number of data
    */

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };

    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));
}

fn challenge() {
    fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
        Box::new(*a + *b)
    }

    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed");
}

fn main() {
    generic_struct_definitions();
    generic_method_definitions();
    generic_function_definitions();
    box_data_type();
    challenge();
}
