fn main() {
    // exp変数をRPN形式の文字列に束縛する
    // COMMENT: "<変数>を<値>に束縛する" のか…
    // このRPNは数式 "6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6" と等しい
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    // rpn関数を呼び出して計算する。返された値にans変数を束縛する
    let ans = rpn(exp);

    // デバッグビルド時のみ答えが正しいかチェックする
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下4桁までの桁を文字列に変換している
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // exp と ans の値を表示する。ans は小数点以下4桁までを表示する。
    println!("{} = {:.4}", exp, ans);
}

// RPN形式の文字列expを受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64 {
    // 変数stackをからのスタックに束縛する
    // stack はミュータブルな変数で、値の更新を許す
    let mut stack = Vec::new();

    // expの要素をスペースで分割し、tokenをそれらに順に束縛する
    // 要素がなくなるまで繰り返す
    for token in exp.split_whitespace() {
        // token が f64 の数値ならスタックに積む
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            // token が数値でないなら、演算子なのか調べる
            match token {
                // token が演算子なら apply2 関数で計算する
                // `|x, y| x + y` はクロージャ
                // 引数 x, y を取り、x + y を計算して答えを返す
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // token が演算子で無いなら、エラーを起こして終了する
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // stack から数値を一つ取り出す。失敗したらエラーを起こして終了する
    stack.pop().expect("Stack underflow")
}

// スタックから数値を2つ取り出し、F型のクロージャfunで計算し、結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, func: F)
// F型のトレイト境界。本文参照 p.59
where
    F: Fn(f64, f64) -> f64,
{
    // 変数y と x をスタックの最後
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャfunで計算し、その結果に変数zを束縛する
        let z = func(x, y);
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかったときはエラーを起こして終了する
        panic!("Stack underflow");
    }
}
