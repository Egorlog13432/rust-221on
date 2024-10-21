#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Rectangle {
    a: Point,
    b: Point,
}

// Функція для обчислення площі прямокутника
fn rectangle_area(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.a.y - rect.b.y).abs();
    width * height
}

// Функція для знаходження перетворення між двома прямокутниками
fn overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = std::cmp::max(0, std::cmp::min(r1.b.x, r2.b.x) - std::cmp::max(r1.a.x, r2.a.x));
    let y_overlap = std::cmp::max(0, std::cmp::min(r1.a.y, r2.a.y) - std::cmp::max(r1.b.y, r2.b.y));
    x_overlap * y_overlap
}

// Функція для обчислення фактичної зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlaps = 0;

    // Обчислюємо загальну площу та площу перекриття
    for i in 0..xs.len() {
        total_area += rectangle_area(&xs[i]);
        for j in i + 1..xs.len() {
            overlaps += overlap_area(&xs[i], &xs[j]);
        }
    }

    // Повертаємо фактичну зайняту площу
    total_area - overlaps
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Тест для функції area_occupied
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
#[test]
fn main() {
    area_occupied_test(); // Запуск тесту
    println!("Тест пройдено успішно!");
}
