use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNG を初期化する。再現性を持たせるために毎回同じシードを使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // n 個の要素が格納できるようにベクタを初期化する
    let mut v = Vec::with_capacity(n);

    // 0 から n - 1 までの合計 n 回、繰り返し乱数を生成し、ベクタに追加する
    // (0 から n - 1 の数列は使わないので、_ で受けることですぐに破棄している)
    for _ in 0..n {
        // RNG の sample メソッドは引数として与えられた分布に従う乱数を1つ生成する
        // Standard 分布は生成する値が数値型(ここでは u32 型)のときは一様分布になる。
        // つまり、その型が取りうるすべての値が同じ確率で出現する。
        v.push(rng.sample(&Standard));
    }

    // ベクタを返す
    v
}
