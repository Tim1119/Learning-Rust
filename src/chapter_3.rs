fn main(){

    let x:String = String::new("Hello world");
    let j = &x[0..5];
    println!("{j}")
}