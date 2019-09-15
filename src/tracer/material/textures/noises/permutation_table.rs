use rand::prelude::*;

const TABLE_SIZE: usize = 256;

#[derive(Copy, Clone)]
pub struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl PermutationTable {
    pub fn new(seed: u32) -> PermutationTable {
        let mut rng = rand::thread_rng();
        let mut seq: Vec<u8> = (0..TABLE_SIZE).map(|x| x as u8).collect();
        seq.shuffle(&mut rng);

        let seq_it = seq.iter();

        let mut perm_table = PermutationTable {
            values: [0; TABLE_SIZE],
        };

        for (x, y) in perm_table.values.iter_mut().zip(seq_it) {
            *x = *y
        }
        perm_table
    }
}
