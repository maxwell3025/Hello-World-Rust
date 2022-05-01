use std::fmt;

struct Node<T>{
	next: Option<Box<Node<T>>>,
	data: T
}

struct List<T>{
	head: Option<Box<Node<T>>>
}

impl<T> List<T>{
	fn new() -> Self{
		return List{head: None}
	}

	fn push(&mut self, data: T){
		let old_head = self.head.take();
		self.head = Some(Box::new(Node{next: old_head, data: data}));
	}

	fn pop(&mut self) -> Option<T>{
		match self.head.take(){
			Some(head)=>{
				self.head = (*head).next;
				Some((*head).data)
			},
			None=>{
				None
			},
		}
	}
}

impl<T: Copy> List<T>{
	fn peek(&self) -> Option<T>{
		match &self.head{
			Some(head)=>{
				Some(head.data)
			},
			None=>{
				None
			}
		}
	}
}
impl<T: fmt::Display> fmt::Display for List<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "[".to_owned();
		let mut next = &self.head;
        while let Some(node) = next{
        	out += format!("{}", node.data).as_str();
			next = &node.next;
			if !matches!(next, None){
				out += "->"
			}
        }
        out += "]";
        write!(f, "{}", out)
    }
}

fn main() {
	let mut ll = List::new();
	ll.push(1);
	ll.push(2);
	ll.push(3);
    println!("The list is: {}", ll);
	if let Some(x) = ll.peek(){
		println!("{}", x);
	}
}
