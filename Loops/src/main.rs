fn main() {
	let k = {
		let mut sum:i64 = 1;
    	for i in 1..21{
     		sum*=i;
     	}
    	sum
	};
	println!("{}",k);
	println!("{}",9223372036854775807i64);
}
