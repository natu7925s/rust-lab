fn main() {

    // 初期化
    let mut numbers: Vec<i32> = Vec::new();

    // 追加
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // 表示
    for num in &numbers {
        println!("{}", num);
    }

    if let Some(first) = numbers.get(4) {
        println!("{}", first);
    } else {
        println!("None");
    }
}