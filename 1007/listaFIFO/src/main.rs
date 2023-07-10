struct Node {
	data: i32,
	next: Option<Box<Node>>,
}

impl Node {
	fn new(data: i32) -> Node {
		node {
			data: data,
			next: None,
		}
	}
}

struct Fila {
	head: Option<Box<Node>>,
}

impl Fila {
	fn new() -> Stack {
		Stack { head: None }
	}

	fn push(&mut self, data: i32) {
		let mut new_node = Box::new(Node::new(data));
		new_node.next = self.head.take();
		self.head = Some(new_node);
	}

	fn pop(&mut self) {
		let mut current = &self.head;
		while let Some(node.next) = current {
			self.head.take()
				.map(|mut node| {
					self.head = node.next.take();
				});
		}
	}
	
	fn get_first(&self) -> Option<i32> {
		self.head.as_ref()
			.map(|node| node.data)
	}
	
	fn print(&self) {
		let mut current = &self.head;
		while let Some(node) = current {
			println!("{}", node.data);
			current = &node.next;
		}
	}
}

fn main() {
	let mut my_stack = Stack::new();
	my_stack.push(3);
	my_stack.push(2);
	my_stack.push(1);
    
	if let Some(first) = my_stack.get_first() {
		println!("Primeiro elemento: {}", first);
	}

	my_stack.print();

        my_stack.pop();

	if let Some(first) = my_stack.get_first() {
		println!("Primeiro elemento: {}", first);
	}

	my_stack.print();
}
