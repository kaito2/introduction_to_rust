use super::SortOrder;
use std::cmp::Ordering;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sort を呼ぶ代わりに, sort_by を呼ぶようにする
    // is_power_of_two は sort_by が呼ぶのでここからは削除
    match *order {
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        // Python 実装とはことなり、x を直接操作している。
        let mid_point = x.len() / 2;
        // xをバイトニックにソートする
        // 第2引数がtrueのときはcomparatorで示される順でソート
        do_sort(&mut x[..mid_point], true, comparator);
        // 第2引数がfalseのときはcomparatorとは逆順でソート
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // 比較に先立ち、forward (bool) を Ordering に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparator クロージャで2要素を比較し、返された Ordering のバリアントが
        // swap_condition として等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use super::sort_by;
    use crate::SortOrder::*;

    #[test]
    fn sort_u32_ascending() {
        // テストデータとして u32 型のベクタを作成し、xに束縛する
        // x に型注釈 Vec<u32> をつける
        // MEMO: sort が [u32] のみ受け付けていたため、 x の型が推論されていたが、
        //       sort 関数をジェネリクス化すると型推論が効かないため型注釈をつける
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // x のスライスを作成し、sort関数を呼び出す
        // &mut x は &mut x[..] と書いても良い
        assert!(sort(&mut x, &Ascending).is_ok());

        // xの要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert!(sort(&mut x, &Descending).is_ok());
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4])
    }

    #[test]
    fn sort_str_ascending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert!(sort(&mut x, &Ascending).is_ok());
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        // 文字列のベクタを作り、ソートする
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert!(sort(&mut x, &Descending).is_ok());
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC",
            ]
        );
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11]; // 2のべき乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }

    // 構造体Studentを定義する
    // 構造体は関連する値を1つにまとめたデータ構造。複数のデータフィールドを持つ
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String, // first_name (名前) フィールド。String型
        last_name: String,  // last_name (苗字) フィールド。String型
        age: u8,            // age (年齢) フィールド。u8型 (8ビット符号なし整数)
    }

    impl Student {
        // 関連関数newを定義する
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体Studentを初期化して返す。Selfはimpl対象の型 (Student) の別称
            Self {
                // to_string メソッドで &str 型の引数から String 型の値を作る。詳しくは5章
                first_name: first_name.to_string(), // first_name フィールドに値を設定
                last_name: last_name.to_string(),   // last_name フィールドに値を設定
                age,                                // age フィールドに age 変数の値を設定
                                                    // フィールドと変数が同名の場合は、このように省略形で書ける
            }
        }
    }
    // 手動で実装すると以下のようになる
    /*
    impl PartialEq for Student {
        fn eq(&self, other: &Self) -> bool {
            // self と otherですべてのフィールド同士を比較して、どのフィールドも等しいなら、
            // self と other は等しい
            self.first_name == other.first_name
                && self.last_name == other.last_name
                && self.age == other.age
        }
    }
    */

    #[test]
    // 年齢で昇順にソートする
    fn sort_students_by_age_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクトルを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // sort_by 関数でソートする。第2引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体を取り、ageフィールドの値をcmpメソッドで比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果を検証
        assert_eq!(x, expected);
    }

    #[test]
    // 名前でソート
    fn sort_students_by_name_ascending() {
        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクトルを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            // sort_by 関数でソートする。第2引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体を取り、ageフィールドの値をcmpメソッドで比較することで大小を決定する
            sort_by(&mut x, &|a, b| a
                .last_name
                .cmp(&b.last_name)
                // もし last_name が等しくないならそれを返す。
                // last_name が等しいなら first_name を比較する
                .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );

        // 結果を検証
        assert_eq!(x, expected);
    }
}
