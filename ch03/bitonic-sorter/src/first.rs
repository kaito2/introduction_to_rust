// pubはこのsort関数が他のモジュールからアクセスできることを示す
// 引数 x の型 &mut [u32] について
//   &は値をポインタ経由で借用することを示す(借用については7章)
//   mut は値が変更可能であることを示す
//   u32 は32ビット符号なし整数
//   [u32] はu32のスライス
pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        // Python 実装とはことなり、x を直接操作している。
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}

// このモジュールは cargo test を実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    // 親モジュール (first) のsort関数を使用する
    use super::sort;

    #[test]
    fn sert_u32_ascending() {
        // テストデータとして u32 型のベクタを作成し、xに束縛する
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // x のスライスを作成し、sort関数を呼び出す
        // &mut x は &mut x[..] と書いても良い
        sort(&mut x, true);

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4])
    }
}
