use std::collections::HashSet;

use phf::Map;

use crate::enigma::consts;

pub fn is_letter_permutation(map: &Map<char, char>) -> bool {
    let set: HashSet<_> = map.values().collect();
    set.len() == consts::ALPHABET_SIZE
}
