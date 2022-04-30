struct Point<T>{
	x: T,
	y: T,
}

fn main() {
	let mut p = Point{x: 1,y: 1};
	p.x = 2;
	println!("({},{})",p.x,p.y);
}
