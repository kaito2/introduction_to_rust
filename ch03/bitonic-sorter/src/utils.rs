use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNG を初期化する。再現性を持たせるために毎回同じシードを使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}
