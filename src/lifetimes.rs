pub fn lifetimes() {
    let r: &i32;

    {
        let b = 5;
        let r = &b;
    }

    //println!("not valid: {}", r)

    //lifetimes:
    {
        let r;          // ---------+-- 'a
        //          |
        {                     //          |
            let x = 5;   // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
        //          |
        // println!("r: {}", r); //          |
    }                         // ---------+

    //r has a lifetime of 'a but it refers to memory with a lifetime of 'b

    let n1: &i32;
    {
        let n = 5;
        n1 = &n;
    }

    //println!("dangling ref: {}", n1) // borrowed value does not live long enough

/*    let str1 = String::from("not so long");
    let mut r = "ddd";
    {
        let str2 = "longer than the shorter";

        //we need lifetime here to know how long will r live
        r = longer_string(str1.as_str(), str2);
        println!("longer string: {}", r);
    }

    println!("lifetime ended earlier: {}", r);*/

    // ------->

    let b1 = String::from("long string is long");
    let b_result;
    {
        let b3 = String::from("xyz");
        b_result = longer_string(b1.as_str(), b3.as_str());
    }
    //println!("The longest string is {}", b_result); //borrowed value does not live long enough
    //even if b1 will be returned, lifetime of the returned value has shorter value, so it's not valid

    //let's try with another function
    let longer = String::from("longer string");
    let result_string: &str;
    {
        let shorter = String::from("shorter");
        result_string = always_longer(longer.as_str(), shorter.as_str())
    }
    println!("valid here because always returns longer lifetime: {}", result_string)
}

fn always_longer<'a, 'b>(longer: &'a str, shorter: &'b str) -> &'a str{
    longer
}

//if I would want to return longer with return value &'b str this is the error:
//this parameter and the return type are declared with different lifetimes...

//the lifetime of the reference returned by the longest function
//is the same as the smaller of the lifetimes of the references passed in
fn longer_string<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}