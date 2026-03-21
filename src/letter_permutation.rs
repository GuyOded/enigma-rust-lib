use crate::consts::{ALPHABET_SIZE, FIRST_LETTER, LAST_LETTER};
pub(crate) mod utils;

#[derive(Debug, Clone, Copy)]
pub struct LetterPermutation<'a> {
    permutation: &'a [(char, char); ALPHABET_SIZE],
}

#[derive(PartialEq, Debug)]
pub(crate) enum PermutationError {
    NotUppercaseLetter,
}

impl<'a> LetterPermutation<'a> {
    pub(crate) fn new(permutation: &'a [(char, char); ALPHABET_SIZE]) -> Self {
        let mut values_exist = [false; ALPHABET_SIZE];

        permutation.iter().zip(FIRST_LETTER..=LAST_LETTER).for_each(|(&(key, value), alphabet_letter)| {
            if key != alphabet_letter {
                panic!("Expected permutation keys to be ordered alphabetically uppercase letters. Found {key}, expected {alphabet_letter}.");
            }

            if values_exist[LetterPermutation::get_letter_index(value)] {
                panic!("The letter {value} is mapped to twice. Permutations should be bijective maps.");
            }

            values_exist[LetterPermutation::get_letter_index(value)] = true;
        });

        Self { permutation }
    }

    fn get_letter_index(letter: char) -> usize {
        (letter as u8 - FIRST_LETTER as u8) as usize
    }

    pub(crate) fn get(&self, letter: char) -> Result<char, PermutationError> {
        letter
            .is_ascii_uppercase()
            .then(|| self.permutation[LetterPermutation::get_letter_index(letter)].1)
            .ok_or(PermutationError::NotUppercaseLetter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_when_not_alphabetic() {
        let perm = [
            ('A', 'F'),
            ('B', 'V'),
            ('C', 'P'),
            ('D', 'J'),
            ('E', 'I'),
            ('F', 'A'),
            ('G', 'O'),
            ('8', 'Y'),
            ('I', 'E'),
            ('J', 'D'),
            ('K', 'R'),
            ('L', 'Z'),
            ('M', 'X'),
            ('N', 'W'),
            ('O', 'G'),
            ('P', 'C'),
            ('Q', 'T'),
            ('R', 'K'),
            ('S', 'U'),
            ('T', 'Q'),
            ('U', 'S'),
            ('V', 'B'),
            ('W', 'N'),
            ('X', 'M'),
            ('Y', 'H'),
            ('Z', 'L'),
        ];

        LetterPermutation::new(&perm);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_not_uppercase() {
        let perm = [
            ('A', 'F'),
            ('B', 'V'),
            ('c', 'P'),
            ('D', 'J'),
            ('E', 'I'),
            ('F', 'A'),
            ('G', 'O'),
            ('8', 'Y'),
            ('I', 'E'),
            ('J', 'D'),
            ('K', 'R'),
            ('L', 'Z'),
            ('M', 'X'),
            ('N', 'W'),
            ('O', 'G'),
            ('P', 'C'),
            ('Q', 'T'),
            ('R', 'K'),
            ('S', 'U'),
            ('T', 'Q'),
            ('U', 'S'),
            ('V', 'B'),
            ('W', 'N'),
            ('X', 'M'),
            ('Y', 'H'),
            ('Z', 'L'),
        ];

        LetterPermutation::new(&perm);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_not_bijective() {
        let perm = [
            ('A', 'F'),
            ('B', 'V'),
            ('c', 'P'),
            ('D', 'V'),
            ('E', 'I'),
            ('F', 'A'),
            ('G', 'O'),
            ('8', 'Y'),
            ('I', 'E'),
            ('J', 'D'),
            ('K', 'R'),
            ('L', 'Z'),
            ('M', 'X'),
            ('N', 'W'),
            ('O', 'G'),
            ('P', 'C'),
            ('Q', 'T'),
            ('R', 'K'),
            ('S', 'U'),
            ('T', 'Q'),
            ('U', 'S'),
            ('V', 'B'),
            ('W', 'N'),
            ('X', 'M'),
            ('Y', 'H'),
            ('Z', 'L'),
        ];

        LetterPermutation::new(&perm);
    }

    #[test]
    fn should_return_err_when_not_uppercase_is_retrieved() {
        let perm = [
            ('A', 'F'),
            ('B', 'V'),
            ('C', 'P'),
            ('D', 'J'),
            ('E', 'I'),
            ('F', 'A'),
            ('G', 'O'),
            ('H', 'Y'),
            ('I', 'E'),
            ('J', 'D'),
            ('K', 'R'),
            ('L', 'Z'),
            ('M', 'X'),
            ('N', 'W'),
            ('O', 'G'),
            ('P', 'C'),
            ('Q', 'T'),
            ('R', 'K'),
            ('S', 'U'),
            ('T', 'Q'),
            ('U', 'S'),
            ('V', 'B'),
            ('W', 'N'),
            ('X', 'M'),
            ('Y', 'H'),
            ('Z', 'L'),
        ];

        let perm = LetterPermutation::new(&perm);
        assert_eq!(perm.get('t'), Err(PermutationError::NotUppercaseLetter))
    }

    #[test]
    fn should_get_letter() {
        let perm = [
            ('A', 'F'),
            ('B', 'V'),
            ('C', 'P'),
            ('D', 'J'),
            ('E', 'I'),
            ('F', 'A'),
            ('G', 'O'),
            ('H', 'Y'),
            ('I', 'E'),
            ('J', 'D'),
            ('K', 'R'),
            ('L', 'Z'),
            ('M', 'X'),
            ('N', 'W'),
            ('O', 'G'),
            ('P', 'C'),
            ('Q', 'T'),
            ('R', 'K'),
            ('S', 'U'),
            ('T', 'Q'),
            ('U', 'S'),
            ('V', 'B'),
            ('W', 'N'),
            ('X', 'M'),
            ('Y', 'H'),
            ('Z', 'L'),
        ];

        let perm = LetterPermutation::new(&perm);
        assert_eq!(perm.get('J'), Ok('D'))
    }
}
