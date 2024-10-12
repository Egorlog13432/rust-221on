fn main() {
    const SIZE: u8 = 11;
    let half = SIZE / 2;

    fn m(a: u8) -> u8 {
        SIZE - 1 - a
    }

    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if x + y < half ||
                    m(x) + y < half ||
                    x + m(y) < half ||
                    m(x) + m(y) < half {
                ' '
            } else {
                '*'
            };
            print!("{c}");
        }
        println!();
    }
}
