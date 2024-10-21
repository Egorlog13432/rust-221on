use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = i32::MAX;
    let mut min_pair = None;

    for window in data.windows(2) {
        let sum = window[0] + window[1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = Some((window[0], window[1]));
        }
    }

    min_pair
}

fn print_vector(data: &[i32]) {
    let formatted: Vec<String> = data.iter().map(|x| x.to_string()).collect();
    println!("Вектор значень: [{}]", formatted.join(", "));
}

fn main() {
    // Генеруємо випадковий вектор
    let random_vector = gen_random_vector(20);

    // Виводимо вектор
    print_vector(&random_vector);

    // Знаходимо мінімальну пару
    if let Some((a, b)) = min_adjacent_sum(&random_vector) {
        println!("Мінімальна пара сусідніх значень: ({}, {})", a, b);
    } else {
        println!("Вектор занадто малий для знаходження сусідніх пар.");
    }
}