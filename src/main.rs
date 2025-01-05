// use std::env;
// use std::fs::File;
// use std::io::prelude::*;


fn main() {

//     let args:Vec<String> = env::args().collect();

//     let query = &args[1];
//     let file_name = &args[2];

//     let mut f = File::open("poem.txt").expect("file not found");

//     let mut contents = String::new();

//     f.read_to_string(&mut contents).expect("something went wrong while reading from this file");


// println!("With text:\n{}", contents);
//     // println!("{:?} {:?}",query,file_name);

let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
println!("{:?}",v2);

}
