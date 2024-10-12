
/* Fill in the blanks and Fix the errors */
#[derive(Debug)]
struct Structure(i32);
#[test]
fn test1() {
    // Types in std and Rust have implemented the fmt::Debug trait
    println!("{} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}
#[test]
fn test2() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };

    /* Make it output:
    Person {
        name: "Sunface",
        age: 18,
    }
    */
    println!("{:#?}", person);
}


use std::fmt;

struct Structure3(i32);

struct Deep(Structure3);

impl fmt::Debug for Deep{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.0)
    }
}
#[test]
fn test3() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure3(7)));
}




/* Make it work*/

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}
#[test]
fn test4() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");

    println!("Success!");
}




/* Make it work */

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}
#[test]
fn test5() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
    println!("Success!");
}