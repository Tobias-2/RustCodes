//second: write a function that finds the second largest element of a collection where the elements of 
//the collection can have any type.



pub fn second_biggest<T: std::cmp::PartialOrd>(collection: &[T]) -> &T {
	
	//new vector
	let mut item = Vec::new();
	
	for i in collection {
		item.push(i);
	}
	
	//sorting 
	for i in 0..item.len() {
		for j in 0..item.len() {
			if item[i] < item[j] {
				item.swap(j,i);
			}
		}
	}
	
	//removing duplicates
	let mut singels = Vec::new();
	
	for i in 0..item.len() - 1 {
		if item[i] != item[i + 1] {
			singels.push(item[i]);
		}
	}
	
	//adding last element
	singels.push(item[item.len() - 1]);
	
	
	//taking second biggest item
	let result = singels[singels.len() - 2];
	
	//returning
	result
}