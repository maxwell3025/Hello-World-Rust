struct Integer(i32);

fn main() {
    let mut a = 0;
    let b = &mut a;
    *b = 1;
    println!("a: {}", a);
    let mut c = Integer(0);
    let d = &mut c;
    *d = Integer(1);
    println!("d: {}", d.0);
}
