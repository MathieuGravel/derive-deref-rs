use std::ops::{Deref, DerefMut};

use derive_deref_rs::Deref;

#[derive(Deref)]
struct IntWrapper(i32);

#[derive(Deref)]
struct MyStruct {
    #[deref]
    string: String,
    _something_else: (),
}

fn main() {
    println!("IntWrapper Example.");
    {
        let int_wrapper = IntWrapper(42);
        println!("int_wrapper contain: {}", *int_wrapper);
        println!("Squared int_wrapper: {}", int_wrapper.pow(2));
    }
    println!();
    println!("MyStruct Example.");
    {
        let mut my_struct = MyStruct {
            string: String::from("Hello World"),
            _something_else: (),
        };
        println!("my_struct {}", my_struct.deref());
        let string = my_struct.deref_mut();
        string.replace_range(6.., "Derive Deref");
        println!("my_struct {}", my_struct.deref());
    }
}
