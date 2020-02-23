
static mut QUEUE: Vec<char> = Vec::new();
static mut CHARS: [i32;128] = [0; 128];


unsafe fn insert(c: char) {
	let i: usize = c as usize;
	if CHARS[i] == 0 {
		QUEUE.push(c);
		CHARS[i] += 1;
	}
}


unsafe fn first_appearing_once() -> char {
	while let Some(c) = QUEUE.last() {
		let i: usize = *c as usize;
		if CHARS[i] == 1{
			return *c;
		}
		QUEUE.pop();
	}
	'#'
}