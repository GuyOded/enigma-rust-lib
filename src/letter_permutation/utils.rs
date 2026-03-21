use crate::consts::ALPHABET_SIZE;

pub(crate) const fn reverse_permutation(
    mut permutation: [(char, char); ALPHABET_SIZE],
) -> [(char, char); ALPHABET_SIZE] {
    let mut i = 0;
    while i < ALPHABET_SIZE {
        permutation[i] = (permutation[i].1, permutation[i].0);
        i += 1;
    }

    let mut i = 0;

    while i < ALPHABET_SIZE {
        let mut j = 0;
        while j + 1 < ALPHABET_SIZE - i {
            if permutation[j].0 > permutation[j + 1].0 {
                let temp = permutation[j];
                permutation[j] = permutation[j + 1];
                permutation[j + 1] = temp;
            }
            j += 1;
        }

        i += 1;
    }

    permutation
}
