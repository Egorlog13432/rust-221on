struct Array<T, const N: usize> {
    data : [T; N]
}
#[test]
fn test1() {
    let _arrays: [Array<i32, 3>; 3] = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [4, 5, 6]
        }
    ];
    let _numder: [Array<i32, 4>; 3] = [
        Array{
            data: [1, 2, 3, 4],
        },
        Array {
            data: [1, 2, 3, 4],
        },
        Array {
            data: [4, 5, 6, 7]
        }
    ];
    println!("Success!");
}

// Fill in the blanks to make it work.
fn print_array<T>(arr: &[T]) {
    println!("{:?}", arr);
}
#[test]
fn test2() {
    let arr = [1, 2, 3];
    print_array(&arr);

    let arr = ["hello", "world"];
    print_array(&arr);
}

// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// Fix the errors in main.
#[test]
fn test3() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 1]); // Size of &str ?
    check_size([(); 1].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; 1]); // Size of char ?

    println!("Success!");
}
pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}