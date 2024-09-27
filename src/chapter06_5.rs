struct Person {
    name: String,
    age: u8,
    hobby: String
}
#[test]
fn test1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding")
    };

    println!("Success!");
}

struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
#[test]
fn test2() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {   }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn test3() {
    let v: Point = Point(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Point) {
    let Point (x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

struct Person1 {
    name: String,
    age: u8,
}
#[test]
fn test4() {
    let age = 18;
    let mut p = Person1 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}

struct Person2 {
    name: String,
    age: u8,
}
#[test]
fn test5() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person2 {
    Person2 {
        age,
        name,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[test]
fn test6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[test]
fn test7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
#[test]
fn test8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}",_name, f.data);
}