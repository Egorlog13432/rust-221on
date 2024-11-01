use std::collections::HashSet;

fn main() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let mut values = vec![0; 8];
    let mut found = false;

    for m in 1..=8 {
        for u in 1..=8 {
            for x in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    values = vec![m, u, x, a, s, l, o, n];
                                    let unique_values: HashSet<_> = values.iter().cloned().collect();

                                    if unique_values.len() == 8 {
                                        println!("   {}{}{}", m, u, a);
                                        println!("{}    {}", x, a);
                                        println!("  ----");
                                        println!("  {}{}{}{}", s, l, o, n);

                                        found = true;
                                    }
                                    if found { break; }
                                }
                                if found { break; }
                            }
                            if found { break; }
                        }
                        if found { break; }
                    }
                    if found { break; }
                }
                if found { break; }
            }
            if found { break; }
        }
        if found { break; }
    }
}