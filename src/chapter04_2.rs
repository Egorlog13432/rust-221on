use std::mem::size_of_val;

#[test]
fn test1(){
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}

#[test]
fn test2(){
    let c1: char = '中';
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}

#[test]
fn test3(){
    let _f: bool = false;

    let t: bool = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test4(){
    let f: bool = false;
    let t: bool = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test5(){
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

#[test]
fn test6(){
    let unit: char = 'a';
    assert!(size_of_val(&unit) == 4);

    println!("Success!");
}