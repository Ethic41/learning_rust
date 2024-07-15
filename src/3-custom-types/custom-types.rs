/* =====[Bismillahirrahmanirrahim]=====
 * -*- coding: utf-8 -*-
 * @Date    : 2024-07-15 13:42:02
 * @Author  : Dahir Muhammad Dahir (dahirmuhammad3@gmail.com)
 * @Link    : link
 * @Version : 1.0.0
 */


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
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Dahir");
    let age = 27;
    let dahir = Person {name, age};

    // Print debug struct
    println!("{:?}", dahir);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };
    
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = rect;

        (x2 - x1) * (y2 - y1)
    }

    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };

    println!("Rectangle area: {}", rect_area(rect));

    fn square(point: Point, size: f32) -> Rectangle {
        let Point { x, y } = point;

        Rectangle {
            top_left: Point { x, y: y + size },
            bottom_right: Point { x: x + size, y },
        }
    }

    let point = Point { x: 0.0, y: 0.0 };
    let size = 10.0;

    let square = square(point, size);

    println!("Square: {:?}", square.top_left.x);

}