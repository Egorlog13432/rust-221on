fn draw_square_with_diagonals(size: usize) {
    for i in 0..size {
        for j in 0..size {
            if i == 0 || i == size - 1 {
                // Верхній і нижній рядок
                print!("#");
            } else if j == 0 || j == size - 1 {
                // Лівий і правий стовпчик
                print!("#");
            } else if i == j || i + j == size - 1 {
                // Діагоналі
                print!("*");
            } else {
                // Порожня частина
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let size = 21;
    draw_square_with_diagonals(size);
}
