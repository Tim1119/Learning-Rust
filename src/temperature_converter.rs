use std::io;

pub fn temperature_converter_cel_to_fah() {
    println!("Temperature Converte App");

   let mut room_temperature = String::new();


   io::stdin().read_line(&mut room_temperature).expect("Could not read from standard input");

   let room_temperature:i32 = match room_temperature.trim().parse() {
        Ok(num)=>num,
        Err(_)=>panic!("The temperature could not be converted to a number")
   };

   println!("The value of {:?} in F is {:?} in C",room_temperature,convert_celsius_to_fahrenheit(room_temperature)) 

}


fn convert_celsius_to_fahrenheit(temperature:i32)-> i32{

   let converted_temperature = 5.0/9.0 * ((temperature) - 32);

   converted_temperature

}