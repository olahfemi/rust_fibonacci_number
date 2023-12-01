// use std::io;

// fn main() {
// 	// let mut index = String::new();
// 	// io::stdin()
// 	// 	.read_line(&mut index)
// 	// 	.expect("Failed to read line");

// 	// let index: i32 = index.trim().parse().expect("Index is not an integer");
// 	// let mut a = 0;
// 	// let mut b = 1;

// 	// if index == 0 {
// 	// 	println!("{a}");
// 	// } else if index == 1 {
// 	// 	println!("{b}");
// 	// } else {
// 	// 	for i in 2..index+1 {
// 	// 	let c = a + b;
// 	// 	a = b;
// 	// 	b = c;
// 	// 	println!("{b}");
// 	// 	} 
// 	// }
// 	let e = fib();
// 	println!("{e}");
	
// }

// fn fib() -> i32 {
// 	let mut index = String::new();
// 	io::stdin()
// 		.read_line(&mut index)
// 		.expect("Failed to read line");

// 	let index: i32 = index.trim().parse().expect("Index is not an integer");
// 	let mut a = 0;
// 	let mut b = 1;

// 	let d = if index == 0 {
// 		return a;
// 	} else if index == 1 {
// 		return b;
// 	} else {
// 		let mut i: i32 = 2;
// 		loop {
// 			for mut i in 2..index+1 {
// 			let i = i + 1;
// 			let c = a + b;
// 			let a = b;
// 			let b = c;
			
// 		} 
// 		if i == index {
// 			break b;
// 		}
// 	}
// 		// return a;
// 	};
// 	return d;
// }

use std::io;
fn main () {
   println!("To end the program, type `exit` ");
   loop {
       println!("Type a positive integer");
       let mut int = String::new();
        io::stdin()
       .read_line(&mut int)
       .expect("");
       if int.trim() == "exit"{
           break;
       }
       let int: u32= match int.trim()
       .parse() {
           Ok(int) => int,
           Err(_) => continue,
       };
       println!("Fibonacci ({}) => {}", int, fib(int));

   }

}

fn fib (n: u32) -> u32 {
   if n <= 0 {
       return 0;
   } else if n == 1 {
       return 1;
   }   fib(n - 1) + fib(n - 2)
}