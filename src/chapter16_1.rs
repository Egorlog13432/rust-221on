#[test]
fn main1() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
    println!("{}", s)
}

#[test]
fn main2() {
    /* Fill in the blanks to make it print:
    Hello world, I am
    Sunface!
    */
    print!("hello world, ");
    println!("I am");
    println!("Sunface!");
}