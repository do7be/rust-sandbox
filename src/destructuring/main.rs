struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let Point { x, y } = point;
    println!("{}, {}", x, y);

    let Point { x: _, y: y2 } = point;
    println!("{}", y2);

    let Point { x: x2, .. } = point;
    println!("{}", x2);

    let points = vec![Point { x: 1, y: 2 }];
    let fuga = points
        .iter()
        .map(|Point { x, y }| x * x + y * y)
        .collect::<Vec<i32>>();
    println!("{:?}", fuga);
}
