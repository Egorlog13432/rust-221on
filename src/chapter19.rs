use std::fmt;

/* Define the Wrapper type */
struct Wrapper(Vec<String>);

// Display is an external trait
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
#[test]
fn test1() {
    // Vec is an external type, so you cannot implement Display trait on Vec type
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}


/* Make it workd */
struct Meters(u32);

impl Meters {
    // Создаем метод, который позволяет возводить в степень
    fn pow(&self, exp: u32) -> u32 {
        self.0.pow(exp) // используем метод pow у u32
    }
}
#[test]
fn test2() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    // The `pow` method is defined on `u32` type, we can't directly call it
    assert_eq!(n.pow(2), 4);
}


/* Make it work */
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// An age verification function that checks age in years, must be given a value of type Years.
fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}
#[test]
fn test3() {
    let age = Years(5);
    let age_days = age.to_days();

    // Проверка возраста с использованием типа Years
    println!("Old enough: {}", old_enough(&age));

    // Если хотите проверить возраст в днях, конвертируйте его обратно в годы
    let age_in_years = age_days.to_years();
    println!("Old enough: {}", old_enough(&age_in_years));
}


use std::ops::Add;
use std::fmt::{self, format};

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}
#[test]
fn test4() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}",d), "There are still 30 meters left");
}

/* Implement calculate_distance  */
fn calculate_distance(a: Meters, b: Meters) -> Meters {
    a + b
}


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

/* Fill in the blank */
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
#[test]
fn test5() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}

enum VeryVerboseEnumOfThingsToDoWithNumbers2 {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers2 {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            VeryVerboseEnumOfThingsToDoWithNumbers2::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers2::Subtract => x - y,
        }
    }
}
#[test]
fn test6(){}


/* Make it work with const generics */
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}
#[test]
fn test7() {
    let arr = my_function::<5>();
    println!("{:?}", arr);
}


/* Make it work with slice references */
#[test]
fn test8() {
    let s: &str = "Hello there!";

    let arr: [u8; 3] = [1, 2, 3];

    let slice: &[u8] = &arr;
    println!("String: {}", s);
    println!("Array: {:?}", arr);
    println!("Slice: {:?}", slice);
}

#[test]
/* Make it work in two ways */
use std::fmt::Display;
fn foobar(thing: &dyn Display) {
    println!("{}", thing);
}

fn main() {
    foobar(&42);
    foobar(&"Hello, world!");
}

