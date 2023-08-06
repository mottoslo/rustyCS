pub fn mutability() {
    let mut x = 5;
    // let x = 5; // this will error
    println!("x before change = {}", x);
    x = 6;
    println!("x after change = {}", x);
}

pub fn constants() {
    // let x = 5;
    // let y = 10; // constants cannot be variables
    // const MAXIMUM_CODING_HOURS_ONEDAY :i32 = x + y; // can't be evaluated at runtime
    // MAXIMUM_CODING_HOURS_ONEDAY = 10; // const cannot be changed

    //const can be expressions etc. that can be evaluated at compile time
    //https://doc.rust-lang.org/reference/const_eval.html - things that are allowed
    const MAXIMUM_CODING_HOURS_ONEDAY :i32 = 10 + 5;  // have to explicitly define type for const
    println!("constant value = {MAXIMUM_CODING_HOURS_ONEDAY}");
}

pub fn shadowing() {
    let x = 5; // x = 5

    let x = x + 1; // x = 6

    println!("-------------scope starts with bracket---------");
    {
        let x = x * 2; // x = 12 ( x is shadowed for INNER SCOPE)
        println!("The value of x in the inner scope is: {x}");
    }   //INNER SCOPE ends, shadowed x returns to original value
    println!("-------------scope ends with bracket---------");

    println!("The value of x is: {x}"); // x = 6

    // shadowing is effectively creating a new variable, therefore we can shadow with different type
    // think of shadowing just as new variable within a scope
    let example = "Some string";
    {
        let mut example = example.len();
        example += 1;
        println!("example inside bracket is shadowed with : {}",example)
    }
    println!("example after bracket : {}",example)
}

pub fn datatypes() {
    println!("i8 - min : {} // max : {}", i8::MIN, i8::MAX);
    println!("i16 - min : {} // max : {}", i16::MIN, i16::MAX);
    println!("i32 - min : {} // max : {}", i32::MIN, i32::MAX);
    println!("i64 - min : {} // max : {}", i64::MIN, i64::MAX);
    println!("i128 - min : {} // max : {}", i128::MIN, i128::MAX);
    println!(" ");
    println!("u8 - min : {} // max : {}", u8::MIN, u8::MAX);
    println!("u32 - min : {} // max : {}", u32::MIN, u32::MAX);
    println!("u128 - min : {} // max : {}", u128::MIN, u128::MAX);
    println!(" ");
    println!("f32 - min : {} // max : {}", f32::MIN, f32::MAX);
    println!("f64 - min : {} // max : {}", f64::MIN, f64::MAX);
    //floats must be either f32 or f64
}

pub fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (unpacked_x, unpacked_y, unpacked_z) = tup;
    println!("unpacked x, y, z = {} / {} / {}",unpacked_x,unpacked_y,unpacked_z);
    //direct access
    println!("tup.0 - {}, tup.1 - {}, tup.2 - {}", tup.0, tup.1, tup.2);
}

pub fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}",a);

    //declare type, size of an array
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}",b);

    //initializa array of same elements
    let mut c = [3; 5];  // [3,3,3,3,3]
    c[0] = 7;
    println!("{:?}",c);
}

//this function returns 6
pub fn functions() -> i32{
    //functions return the value of last expression in the function
    {
        //curly brackets are expressions
        5+1 //expressions dont need ending semicolon
    }
}

pub fn controlflow() {
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 100;  // loop can return value with break keyword
        }
    }; // semicolon is there for let keyword (statement)
    println!("first example result : {}", result);

    //loop control
    //if you have multiple loops, you can annotate it and control continue and break manually
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("second example End count = {count}");

    //for loop in arrays
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value for third example is: {element}");
    }

    //for loop for range
    for i in (1..4){
        println!("the value for fourth example is {}",i);
    }
    //for loop for range
    for i in (1..=4){
        println!("the value for fourth example is {}",i);
    }

    //another example of loop labels
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x < 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }
    println!("x = {x}");
}

