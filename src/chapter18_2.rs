/* Refactoring the following code using iterators */
#[test]
fn test1() {
    let arr = [0; 10];
    for i in arr.into_iter() {
        println!("{}",arr[i]);
    }
}

/* Fill in the blank */
#[test]
fn test2() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

#[test]
/* Fill the blanks and fix the errors.
Using two ways if possible */
fn test3() {
    let mut v1 = vec![1, 2].into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}



#[test]
/* Make it work */
fn test4() {
    let arr = vec![0; 10];
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("{:?}",arr);
}


#[test]
/* Fill in the blank */
fn test5() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut(){
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}


#[test]
/* Fill in the blank */
fn test6() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next(){
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}



struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    /* Implement next method */
    fn next(&mut self) -> Option<Self::Item> {
        let forward = self.curr + self.next;

        self.curr = self.next;
        self.next = forward;

        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}
#[test]
fn test7() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}

#[test]

/* Fill in the blank and fix the errors */
fn test8() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}",v1);
    println!("{:?}",total);
}

#[test]
/* Make it work */
use std::collections::HashMap;
fn test9() {
    let names = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<&str, i32> = names.into_iter().collect();

    println!("{:?}",folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.into_iter().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}


#[test]
/* Fill in the blanks */
fn test10() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}


#[test]

fn test11() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];

    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}", folks);
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}
#[test]
fn test12() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

