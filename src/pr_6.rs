fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[test]
fn main() {
    assert_eq!(gcd(15, 6), 3);
    assert_eq!(gcd(60, 24), 12);

    println!("Success!");
}
