use std::io;

fn is_prime(n: i64) -> bool {

    // 偶数チェック
    if n % 2 == 0 {
        return n == 2;
    }

    // 素数判定の上限
    let n_root = (n as f64).sqrt() as i64;

    // 判定処理
    let mut i = 3;
    while i <= n_root {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}

fn main() {
    
    // メインループ
    loop {

        // メニュー表示
        println!("====================");
        println!("メニューを選んでください：");
        println!("1: 素数判定");
        println!("0: 終了");

        // 入力受付
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力エラー");

        // 入力チェック
        let choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("無効な入力です");
                continue;
            }
        };

        // 入力判定
        match choice {
            1 => {
                println!("--------------------");
                println!("素数判定を行う数字を入力してください:");

                let mut ni = String::new();
                io::stdin().read_line(&mut ni).expect("入力エラー");
                let n: i64 = match ni.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("無効な入力です");
                        continue;
                    }
                };

                let is_p = is_prime(n);
                let result = if is_p {
                    "素数です"
                } else {
                    "素数ではありません"
                };
                println!("数値 {} は : {} ", n, result);
            }
            0 => {
                println!("終了します");
                break;
            }
            _ => {
                println!("メニューにない番号です");
            }
        }


    }
}
