use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn capitalize_and_square(value: (String, Vec<i32>)) -> (String, Vec<i32>) {
    let (x, y) = value;
    let x: String = x.to_ascii_uppercase();
    let y: Vec<_> = y.into_iter().map(|it| it * it).collect();
    (x, y)
}
