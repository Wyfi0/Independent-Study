
// Hide unused code errors at compile
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String, // set name as a string
    age: u8, // set age as 8-bit unsigned ineger
}

// Declare empty struct "Unit"
struct Unit;

// A tuple struct 
struct Pair(i32, f32); // Declare struct with 32-bit integer and 32-bit floating-point number

// Struct with two fields, x and y
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct 
struct Rectangle {
    // A rectangle can be derived from top left and bottom right points
    top_left: Point, // the top left as a point with two coordinates
    bottom_right: Point, 
}

// now the actual program
fn main() {
    // create struct with field creation shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age }; // take those vars and use them in a struct named peter
    
    // print the struct with debug info (thats the :?)
    println!("Debug {:?}", peter);
    //println!("{}", peter); you cant just print it
    
    // Intantiate a "Point" called point from the Point struct
    let point: Point = Point { x: 10.3, y: 0.4};

    // print the coords of the point
    println!("point coords: ({}, {})", point.x, point.y); // YESSSSSSS! accesed with .x or .y

    // Make a new point by using struct update syntax to use the fields of the other one
    let bottom_right = Point { x: 5.2, ..point }; // ..point means range to point, in this case the
                                                  // second feild
    
    // Print the new bottom_right
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let _rectangle = Rectangle {
        // struct intantation is an expression also
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

}

