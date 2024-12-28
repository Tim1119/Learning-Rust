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

pub trait Summary{

    fn summarize(&self)->String;

    fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    fn summarize_author(&self) -> String ;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{

    fn summarize(&self)->String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    // format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
        }

    }
    
fn main() {


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
        };

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
            };
            println!("New article available! {}", article.summarize());
        println!("1 new tweet: {}", tweet.notify());
        // println!("1 new tweet: {}", tweet.summarize());


}