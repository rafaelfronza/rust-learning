#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
        name: String,
        crew_size: u8,
        propellant: f64
    }

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn defining_structs() {
    /* 
        Struct:
            * Group of multiple items of mixed data types
            * Elements are named
    */
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    // Struct update syntax
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle.clone()
    };

    println!("name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}

fn struct_methods() {
    /*
        Method:
            * Subroutine associated with a struct
            * Can have input parameters and a return value
            * Declared using the "fn" keyword
            * First parameter is a reference to the struct instance
    */
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    let vehicle_name = vehicle.get_name();

    println!("vehicle name is {}", vehicle_name);
}

fn main() {
    defining_structs();
    struct_methods();
}