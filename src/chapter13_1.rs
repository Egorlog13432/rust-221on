
// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!();
    }

    println!("Exercise Failed if printing out this line!");
}
#[test]
fn test1() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}


// MAKE the code work by fixing all panics
#[test]
fn test2() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v: Vec<i32> = vec![1, 2, 3];
    let _ele: i32 = v[2];
    // unwrap may panic when get return a None
    let _ele: &i32 = v.get(1).unwrap();

    // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
    let _v = production_rate_per_hour(2);

    // because of the same reason as above, we have to wrap it in a function to make the panic occur
    divide(15, 1);

    println!("Success!")
}

fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u16) -> f64 {
    let cph: u16 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u16) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}