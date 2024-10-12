use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.x)
    }
}
#[test]
fn test1() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");
}


// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;
#[test]
fn test2() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}


use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point2 {
    x: i32,
    y: i32
}

impl FromStr for Point2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point2 { x: x_fromstr, y: y_fromstr })
    }
}
#[test]
fn test3() {
    // FILL in the blanks in two ways
    // DON'T change code anywhere else
    let p = "(3, 4)".parse::<Point2>();
    assert_eq!(p.unwrap(), Point2{ x: 3, y: 4} );

    println!("Success!");
}