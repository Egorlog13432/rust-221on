#[test]
fn test1() {
    let i = 3;
    {
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐
        //                                                      │
        println!("borrow1: {}", borrow1); //                    │
    } // `borrow1 ends. ────────────────────────────────────────┘
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}

#[test]
fn test2() {
    {
        let r;                // ---------+-- 'a
                                    //          |
        {                           //          |
            let x = 5;         // -+-- 'b  |
            r = &x;                 //  |       |
        }                           // -+       |
                                    //          |
        println!("r: {}", r);       //          |
    }                               // ---------+
}


/* Make it work by adding proper lifetime annotation */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[test]
fn test3() {
    let x: &str ="long";
    let y: &str = "longer";

    println!("{}", longest(x, y));
}


// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn invalid_output() -> String {
    String::from("foo")
}
#[test]
fn test4() {
    let x: String = invalid_output();

    println!("{}", x);
}



// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* Make it work */
// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow() {
    let _x: i32= 12;

    // ERROR: `_x` does not live long enough
    let y: &i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than `'a` . A short lifetime cannot be coerced into a longer one.
}
#[test]
fn test5() {
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}


/* Make it work by adding proper lifetime annotation */

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a > {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
#[test]
fn test6() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}



/* Make it work */

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example1<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}
#[test]
fn test7()
{
    /* 'a tied to fn-main stackframe */
    let var_a = 35;
    let example: Example1;

    {
        /* Lifetime 'b tied to new stackframe/scope */
        let var_b = NoCopyType {};

        /* fixme */
        example = Example1 { a: &var_a, b: &var_b };
        println!("(Success!) {:?}", example);
    }
}




#[derive(Debug)]
struct NoCopyType1 {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType1
}

/* Fix function signature */
fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType1
{ foo.b }
#[test]
fn test8()
{
    let no_copy = NoCopyType1 {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}


/* Make it work by adding proper lifetime annotations */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl ImportantExcerpt<'_> {
    fn level<'a>(&'a self) -> i32 {
        3
    }
}
#[test]
fn test9() {}


fn input(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 { x }

fn longest1<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) { self.0 += 1; }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either1<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
