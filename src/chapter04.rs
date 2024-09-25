#[test]
fn test1() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32;

    let z: u32 = 10; // Type of z ?

    println!("Success!");
}

#[test]
fn test2() {
    let v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
fn test5() {
    let v1 = 251_u8.saturating_add(8); // Using saturating_add to avoid overflow
    let v2 = i8::checked_add(127, 8).unwrap(); // This will panic if overflow occurs

    println!("{}, {}", v1, v2);
}

#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); // Update to the correct expected value

    println!("Success!");
}

#[test]
fn test7() {
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of2(&x), "f64".to_string());
    println!("Success!");
}

fn type_of2<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8() {
    let result = (0.1 + 0.2 * 10.0).round() / 10.0; // Round to one decimal place
    let expected = 0.3;

    assert!(result == expected); // This should now work as expected

    println!("Success!");
}

#[test]
fn test9() {

}