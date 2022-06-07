use rand_core::{CryptoRng, RngCore};

pub struct SafeShuffler<R: CryptoRng + RngCore> {
    rng: R,
}

impl<R: CryptoRng + RngCore> SafeShuffler<R> {
    pub fn new(rng: R) -> Self {
        SafeShuffler { rng }
    }

    pub fn shuffle<T>(&mut self, mut seq: Vec<T>) -> Vec<T> {
        for i in 0..seq.len() {
            let index = self.safe_range(seq.len());
            seq.swap(i, index as usize);
        }
        seq
    }

    fn safe_range(&mut self, upper_bound: usize) -> usize {
        let _word_size = usize::BITS / 8;

        let bit_length = usize::MAX.count_ones() - upper_bound.leading_zeros();
        let min_byte_length = (bit_length + 7) / 8;

        let mut buffer: [u8; 8] = [0; 8];
        let mut num: usize;

        loop {
            self.rng.fill_bytes(&mut buffer);

            let byte_mask = (1 << (min_byte_length * 8)) - 1;
            let msb_offset = 8 - (bit_length & 7);
            let bit_mask = byte_mask >> msb_offset;

            let unmasked = usize::from_be_bytes(buffer);
            num = unmasked & bit_mask;

            if num < upper_bound {
                break;
            }
        }
        num
    }
}
