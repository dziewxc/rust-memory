use std::string::String;

pub fn reference() {
    let mut r = String::from("heap");
    r_fun(&mut r);
    //what is mutable reference changing in the memory? pointer -> pointer -> heap
    //I guess we are just creating new value on the heap and it's still easy to say when should we release memory
    println!("still have access to r: {}", r);

    let mut var = String::from("lemme"); //var has to be mutable to create mut&
    //we can only have one mut reference in one scope:
    let mut_ref1 = &mut var;
    let mut_ref2 = &mut var; //second mutable borrow occurs here

    println!("not a bug: {}", mut_ref2); //this works
    //println!("a bug: {}", mut_ref1); //cannot borrow `var` as mutable more than once at a time
    //why error is only when I try to print mut_ref1? --> because till the usage compiler doesn't know where scope ends

    //that's how we can avoid data races - accessing and writing to the same memory by two vars

    //we can create more than one mut ref in different scopes
    let mut p1 = 5;
    {
        let p2 = &mut p1;
        println!("the first mut ref: {}", p2);
    }

    let p3 = &mut p1;
    println!("in different place we have p3 accessing it: {}", p3);

    //we also can't have mutable references when we have immutable one
    //we don't want the value to change while accessing from immutable

    //scope of the reference ends with the last occurrence (interesting...)
    //that's why we can do this:
    let mut h1 = String::from("h1");
    let h2 = &mut h1;
    println!("I'm the last occurrence of: {}", h2);

    let h3 = &mut h1;

    println!("now I can be the only mut one! {}", h3);
}

fn r_fun(s: &mut String) { //s is a pointer to (pointer, len, cap) of r
    //s is not owner of the value, nothing happens after end of scope
    println!("we can use reference here: {}", s);

    s.push_str("yaaas");
    //the reference is mutable, so we can edit it

    //borrowing -> passing variable as reference
}