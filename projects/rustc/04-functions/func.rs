fn square_1(n:i32) -> i32 {
    // returnなしでも返せる、その場合はセミコロン(;)はつけない
    // rustらしいのはこっち
    n.pow(2)
}

fn square_2(n:i32) -> i32 {
    // returnを使う場合はセミコロンは必要
    return n.pow(2);
}

fn main() {
    let n = 10;

    println!("square1:{}", square_1(n));
    println!("square2:{}", square_2(n));
}