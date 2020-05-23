pub fn ref_2() {
    let o = 2; //value on stack
    let i = o; //i32 implements Copy trait, so it's a copy, new value on stack

    let mut a = String::from("a"); //(pointer, length, capacity) on stack --> "a" on heap
    //first problem: when to release memory
    //second: double free
    let mut b = a; //this is moving, a is no longer valid
    //this will create new (pointer, length, capacity) on stack

    let mut a1 = 1;
    let mut b1 = &a1; //reference, a1 is still valid
    //we don't create new (pointer, length, capacity), only pointer to the old one

    //REFERENCE:
    //pointer --> (pointer, length, capacity) --> heap

    //MOVING:
    //(pointer, length, capacity) --> heap
    //                               /\
    //(pointer, length, capacity)   /

    //what will happen if we will drop the memory and reference will try to acccess it?

    let mut c1 = String::from("c1");
    println!("c1 before borrowing: {}", c1);
    {
        let c2 = &mut c1;
        c2.push_str(": editing borrowed"); //so I can easily edit borrowed but in different scope
    }
    //c1 = String::from("kk");
    //println!("c2 after c1's edit: {}", c2); //cannot assign to `c1` because it is borrowed

    println!("c1 after borrowing: {}", c1); //cannot borrow `c1` as immutable because it is also borrowed as mutable
    //c2.push_str("editing borrowed");
    //println!("c1 after editing: {}", c1);

    //always only one var can edit value on heap
}