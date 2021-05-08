use std::io;
//use std::cmp::Ordering;
use colored::*;

fn main() {
  println!("{}{}", "Find factors for any given number with brute force".bright_yellow().bold(), "\n(To quit, press ctrl+c)".bright_red());

  println!("{}", "Please input your number:".bright_green());
  let mut num = String::new();
	io::stdin()
		.read_line(&mut num)
		.expect("Failed to read line");
	
	let num: u32 = num.trim().parse().expect("Please type a number!");

	println!("\n");
	//let mut u64: i = 0;
	for i in 1..num+1 {
		if num % i == 0 {
			println!("{} x {}", num/i, i);
		};
	};
	println!("{}", "All factors have been found".bright_cyan());
}
