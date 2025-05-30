fn main() {

    /*
        不変変数と可変変数
     */
    
    // 不変変数は再代入NG
    let a = 10;
    // a = 20 NG

    // 可変変数は再代入OK
    let mut b = 10;

    println!("b:{}", b);

    b = 20; // OK
    
    println!("a:{}", a);
    println!("b:{}", b);

    /*
        型宣言
     */

    // 整数型
    let n1 = 10;        // デフォルトでi32
    let n2:i128 = 10;   // i128
    let n3 = 10i128;    // i128  (こっちでも可、でも見にくいから非推奨)
    
    println!("n1:{}", n1);
    println!("n2:{}", n2);
    println!("n3:{}", n3);


    // 実数型
    let f1 = 10.1;      // f64
    let f2:f32 = 10.1;  // f32

    println!("f1:{}", f1);
    println!("f2:{}", f2);

    // 真理値型
    let b1 = true;
    let b2:bool = true;

    println!("b1:{}", b1);
    println!("b2:{}", b2);

    // 文字列型
    let s = "hello rust";   // 固定サイズ 変更不可
    let st = String::from("hello world");   // string型 変更可

    println!("s:{}", s);
    println!("st:{}", st);

}