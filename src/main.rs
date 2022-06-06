use rand_core::{OsRng, RngCore};

fn sequence(n: u32) -> Vec<u32> {
    (0..n).into_iter().collect::<Vec<u32>>()
}

fn get_u32(upper: u32) -> u32 {
    // determine the maximum bit length that `upper` can have
    let bit_length = u32::MAX.count_ones() - upper.leading_zeros();
    let min_byte_length = (bit_length + 7) / 8;

    let mut rng = OsRng;

    let mut buffer: [u8; 4] = [0; 4];
    let mut num: u32;

    loop {
        rng.fill_bytes(&mut buffer);
        let mask = (1 << (min_byte_length * 8)) - 1;

        num = u32::from_le_bytes(buffer);

        // sampled mod 2^L
        num = num & mask;

        if num < upper {
            break;
        }
    }
    num
}

fn fisher_yates(mut seq: Vec<u32>) -> Vec<u32> {
    for i in 0..seq.len() {
        let index = get_u32(seq.len() as u32);
        seq.swap(i, index as usize);
    }
    seq
}

fn main() {
    let orig = sequence(10);
    let scrambled = fisher_yates(orig.to_vec());

    orig.iter()
        .zip(scrambled.iter())
        .for_each(|(old, new)| println!("{old} --> {new}"));
}
