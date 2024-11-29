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
            1 => addition(),
            2 => subtraction(),
            3 => multiplication(),
            4 => division(),
            5 => break,
            _ => println!("Please input one of the above numbers"),
        }
    }
}

fn addition () {
    println!("What is your first number(to the first decimal)");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("couldnt read line");
    let x: f64 = x.trim().parse().expect("cant parse");

    println!("What is your second number?(to the first decimal)");
    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("couldnt read line");
    let y: f64 = y.trim().parse().expect("cant parse");
}

fn subtraction () {
    println!("what is your first number(to the first decimal)");
let mut x = String::new();
io::stdin()
	.read_line(&mut x)
	.expect("couldnt read line");
let x: f64 = x.trim().parse().expect("couldnt parse");

println!("what is your second number(to the first decimal)");
let mut y = String::new();
io::stdin()
	.read_line(&mut y)
	.expect("couldnt read line");
let y: f64 = y.trim().parse().expect("couldnt parse");
}

fn multiplication () {
    println!("what is your first number(to the first decimal)");
let mut x = String::new();
io::stdin()
	.read_line(&mut x)
	.expect("couldnt read line");
let x: f64 = x.trim().parse().expect("couldnt parse");

println!("what is your second number(to the first decimal)");
let mut y = String::new();
io::stdin()
	.read_line(&mut y)
	.expect("couldnt read line");
let y: f64 = y.trim().parse().expect("couldnt parse");
}

fn division () {
    println!("what is your first number(to the first decimal)");
let mut x = String::new();
io::stdin()
	.read_line(&mut x)
	.expect("couldnt read line");
let x: f64 = x.trim().parse().expect("couldnt parse");

println!("what is your second number(to the first decimal)");
let mut y = String::new();
io::stdin()
	.read_line(&mut y)
	.expect("couldnt read line");
let y: f64 = y.trim().parse().expect("couldnt parse");
}
