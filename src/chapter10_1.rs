struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
#[test]
fn test1() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('A'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('Z'));

    println!("Success!");
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
#[test]
fn test2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}


// Implement struct Point to make it work.
struct Point<T> {
    x: T,
    y: T,
}
#[test]
fn test3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}


// Modify this struct to make the code work
struct Point2<T, U> {
    x: T,
    y: U,
}
#[test]
fn test4() {
    // DON'T modify this code.
    let p = Point2{x: 5, y : "hello".to_string()};

    println!("Success!");
}


// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

#[test]
fn test5() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

struct Point6<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point6<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V, W>(self, other: Point6<V, W>) -> Point6<T, W> {
        Point6{
            x: self.x,
            y: other.y,
        }
    }
}
#[test]
fn test6() {
    let p1 = Point6 { x: 5, y: 10 };
    let p2 = Point6 { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

// Fix the errors to make the code work.
struct Point7<T> {
    x: T,
    y: T,
}

impl Point7<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
#[test]
fn test7() {
    let p: Point7<f32> = Point7{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}