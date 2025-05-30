mod math;

fn main() {
    let x = 5;
    let y = 3;

    let sum = math::add(x, y);
    let sq = math::square(x);

    println!("sum:{}", sum);
    println!("sq:{}", sq);
}