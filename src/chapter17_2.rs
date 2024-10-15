#[test]
fn test1() {
    let v = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}


#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
fn init() -> Option<&'static mut Config> {
    let config_box = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    let ptr = Box::into_raw(config_box);
    unsafe {
        config = Some(&mut *ptr);
    }
    Some(unsafe { &mut *ptr })
}

#[test]
fn test2() {
    unsafe {
        init();
        println!("{:?}",config)
    }
}


#[test]
fn test3() {
    let static_string: &'static str = "I'm in read-only memory";
    {
        // Make a `string` literal and print it:

        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    println!("static_string reference remains alive: {}", static_string);
}


// Make a constant with `'static` lifetime.
static NUM: i32 = 18;


// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}
#[test]
fn test4() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}



/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}
#[test]
fn test5() {
    // i is owned and contains no references, thus it's 'static:
    static i: i32 = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}


use std::fmt::Display;
#[test]
fn test6() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string); // Compilation error
    print_d(&string); // Compilation error
    print_e(&string);
    print_f(&string);
    print_g(&string); // Compilation error
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_d(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &(dyn Display + 'static)) {
    println!("{}", t);
}