fn main() {
    
    let mut i = 0;
    loop {
        println!("loop:{}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }

    for j in 0..5 {
        println!("for1:{}", j);
    }

    let array = ["a", "b", "c", "d", "e"];
    for e in array {
        println!("for2:{}", e);
    }
}