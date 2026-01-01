// use chrono::{DateTime, Local, Utc};

// fn main() {
//     let utc: DateTime<Utc> = Utc::now();
//     println!("{}",utc);
//     println!("{}",Local::now());
// }



// fn main() {
//     let x: i32 = 5;
//     let is_male = true;
//     let mut greet = String::from("hello");
//     let name: [i32;5] = [1,2,3,4,5];
//     let xs = vec![1,2,3,4,5];
//     println!("{:?}",xs.len());
//     let length = calculate_length(&greet);
//     println!("greet is {}",greet);
//     println!("length of greet is {}",length);
//     let new_length = append_text( &mut greet);
    
//     println!("new length of greet is {}",new_length);
// }


// fn calculate_length(s: &String) -> usize {

//     s.len()
// }

// fn append_text(s: &mut String) -> &String {

//     s.push_str(", World");
//     return s;

// }


// structs 

// struct User {
//     active: bool,
//     username: String,
//     email:String,
//     sign_in_count: u64
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("john"),
//         email: String::from("john@example.com"),
//         sign_in_count: 1
//     };
//     println!("User 1 username:{:?}", user1.username);   
// }
// 
// struct Rect {
    // width: u32,
    // height:u32,
// }
// impl Rect {
    // fn area(&self) -> u32 {
        // self.width * self.height
    // }
// }
// 
// fn main() {
    // let rect = Rect {
        // width: 30,
        // height: 20,
    // };
    // println!("area of rect is {}", rect.area());
// }
// 
// enum Direction {
    // North,
    // East,
    // South,
    // West,
// }
// 
// fn main() {
    // let my_direction = Direction::North;
    // let new_directtion = my_direction;
// }
// 
// 

// define an enum called Shape


// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64,f64),
// }

// fn calculate_area(shape: Shape) -> f64 {
//     match shape {
//         Shape::Circle(radius) => return radius * radius * 3.14,
//         Shape::Square(side) => return side * side,
//         Shape::Rectangle(width,height) => return width * height,
//     }
// }

// fn main() {
//     let circle = Shape::Circle(10.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0,5.0);
//     println!("area of circle is {}",calculate_area(circle));
//     println!("area of square is {}",calculate_area(square));
//     println!("area of rectangle is {}",calculate_area(rectangle));  
// }

// Error handling 


// use std::fs;

// fn main() {

//     let greeting_file_results = fs::read_to_string("greeting.txt");


//     match greeting_file_results {
//         Ok(file_content) => {
//              println!("file read successfully: {:?}", file_content);
//         },
//         Err(e) => {
//             println!("error reading file: {:?}", e);
//         }
// }
// }

// learn about custom errors 


// Option enum  = used to handle optional values 

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'a' {
//             return Some(index as i32);
//         }
//      }
//      None
// }

// fn main() {
//     let my_string = String::from("raman");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string"),
//     };
// }


// GENERICS AND TRAIT BOUNDS Generics in Rust are a way to write flexible, reusable, and type-safe code. They allow you to define functions, structs, enums, or methods that can operate on different types of data, without knowing the exact types ahead of time. Instead of specifying a concrete type, you can use a placeholder that will be replaced by a specific type when the code is compiled.
// create a first_element function
// pub fn main() {
    // let v = vec![1,2,3];
    // let v2 = vec![String::from("yuvraj"), String::from("Pandey")];
    // let v3 = vec![1.0,2.0,3.0];
    // println!("{}",first_element(v).unwrap());
    // println!("{}",first_element(v2).unwrap());
    // println!("{}",first_element(v3).unwrap());
// }
// 
// fn first_element<T>(v: Vec<T>) -> Option<T> {
    // return v.into_iter().nth(0);
// }
// 



// Traits similart to interfaces in ts, let you define shape/ interface of what you'r buildinig

// trait Shape {
//     fn area(&self) -> f64;
// }


// struct Rect {
//     width: f32,
//     height: f32,
// }

// impl Shape for Rect {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }



// // you can defina a function that takes in any struct that implements this trait

// fn get_area(shape: impl Shape) -> f64 {
//     shape.area()
// }

// fn get_area<T: Shape> (shape: T) -> f64 {
//     shape.area()
// }