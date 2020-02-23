pub struct List<T> {
	head: Option<Box<Node<T>>>,
}

struct Node<T> {
	elem: T,
	next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
	
	pub fn new() -> Self{
		List {
			head: None,
		}
	}
	
	pub fn push(&mut self, elem: T) {
		let new_head= Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_head);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.elem
		})
	}
	
	 pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub fn convert<T>(list: List<T>) -> Vec<T> {
	let mut v = vec!();
	let mut iter = list.into_iter();
	while let Some(item) = iter.next() {
		v.push(item);
	}
	v
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_01() {
		let mut list = List::new();
        list.push(1);
		list.push(2);		
		list.push(3);
		list.push(4);
		let v = convert(list);
		assert_eq!(v, vec!(4,3,2,1));
    }
	
	#[test]
    fn test_find_02() {
		let list = List::new();
		let v : Vec<i32> = convert(list);
		assert_eq!(v, vec!());
    }
}