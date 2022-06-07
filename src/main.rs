use crypto_shuffle::SafeShuffler;
use rand_core::OsRng;

fn sequence(n: u32) -> Vec<u32> {
    (0..n).into_iter().collect::<Vec<u32>>()
}

fn main() {
    let orig = sequence(10);
    let mut rng = OsRng;
    let mut shuffler = SafeShuffler::new(&mut rng);
    let scrambled = shuffler.shuffle(orig.to_vec());

    orig.iter()
        .zip(scrambled.iter())
        .for_each(|(old, new)| println!("{old} --> {new}"));
}
