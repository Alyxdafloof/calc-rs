//Calculator program for Rust//

//Libraries used//
use std::io;

fn main() {

    loop {
        println!("What operation are you using?");
        println!("1.) Addition");
        println!("2.) Subtraction");
        println!("3.) Multiplication");
        println!("4.) Division");
        println!("5.) Exit");

        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("couldnt read line");
        let op = op.trim().parse().expect("Could not parse");

        match op {
            5 => break,
            _ => println!("Please input one of the above numbers"),
        }
    }
}
