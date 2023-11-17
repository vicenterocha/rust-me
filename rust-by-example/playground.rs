// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let bottom_right = Point { x: 5.2, ..point };

    // Destructure the point using a `let` binding
    let Point { x: left_edge1, y: top_e2dge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge1, y: top_e2dge },
        bottom_right: bottom_right,
    };

    println!("pair contains {:?} and {:?}", left_edge1, top_e2dge);
}
