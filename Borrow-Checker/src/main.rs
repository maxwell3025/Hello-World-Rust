struct NamedStruct<'a>{
    reference: &'a i32
}

impl<'a> NamedStruct<'a>{
    pub fn new(x: &'a i32) -> Self{
        NamedStruct{
            reference: x
        }
    }
}

fn main() {
    let myInt: & i32 = &420;
    let mut other2 = myInt;
    other2 = &132552;
    println!("other: {}", other2);
    let mut thing = NamedStruct::new(&myInt);
    let other = &myInt;
}
