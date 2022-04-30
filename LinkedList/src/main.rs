use std::fmt;
use std::ptr;

struct Node{
	next: *mut Node,
	value: i32
}

impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct LinkedList{
	head: *mut Node,
	length: i32
}

impl LinkedList{
	fn new() -> LinkedList{
		LinkedList{head: ptr::null::<Node>() as *mut Node, length:0}
	}
	fn push(&mut self, value:i32){
			let new_node: *mut Node = Box::into_raw(Box::<Node>::new(Node{next: self.head, value: value}));//pointer to new head
			self.head = new_node;
			self.length+=1;
	}
	fn peek(&self, index:i32) -> i32{
		if index >= self.length || index < 0{
			panic!("index out of bounds: {}", index);
		}
		unsafe{
			let mut current_node = self.head;
			for _i in 0..index{
				current_node = (*current_node).next;
			}
			(*current_node).value
		}
	}
}

impl fmt::Display for LinkedList{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "[".to_owned();
        for _i in 0..self.length{
        	if _i != 0{
        		out += " -> ";
        	}
        	out += format!("{}", self.peek(_i)).as_str();
        }
        out += "]";
        write!(f, "{}", out)
    }
}

fn main() {
	let mut LL = LinkedList::new();
	LL.push(1);
	LL.push(2);
	LL.push(3);
    println!("The list is: {}", LL);
}
