use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let (time, period) = s.split_at(8);
    let hour: i32 = time[0..2].parse().unwrap();

    let converted_hour = match period {
        "AM" => {
            if hour == 12 { 0 } else { hour }
        }
        "PM" => {
            if hour == 12 { 12 } else { hour + 12 }
        }
        _ => hour,
    };

    format!("{:02}{}", converted_hour, &time[2..])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}