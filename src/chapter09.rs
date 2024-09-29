struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area(self) -> u32 {
        self.width * self.height
    }
}
#[test]
fn test1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
}
#[test]
fn test2() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}

struct TrafficLight2 {
    color: String,
}

impl TrafficLight2 {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
#[test]
fn test3() {
    println!("Success!");
}


#[derive(Debug)]
struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self{
            color: String::from("red")
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}
#[test]
fn test4() {
    let light = TrafficLight3::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

struct Rectangle2 {
    width: u32,
    height: u32,
}

// Using multiple `impl` blocks to rewrite the code below.
impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle2{
    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test]
fn test5() {
    println!("Success!");
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Green => "green",
        }
    }
}
#[test]
fn test6() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}