use std::any;
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32
}

// trait Description {
//     fn describe(&self) -> String;
// }

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {
    // fn describe(&self) -> String {
    //     format!("the {} flying at {} miles per second!", self.name, self.velocity)
    // }

}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flying {} miles high with {} crew member onboard!", self.name, self.altitude, self.crew_size)
    }
}



fn implement_traits() {
    /*
        - a collection of methods
        - Data Types can implement a trait
        - Generics use Traits to specify the capabilities of unknown data types
        - similar to interfaces in other programming languages
    */
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}

// fn default_trait_implementation() {
    
// }

#[derive(PartialEq, PartialOrd)]
struct SatelliteOne {
    name: String,
    velocity: f64 // miles per second
}

fn derive_traits() {
    let hubble = SatelliteOne {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };

    let gps = SatelliteOne {
        name: String::from("GPS"),
        velocity: 2.42
    };

    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}

fn trait_bounds() {
    /*
        - Require a generic type to implement specific traits
        - Guarantees the generic type will have necessary behaviors
    */
    // fn print_type<T: fmt::Display>(item: T) {
    fn print_type<T: fmt::Debug>(item: T) {
        println!("{:?} is {}", item, any::type_name::<T>());
    }

    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type([13]);
}

fn multiple_trait_bounds() {
    // fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
    fn compare_and_print<T, U>(a: T, b: U)
        where T: fmt::Display + PartialEq + From<U>,
              U: fmt::Display + PartialEq + Copy 
    {
        if a == T::from(b) {
            println!("{} is equal to {}", a, b);
        } else {
            println!("{} is NOT equal to {}", a, b);
        }
    }

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}

fn return_types_with_implemented_traits() {
    // 
}

fn main() {
    implement_traits();
    // default_trait_implementation();
    derive_traits();
    trait_bounds();
    multiple_trait_bounds();
    return_types_with_implemented_traits();
}
