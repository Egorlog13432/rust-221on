
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}
#[test]
fn test1() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}

// IMPLEMENT this function.
fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}


trait Bird1 {
    fn quack(&self);
}

struct Duck1;
impl Duck1 {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan1;
impl Swan1 {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird1 for Duck1 {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird1 for Swan1 {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}
#[test]
fn test2() {
    // FILL in the blank to make the code work.
    let birds1: [&dyn Bird; 2] = [&Duck1, &Swan1];

    for bird in birds1 {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}


// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
#[test]
fn test3() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}




trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(a: T) {
    a.method();
}

// Implement below with trait objects.
fn dynamic_dispatch(a: &dyn Foo) {
    a.method();
}
#[test]
fn test4() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}



// Use at least two approaches to make it work.
// DON'T add/remove any code line.
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn my_function<T: MyTrait>(x: T) -> T {
    x.f()
}
#[test]
fn test5() {
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("Success!");
}
