mod consts;
pub mod error;
mod letter_permutation;
pub mod reflectors;
pub mod rotor;
pub mod rotors;
pub mod rotors_controller;

use reflectors::Reflector;
use rotor::Rotor;
use std::collections::HashMap;

use error::Error;

use crate::rotors_controller::RotorsController;

#[derive(Debug)]
pub struct Enigma {
    rotor_controller: RotorsController,
    reflector: Reflector,
    transpositions: HashMap<char, char>,
}

impl Enigma {
    pub fn new(
        left_rotor: Rotor,
        middle_rotor: Rotor,
        right_rotor: Rotor,
        reflector: Reflector,
    ) -> Self {
        Self {
            rotor_controller: RotorsController::new(left_rotor, middle_rotor, right_rotor),
            reflector,
            transpositions: HashMap::new(),
        }
    }

    pub fn encrypt_char(&mut self, letter: char) -> Result<char, Error> {
        let enciphered = self.rotor_controller.increment_and_map(letter)?;

        let enciphered = self
            .reflector
            .map
            .get(enciphered)
            .map_err(|_| Error::NonAlphabetic)?;

        self.rotor_controller.inverse_map_letter(enciphered)
    }

    pub fn encrypt_string(&mut self, text: String) -> Result<String, Error> {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }

    pub fn encrypt_str(&mut self, text: &str) -> Result<String, Error> {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }

    pub fn encrypt_str_iter(&mut self, text: &str) -> impl Iterator<Item = Result<char, Error>> {
        text.chars().map(|c| self.encrypt_char(c))
    }

    pub fn increment_by(&mut self, amount: usize) {
        self.rotor_controller.increment_by(amount);
    }

    ///
    /// Returns the encryption result of the next character encryption without changing the rotors' position.
    ///
    /// The encryption works the same - increment rotor first, then calculate the character through all the permutations.
    /// At the end of the encryption process the rotors are returned to their original location.
    ///
    pub fn peak_cipher(&mut self, char: char) -> Result<char, Error> {
        let rotor_positions = (
            self.rotor_controller.get_left_position(),
            self.rotor_controller.get_middle_position(),
            self.rotor_controller.get_right_position(),
        );

        let encryption_result = self.encrypt_char(char);

        self.set_left_rotor_position_from_char(rotor_positions.0);
        self.set_middle_rotor_position_from_char(rotor_positions.1);
        self.set_right_rotor_position_from_char(rotor_positions.2);

        encryption_result
    }

    ///
    /// Similar to peak_cipher but doesn't increment the rotor before mapping the given character through all permutations.
    ///
    pub fn peak_without_increment(&self, letter: char) -> Result<char, Error> {
        let enciphered = self.rotor_controller.map_letter(letter)?;

        let enciphered = self
            .reflector
            .map
            .get(enciphered)
            .map_err(|_| Error::NonAlphabetic)?;

        self.rotor_controller.inverse_map_letter(enciphered)
    }

    pub fn set_transposition(&mut self, first: char, second: char) {
        let (first, second) = match (first.is_alphabetic(), second.is_alphabetic()) {
            (true, true) => (first.to_ascii_uppercase(), second.to_ascii_uppercase()),
            (false, _) => panic!("Transposition characters must be letters but got '{first}'"),
            (_, false) => panic!("Transposition characters must be letters but got '{second}'"),
        };

        if first == second {
            return;
        }

        if let Some(value) = self.transpositions.remove(&first) {
            self.transpositions.remove(&value);
        }
        if let Some(value) = self.transpositions.remove(&second) {
            self.transpositions.remove(&value);
        }

        self.transpositions.insert(first, second);
        self.transpositions.insert(second, first);
    }

    pub fn get_transpositions(&self) -> &HashMap<char, char> {
        &self.transpositions
    }

    pub fn clear_transposition(&mut self, letter: char) -> Option<char> {
        if let Some(removed) = self.transpositions.remove(&letter) {
            self.transpositions.remove(&removed);
            return Some(removed);
        }

        None
    }

    pub fn clear_transpositions(&mut self) {
        self.transpositions.clear();
    }

    pub fn set_left_rotor(&mut self, rotor: Rotor) {
        self.rotor_controller.set_left_rotor(rotor);
    }

    pub fn set_middle_rotor(&mut self, rotor: Rotor) {
        self.rotor_controller.set_middle_rotor(rotor);
    }

    pub fn set_right_rotor(&mut self, rotor: Rotor) {
        self.rotor_controller.set_right_rotor(rotor);
    }

    pub fn set_reflector(&mut self, reflector: Reflector) {
        self.reflector = reflector;
    }

    pub fn get_left_rotor_position(&self) -> char {
        self.rotor_controller.get_left_position()
    }

    pub fn get_middle_rotor_position(&self) -> char {
        self.rotor_controller.get_middle_position()
    }

    pub fn get_right_rotor_position(&self) -> char {
        self.rotor_controller.get_right_position()
    }

    pub fn set_left_rotor_position_from_char(&mut self, position: char) {
        self.rotor_controller
            .set_left_rotor_position_from_char(position);
    }

    pub fn set_middle_rotor_position_from_char(&mut self, position: char) {
        self.rotor_controller
            .set_middle_rotor_position_from_char(position);
    }

    pub fn set_right_rotor_position_from_char(&mut self, position: char) {
        self.rotor_controller
            .set_right_rotor_position_from_char(position);
    }

    pub fn set_left_rotor_position_from_int(&mut self, position: usize) {
        self.rotor_controller
            .set_left_rotor_position_from_int(position);
    }

    pub fn set_middle_rotor_position_from_int(&mut self, position: usize) {
        self.rotor_controller
            .set_middle_rotor_position_from_int(position);
    }

    pub fn set_right_rotor_position_from_int(&mut self, position: usize) {
        self.rotor_controller
            .set_right_rotor_position_from_int(position);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::error::Error;
    use crate::rotors;
    use crate::{Enigma, reflectors};

    #[test]
    fn enigma_encrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt_string(String::from("HelloWorld")).unwrap();

        assert_eq!(encrypted.as_str(), "MFNCZBBFZM");
    }

    #[test]
    fn enigma_decrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let deciphered = enigma.encrypt_string(String::from("MFNCZBBFZM")).unwrap();

        assert_eq!(deciphered, "HELLOWORLD");
    }

    #[test]
    fn enigma_encrypts_and_decrypts_long_text() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let text = String::from("The afternoon arrived quietly, as afternoons often do, carrying with it a sense of polite uncertainty and a mild awareness of its own existence. Nothing in particular happened, and yet several things occurred in a manner that suggested they might matter later, even though they never did. The room held its shape confidently, filled with air that had clearly been there for some time, and the furniture agreed silently to continue being furniture.
A man sat near a window that did not request attention, observing the way light behaved when it decided not to behave at all. The light was neither bright nor dim, but something in between that implied commitment without requiring follow-through. He considered standing up, briefly, and then chose to remain seated, which felt like a decision worthy of internal acknowledgment.
Outside, the street continued being a street with admirable consistency. Cars passed by with destinations they believed in deeply, and pedestrians walked with expressions suggesting they were thinking about something unrelated to walking. A dog paused to reconsider its priorities, then resumed them exactly as before.");
        let cleaned_text = text
            .replace(" ", "")
            .replace(",", "")
            .replace(".", "")
            .replace("-", "")
            .replace("\n", "");

        let cipher = enigma.encrypt_string(cleaned_text.clone()).unwrap();
        enigma.set_left_rotor(rotors::create_rotor_3());
        enigma.set_middle_rotor(rotors::create_rotor_2());
        enigma.set_right_rotor(rotors::create_rotor_1());

        let plain = enigma.encrypt_string(cipher).unwrap();

        assert_eq!(plain, cleaned_text.to_ascii_uppercase())
    }

    #[test]
    fn enigma_position_set_properly() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt_string(String::from("HelloWorld")).unwrap();

        enigma.set_left_rotor_position_from_char('A');
        enigma.set_middle_rotor_position_from_char('A');
        enigma.set_right_rotor_position_from_char('A');

        let plain = enigma.encrypt_string(encrypted).unwrap();

        assert_eq!(plain, "HELLOWORLD");
    }

    #[test]
    fn rotor_state_should_change() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let text = String::from(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let _ = enigma.encrypt_string(text);

        assert_ne!(enigma.get_left_rotor_position(), 'A');
        assert_ne!(enigma.get_middle_rotor_position(), 'A');
        assert_ne!(enigma.get_right_rotor_position(), 'A');
    }

    #[test]
    fn non_alphabetic_encryption_should_return_error() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let r = enigma.encrypt_char(' ');
        assert_eq!(r.err().unwrap(), Error::NonAlphabetic)
    }

    #[test]
    fn encrypt_and_decrypt_with_transpositions_should_result_in_plain() {
        let left = rotors::create_rotor_2();
        let mid = rotors::create_rotor_1();
        let right = rotors::create_rotor_4();

        let reflector = reflectors::create_reflector_a();

        let mut enigma = Enigma::new(left, mid, right, reflector);

        enigma.set_transposition('H', 'G');
        enigma.set_transposition('I', 'D');
        enigma.set_transposition('Z', 'U');
        enigma.set_transposition('B', 'X');
        enigma.set_transposition('F', 'W');
        enigma.set_transposition('A', 'M');
        enigma.set_transposition('Q', 'V');
        enigma.set_transposition('K', 'N');
        enigma.set_transposition('P', 'E');

        enigma.set_left_rotor_position_from_char('G');
        enigma.set_middle_rotor_position_from_char('I');
        enigma.set_right_rotor_position_from_char('I');

        let cipher = enigma.encrypt_string(String::from("internal")).unwrap();

        enigma.set_left_rotor_position_from_char('G');
        enigma.set_middle_rotor_position_from_char('I');
        enigma.set_right_rotor_position_from_char('I');

        let decipher = enigma.encrypt_string(cipher).unwrap();

        assert_eq!(decipher, "INTERNAL");
    }

    #[test]
    fn enigma_should_encrypt_with_iterator() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted: String = enigma
            .encrypt_str_iter(&"HelloWorld")
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(encrypted, "MFNCZBBFZM");
    }

    #[test]
    fn enigma_should_encrypt_with_str() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted: String = enigma.encrypt_str("HelloWorld").unwrap();

        assert_eq!(encrypted, "MFNCZBBFZM");
    }

    #[test]
    fn peaking_encryption_should_encrypt_without_incrementing() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        (0..28).for_each(|_| assert_eq!(enigma.peak_cipher('H').unwrap(), 'M'));
    }

    #[test]
    fn setting_and_clearing_transposition_with_key_should_remove_it() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        enigma.set_transposition('A', 'B');
        enigma.clear_transposition('A');

        assert!(enigma.transpositions.is_empty())
    }

    #[test]
    fn setting_and_clearing_transposition_with_value_should_remove_it() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        enigma.set_transposition('A', 'B');
        enigma.clear_transposition('B');

        assert!(enigma.transpositions.is_empty())
    }

    #[test]
    fn setting_transpositions_with_same_key_and_value_should_do_nothing() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        enigma.set_transposition('A', 'A');

        assert!(enigma.transpositions.is_empty())
    }

    #[test]
    fn resetting_transposition_with_different_value_should_remove_existing() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        enigma.set_transposition('A', 'B');
        enigma.set_transposition('A', 'C');

        assert_eq!(
            enigma.transpositions,
            HashMap::from([('A', 'C'), ('C', 'A')])
        )
    }

    #[test]
    fn resetting_transposition_with_different_key_should_remove_existing() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);
        enigma.set_transposition('A', 'B');
        enigma.set_transposition('B', 'C');

        assert_eq!(
            enigma.transpositions,
            HashMap::from([('B', 'C'), ('C', 'B')])
        )
    }

    #[test]
    fn increment_by_should_work_for_every_position() {
        let mut increment_by_result = String::new();
        const TEST_LETTER: char = 'A';
        const INCREMENT_CYCLES: usize = 100;
        const INCREMENT_AMOUNT: usize = 7;

        let mut enigma = Enigma::new(
            rotors::create_rotor_1(),
            rotors::create_rotor_2(),
            rotors::create_rotor_3(),
            reflectors::create_reflector_a(),
        );

        for _ in 0..INCREMENT_CYCLES {
            enigma.increment_by(INCREMENT_AMOUNT);
            increment_by_result.push(enigma.peak_without_increment(TEST_LETTER).unwrap());
        }

        let left_rotor_position_after_inc_by = enigma.get_left_rotor_position();
        let middle_rotor_position_after_inc_by = enigma.get_middle_rotor_position();
        let right_rotor_position_after_inc_by = enigma.get_right_rotor_position();

        enigma.set_left_rotor_position_from_char('A');
        enigma.set_middle_rotor_position_from_char('A');
        enigma.set_right_rotor_position_from_char('A');
        let mut encrypt_result = String::new();

        for _ in 0..INCREMENT_CYCLES {
            for _ in 0..INCREMENT_AMOUNT - 1 {
                let _ = enigma.encrypt_char(TEST_LETTER);
            }
            encrypt_result.push(enigma.encrypt_char(TEST_LETTER).unwrap());
        }

        assert_eq!(
            (
                left_rotor_position_after_inc_by,
                middle_rotor_position_after_inc_by,
                right_rotor_position_after_inc_by
            ),
            (
                enigma.get_left_rotor_position(),
                enigma.get_middle_rotor_position(),
                enigma.get_right_rotor_position()
            )
        );

        assert_eq!(increment_by_result, encrypt_result);
    }
}
