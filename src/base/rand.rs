use rand::Rng;

pub fn gen_rand(low: usize, high: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high+1)
}