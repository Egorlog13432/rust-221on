fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let count = shipments.len();

    if total % count as u32 != 0 {
        return 0;
    }

    let average = total / count as u32;
    let mut moves = 0;

    for &shipment in shipments {
        if shipment > average {
            moves += shipment - average;
        }
    }

    moves as usize
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![0; n];
    let total_weight: u32 = (1..=n as u32).sum();
    let average = total_weight / n as u32;

    for i in 0..n {
        shipments[i] = average;
    }

    shipments
}

fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let answer1 = count_permutation(&shipments1);
    println!("Answer for [8, 2, 2, 4, 4]: {}", answer1); // Виведе: Answer: 4

    let shipments2 = vec![9, 3, 7, 2, 9];
    let answer2 = count_permutation(&shipments2);
    println!("Answer for [9, 3, 7, 2, 9]: {}", answer2); // Виведе: Answer: 7

    let shipments3 = gen_shipments(5);
    println!("Generated shipments: {:?}", shipments3);
}