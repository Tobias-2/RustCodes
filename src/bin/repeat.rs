use test4::repeat::repeat_string;

fn main() {
	let text = "jo".to_string();
	let n: usize = 7;
	let result = repeat_string(&text, n);
	println!("{text} x {n} -> {result}");
}