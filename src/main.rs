use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn quicksort(list : &mut [i32]) -> usize {
//	println!("\nStarting Sort {:?} len= {}",list,list.len());
	let mut swap_count = 0;
	if list.len() > 1 {
		let mut i = 0;
		swap_count = list.len() - 1;

		// pivot is located at list[0]
		for j in 1..list.len() {
			if list[j] < list[0] {
				i += 1;  // Advance the i pointer to its eventual new location
						 // swap the existing value with the additional less than pivot entry
				//println!("swapping {} {} at i {} j {}", list[i],list[j],i,j);
				let swap = list[i];
				list[i] = list[j];
				list[j] = swap; 
			}
		}
		
	//	println!("\nAfter partition {:?} left: {} right: {} middle {} median {} count= {}",list,list[0],list[list.len()-1], list[i], "tbd",list.len());
	//	println!("swapping pivot {} {} at i {} ", list[0],list[i],i);
		let swap = list[i];
		list[i] = list[0];
		list[0] = swap;
//		println!("\nAfter pivot n{:?} left: {} right: {} middle {} median {} count= {}",list,list[0],list[list.len()-1], list[i], "tbd",list.len());
//		println!("splitting at {} {} Len {}",i, i+1,list.len());
		swap_count += quicksort(&mut list[0..i]);
		if i+1 < list.len() {
			swap_count += quicksort(&mut list[i+1..]);
		}
	
	}
//	println!("Swap count so far.. {}",swap_count);
	return swap_count;

}



fn main() {
    let args: Vec<String> = env::args().collect();

	println!("Args {:?} {}",args,args.len());

	if args.len() < 2 {
        eprintln!("Usage: {} filename", args[0]);
        process::exit(1);
	}

  // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
	let mut digits = Vec::<i32>::new();

	let mut count = 0;
    for line in reader.lines() {
		count += 1;	
		let x : i32 = line.unwrap().parse::<i32>().unwrap();
		digits.push(x);
    }
	println!("Read {} numbers",digits.len());

	let mut digit_copy = vec![0;10];
	digit_copy[0..10].clone_from_slice(&digits[0..10]);
	let cnt = quicksort(&mut digit_copy[..]);
	println!("First 10 result is {}",cnt);

	let mut digit_copy = vec![0;100];
	digit_copy[0..100].clone_from_slice(&digits[0..100]);
	let cnt = quicksort(&mut digit_copy[..]);
	println!("First 100 result is {}",cnt);

	let mut digit_copy = vec![0;1000];
	println!("Len dc1 {}", digit_copy.len());
	digit_copy[0..1000].clone_from_slice(&digits[0..1000]);
	let cnt = quicksort(&mut digit_copy[..]);
	println!("First 1000 result is {}",cnt);

	let mut digit_copy = vec![0;digits.len()];
	println!("Len dc1 {}", digit_copy.len());
	digit_copy[..].clone_from_slice(&digits[..]);
	let cnt = quicksort(&mut digit_copy[..]);
	println!("Full result is {}",cnt);

	let mut digit_copy = vec![0;digits.len()];
	digit_copy[..].clone_from_slice(&digits[..]);
	let swap = digit_copy[0];
	digit_copy[0] = digit_copy[digits.len()-1];
	digit_copy[digits.len()-1] = swap;
	let cnt = quicksort(&mut digit_copy[..]);
	println!("Last as pivot result is {}",cnt);
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn check1() {
			let mut x = vec!(1,2,3,4);
			let _cnt = quicksort(&mut x[..]);
			println!("Count is {}", _cnt);
			assert_eq!(x, vec!(1,2,3,4));

			let mut x = vec!(1,2,4,3);
			let _cnt = quicksort(&mut x[..]);
			println!("Count is {}", _cnt);
			assert_eq!(x, vec!(1,2,3,4));

			let mut x = vec!(2, 20, 1, 15, 3, 11, 13, 6, 16, 10, 19, 5, 4, 9, 8, 14, 18, 17, 7, 12);
			let _cnt = quicksort(&mut x[..]);
			println!("Count is {}", _cnt);
			assert_eq!(x, (1..=20).collect::<Vec<i32>>());
			assert_eq!(_cnt, 66);
/*
			let x = vec!(4,1,2,3);

			let x = vec!(4,3,1,2);

			let x = vec!(9,8,7,6,5,4,3,2,1);

			let x = vec!(11,10,9,8,7,6,5,4,3,2,1);
*/

    }

}
