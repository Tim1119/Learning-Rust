// // mod guessing_game;
// // mod temperature_converter;
// mod first_word;
// mod rust_struct;
// use std::collections::HashMap;

// // use guessing_game::play_guessing_game;
// // use temperature_converter::temperature_converter_cel_to_fah;
// use first_word::get_first_word;
// use rust_struct::structs;
// use std::{fs::File};
// use std::io::{Error, Read};
// use std::io::ErrorKind;

// use std::fmt::Debug;


// core::cmp::PartialOrd

// use std::fmt::Display;

struct Pair <T>{
    x:T,
    y:T
}

// impl <T> Pair<T>{
//     fn new(x:T,y:T)->Self{
//         Self{x:x,y:y}
//     }
// }

// impl <T:Display + PartialOrd> Pair<T>{
//     fn   cmp_display(&self) {
//             if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//             } else {
//             println!("The largest member is y = {}", self.y);
//             }
//     }
// }

// impl <T:Display> toString(){

// }

    
fn main() {

    let s: &'static str = "I have a static lifetime.";

    let mut r =3;

    {
        let x =4;
        let r = x;
    }

    println!("The valeu of r is {r}")
    
        // let number_list = vec![34, 50, 25, 100, 65];
        // let result = largest(&number_list);
        // println!("The largest number is {}", result);
        // let char_list = vec!['y', 'm', 'a', 'q'];
        // let result = largest(&char_list);
        // println!("The largest char is {}", result);



}

// fn largest<T:PartialOrd>(list:&[T])->&T{
//     let mut largest_value = list[0];

//     for &i in list.iter(){

//         if i > largest_value{
//             largest_value = i;
//         }
//     }

//     largest_value

// }


#[test]
fn hello_world(){
    assert_eq!(3,3)
}