//repeat: write a function that repeats a string n times, thus producing a new string.



pub fn repeat_string(text: &str, n: usize) -> String {
	                                                                                                                      
	let mut result = String::new();
	
	let mut count = 0;
	
	loop {
		count += 1;
		
		result.push_str(text);
		
		if count == n {
			break;
		}
	}
	
	result
}



