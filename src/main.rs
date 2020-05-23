mod references;
mod references2;
mod dangling_ref;
mod lifetimes;

use references::reference;
use references2::ref_2;
use dangling_ref::dang;
use lifetimes::lifetimes;

fn main() {
    //empty stack convention
    //full stack convention

    //ascending stack
    //descending stack

    //The most common kind of pointer in Rust is a reference
    //References are indicated by the & symbol and borrow the value they point to

    //One example that we’ll explore in this chapter is the reference counting smart pointer type.
    // This pointer enables you to have multiple owners of data by keeping track of the
    // number of owners and, when no owners remain, cleaning up the data

    //references are pointers that only borrow data; in contrast,
    // in many cases, smart pointers own the data they point to

    //this is on stack
    let x = 5; //each value has exactly one owner
    let y = x; //this is also on stack, it's a copy
    //integer implements Copy trait
    //we can still use x here
    //Drop and Copy traits can be used together

    println!("here is x: {}", x);

    {
        let z = 6; //stack
        let v = z; //copy
    } //both gets dropped, drop() called

    let a = String::from("aga"); //mutable, on heap
    //we are allocating string on heap and its (pointer, length and capacity) on stack
    //we only "allocate" on heap, not on stack
    //copy of pointer, length and cap === shallow copy
    //copy of value === deep copy

    let b = a; //move
    //move === shallow copy + invalidation of the first value
    //if Rust would not move, we would have double drop

    //println!("a is invalid here: {}", a); ERROR: value borrowed here after move

    //ownership is about returning memory
    //Rust will never automatically create “deep” copies of your data

    let a1 = String::from("a");
    let b1 = a1.clone(); //deep copy

    //functions will work the same as assignments
    let m = 2;
    print_m(m);
    println!("still valid here because of copy trait: {}", m);

    let n = String::from("aga");
    print_n(n); //moved here
    //println!("not valid: {}", n); // ERROR: value borrowed here after move

    let o = String::from("o");
    let o2 = takes_and_returns(o); //o gets invalidated, ownership moved

    reference();
    ref_2();
    dang();
    lifetimes();
}

fn print_m(m: i32) {
    println!("I'm the second owner of: {}", m);
}

fn print_n(n: String) {
    println!("now I'm the owner of: {}", n);
}

fn takes_and_returns(o: String) -> String {
    o //takes ownership and gives it back
}