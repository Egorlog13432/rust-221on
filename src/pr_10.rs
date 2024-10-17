fn rotate(s: &str, n: isize) -> String {
    let len = s.len();
    
    if len == 0 { return s.to_string(); }

    let n = n.rem_euclid(len as isize) as usize;
    
    let (tail, head) = s.split_at(len - n);
    format!("{}{}", tail, head)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(rotate(s, *n), exp.to_string())
        );
}
 

