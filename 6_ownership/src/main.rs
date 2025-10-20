fn variable_scope() {
    let planet = "Earth";
    if true {
        // let planet = "Earth";
        println!("planet is {planet}");
    }
    // println!("planet is {planet}");
}

fn shadowing_variables() {
    let planet = "Earth";
    {
        println!("planet is {planet}");
        // let planet = "Mars";
        let mut planet = 4;
        println!("planet is {planet}");
    }
    println!("planet is {planet}");
}
/*
fn stack_and_heap_memory() {
    // Stack and Heap Memory
    /*
    Stack:
        * Values stored in sequential order
        * Data added and removed as last in, first out (LIFO)

        + Push and Pop data very quickly
        - Access data very quickly
        - Small size (Determined by the compiler)
        - All data must have a known, fixed size
        --> We may not know how much memorry will be needed by our program at compile time

    Heap:
        * Adding and accessing data is slower than the stack
        * Dinamycally add and remove data
        * Store large data structures in the Heap memory
    */
}
*/
fn string_data_size() {
    /*
    String Literal:
        - Hardcoded into the executable (eg. "hello")
        - Immutable
        - Must be known before compilation
    String Type:
        - Allocated on the heap
        - Mutable
        - Dynamically generated at runtime
    */
    let mut message = String::from("Earth");
    println!("message is {message}");
    message.push_str(" is home.");
    println!("message now is {message}");
}
/*
fn ownership() {
    /*
    Heap Memory has a plenty of space, but not infinite space.
    If keep adding things to the heap, without cleaning it, it will run out of space and memory.

    Explicit Allocation and Deallocation
        * Programmer is responsible for memory management
            -- e.g.: C/C++ have malloc() and free()
        + Programmer has a lot of control
        - Can have memory leaks
        - Invalid memory access
    
    Garbage Collection
        * Garbage collector automatically cleans up memory
        + Easy
        - Wasteful of memory
        - Can run ar inconvenient times

    = Ownership =
        * Variables are responsible for freeing their own resources
        * Rules:
            1 - Every value is "owned" by one, and only one, variable at time
            2 - When the owning variable goes out of scope, the value is dropped
        + Safe
        + Efficient
        - Requires understand of ownership
    */
}
*/
fn moving_cloning_and_copying_data() {
    // MOVE - Transfering the ownership
    let outer_planet: String; // stored in stack memory
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {inner_planet}");
        outer_planet = inner_planet;
    } // stored in heap memory
    println!("outer_planet is {outer_planet}");

    // CLONE
    let clone_outer_planet: String; // stored in stack memory
    {
        let mut clone_inner_planet = String::from("Mercury");
        clone_outer_planet = clone_inner_planet.clone(); // duplicate string data
        clone_inner_planet.clear();
        println!("inner_planet is {clone_inner_planet}");
    } // stored in heap memory
    println!("outer_planet is {clone_outer_planet}");

    // COPY
    /*
        * Only done for stack-only data types (integer and floating point)
        * Copying occurs implicitly; cloning must be done explicitly
    */
    let copy_outer_planet: i32; // stored in stack memory
    {
        let mut copy_inner_planet: i32 = 1;
        copy_outer_planet = copy_inner_planet;
        copy_inner_planet += 1;
        println!("inner_planet is {copy_inner_planet}");
    } // stored in heap memory
    println!("outer_planet is {copy_outer_planet}");
}

fn transferring_ownership_one() {
    fn to_main() {
        let rocket_fuel = 1;
        process_fuel(rocket_fuel);
        println!("rocket_fuel is {rocket_fuel}");
    }

    fn process_fuel(propellant: i32) {
        println!("processing propellant {propellant}...");
    }

    to_main();
}

fn transferring_ownership_two() {
    fn to_main() {
        let rocket_fuel = String::from("RP-1");
        let rocket_fuel = process_fuel(rocket_fuel);
        println!("rocket_fuel is {rocket_fuel}");
    }

    fn process_fuel(propellant: String) -> String {
        println!("processing propellant {propellant}...");
        let new_fuel = String::from("LNG");
        new_fuel
    }

    to_main();
}

fn main() {
    variable_scope();
    shadowing_variables();
    string_data_size();
    moving_cloning_and_copying_data();
    transferring_ownership_one();
    transferring_ownership_two();
}
