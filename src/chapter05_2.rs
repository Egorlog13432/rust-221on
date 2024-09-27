#[test]
fn test1(){
    let x: i32 = 5;
    // Fill the blank
    let p: &i32 = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

#[test]
fn test2(){
    let x: i32 = 5;
    let y: &i32 = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

#[test]
fn test3(){
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

#[test]
fn test4(){
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

#[test]
fn test5(){
    let mut s: String = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

#[test]
fn test6(){
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
fn test7(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

#[test]
fn test8(){
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

#[test]
fn test9(){
    let mut s = String::from("hello, ");

    borrow_object2(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object2(s: &String) {}

#[test]
fn test10(){
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    println!("{}",r2);
}

#[test]
fn test11(){
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2 )
}