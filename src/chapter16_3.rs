/* Fill in the blanks */
#[test]
fn test1() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112");
    println!("Success!");
}

#[test]
fn test2() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = "2"), "21");
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

    /* Fix the error */
    // Named argument must be placed after other arguments
    println!("{0} {abc}", 2, abc = "def"); // Fix applied



    println!("Success!");
}

#[test]
fn test3() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello {:1$}!", "x", 5), "Hello x    !");
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");

    println!("Success!");
}

#[test]
fn test4() {
    // Left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // Right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // Center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // Left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!");
}

#[test]
fn test5() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

    println!("Success!");
}


/* Fill in the blanks */
#[test]
fn test6() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!");
}


#[test]
fn test7() {
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {:.3}!", "abcdefg"), "Hello abc!");

    println!("Success!");
}


#[test]
fn test8() {
    assert_eq!(format!("{:#b}", 27), "0b11011");
    assert_eq!(format!("{:#o}", 27), "0o33");
    assert_eq!(format!("{:#x}", 27), "0x1b");
    assert_eq!(format!("{:#X}", 27), "0x1B");

    println!("{:x}!", 27); // Hex with no prefix => 1b

    println!("{:#010b}", 27); // Pad binary with 0, width = 10,  => 0b00011011

    println!("Success!");
}


fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}

#[test]
fn test9() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    /* Make it print:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        println!("{name}: {:#?}", score);
    }
}