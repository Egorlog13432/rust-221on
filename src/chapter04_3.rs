#[test]
fn testEs(){
    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z: u32= {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

#[test]
fn test1(){
    let v: () = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
}

#[test]
fn test2(){
    let v:i32 = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

#[test]
fn test3(){
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}