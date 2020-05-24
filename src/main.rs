extern crate rand;
extern crate termion;

use rand::Rng;
use clap::Clap;
use termion::{color, style};

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author =  "dicedtomato")]
struct Opts {
    #[clap(short, long)]
    people: i64,
    #[clap(short, long)]
    distance: i64,
    #[clap(short, long)]
    regulation: i64

}
#[derive(Clone)]
struct Person {
    name: i64,
    near: i64,
    near_meters: i64
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut people: Vec<Person> = Vec::new(); 
    let mut rng = rand::thread_rng();
    for name in 0..opts.people {
        let near: i64 = rng.gen_range(0, opts.people);
        let near_meters: i64 = rng.gen_range(0, opts.distance);
        people.push(Person {
            name,
            near,
            near_meters
        });
    }
    let p = people.clone();
    let mut num_people_close = 0;
    println!("{bold}{red}Not Following Regulations:", red = color::Fg(color::Red), bold = style::Bold);
    for person in people {
        if person.near_meters < opts.regulation {
            println!("{bold}{green}Person {}{reset} is close to {bold}{green}Person {}{reset} by {bold}{red}{}m{reset}", person.name, person.near, person.near_meters, red = color::Fg(color::Red), green = color::Fg(color::Green), bold = style::Bold, reset = style::Reset);
            num_people_close = num_people_close + 1;
        }
    }
    println!("{bold}{blue}Following Regulations:", blue = color::Fg(color::Blue), bold = style::Bold);
    for person in p {
        if person.near_meters > opts.regulation {
            println!("{bold}{green}Person {}{reset} is not close to {bold}{green}Person {}{reset} by {bold}{blue}{}m{reset}", person.name, person.near, person.near_meters, blue = color::Fg(color::Blue), green = color::Fg(color::Green), bold = style::Bold, reset = style::Reset);
        }
    }
    println!("\n\n");
    println!("{bold}{blue}Statistics:{reset}", blue = color::Fg(color::Blue), bold = style::Bold, reset = style::Reset);
    println!("{bold}{red}{}%{reset} of {bold}{blue}{}{reset} people are {bold}{red}close{reset} to someone", (num_people_close as f64 / opts.people as f64) * 100 as f64, opts.people, blue = color::Fg(color::Blue), red = color::Fg(color::Red), bold = style::Bold, reset = style::Reset);
    println!("{bold}{green}{}%{reset} of {bold}{blue}{}{reset} people are {bold}{green}not close{reset} to someone", ((opts.people-num_people_close) as f64 / opts.people as f64) * 100 as f64, opts.people, blue = color::Fg(color::Blue), green = color::Fg(color::Green), bold = style::Bold, reset = style::Reset);
}
