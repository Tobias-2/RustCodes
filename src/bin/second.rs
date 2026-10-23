use test4::second::second_biggest;

fn main() {
	let array = [4, 6, 7, 8, 3, 6, 66, 773, 773, 33, 22];
	println!("Second biggest number of collection {:?} is {}", 
		&array, second_biggest(&array));
}