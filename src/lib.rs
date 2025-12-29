mod consts;
pub mod error;
pub mod reflectors;
pub mod rotor;

use reflectors::Reflector;
use rotor::Rotor;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::Error;

#[derive(Debug)]
pub struct Enigma {
    left_rotor: Rc<RefCell<Rotor>>,
    middle_rotor: Rc<RefCell<Rotor>>,
    right_rotor: Rc<RefCell<Rotor>>,
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
        let left_rotor = Rc::new(RefCell::new(left_rotor));
        let middle_rotor = Rc::new(RefCell::new(middle_rotor));
        let right_rotor = Rc::new(RefCell::new(right_rotor));

        middle_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&left_rotor));
        right_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&middle_rotor));

        Self {
            left_rotor,
            middle_rotor,
            right_rotor,
            reflector,
            transpositions: HashMap::new(),
        }
    }

    pub fn encrypt_char(&self, letter: char) -> Result<char, Error> {
        let letter = letter.to_ascii_uppercase();
        let enciphered = self.transpositions.get(&letter).unwrap_or(&letter);

        let enciphered = self
            .right_rotor
            .borrow_mut()
            .increment_and_map(*enciphered)?;
        let enciphered = self.middle_rotor.borrow().map_letter(enciphered)?;
        let enciphered = self.left_rotor.borrow().map_letter(enciphered)?;

        let enciphered = self.reflector.map.get(&enciphered).unwrap_or(&enciphered);

        let enciphered = self.left_rotor.borrow().inverse_map_letter(*enciphered)?;
        let enciphered = self.middle_rotor.borrow().inverse_map_letter(enciphered)?;
        let enciphered = self.right_rotor.borrow().inverse_map_letter(enciphered)?;

        Ok(*self.transpositions.get(&enciphered).unwrap_or(&enciphered))
    }

    pub fn encrypt(&self, text: String) -> Result<String, Error> {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }

    pub fn set_transposition(&mut self, first: char, second: char) {
        let (first, second) = match (first.is_alphabetic(), second.is_alphabetic()) {
            (true, true) => (first.to_ascii_uppercase(), second.to_ascii_uppercase()),
            (false, _) => panic!("Transposition characters must be letters but got '{first}'"),
            (_, false) => panic!("Transposition characters must be letters but got '{second}'"),
        };

        if let Some(value) = self.transpositions.remove(&first) {
            self.transpositions.remove(&value);
        }
        if let Some(value) = self.transpositions.remove(&second) {
            self.transpositions.remove(&value);
        }

        self.transpositions.insert(first, second);
        self.transpositions.insert(second, first);
    }

    pub fn set_left_rotor(&mut self, rotor: Rotor) {
        self.left_rotor = Rc::new(RefCell::new(rotor));
        self.middle_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&self.left_rotor));
    }

    pub fn set_middle_rotor(&mut self, rotor: Rotor) {
        self.middle_rotor = Rc::new(RefCell::new(rotor));
        self.middle_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&self.left_rotor));
        self.right_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&self.middle_rotor));
    }

    pub fn set_right_rotor(&mut self, rotor: Rotor) {
        self.right_rotor = Rc::new(RefCell::new(rotor));
        self.right_rotor
            .borrow_mut()
            .set_next_rotor(Rc::clone(&self.middle_rotor));
    }

    pub fn set_reflector(&mut self, reflector: Reflector) {
        self.reflector = reflector;
    }

    pub fn get_left_rotor_position(&self) -> char {
        self.left_rotor.borrow().get_position()
    }

    pub fn get_middle_rotor_position(&self) -> char {
        self.middle_rotor.borrow().get_position()
    }

    pub fn get_right_rotor_position(&self) -> char {
        self.right_rotor.borrow().get_position()
    }

    pub fn set_left_rotor_position(&mut self, position: char) {
        self.left_rotor.borrow_mut().set_position(position);
    }

    pub fn set_middle_rotor_position(&mut self, position: char) {
        self.middle_rotor.borrow_mut().set_position(position);
    }

    pub fn set_right_rotor_position(&mut self, position: char) {
        self.right_rotor.borrow_mut().set_position(position);
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::rotor::rotors;
    use crate::{Enigma, reflectors};

    #[test]
    fn enigma_encrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt(String::from("HelloWorld")).unwrap();

        assert_eq!(encrypted.as_str(), "MFNCZBBFZM");
    }

    #[test]
    fn enigma_decrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let enigma = Enigma::new(left, middle, right, reflector);

        let deciphered = enigma.encrypt(String::from("MFNCZBBFZM")).unwrap();

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

        let cipher = enigma.encrypt(cleaned_text.clone()).unwrap();
        enigma.set_left_rotor(rotors::create_rotor_3());
        enigma.set_middle_rotor(rotors::create_rotor_2());
        enigma.set_right_rotor(rotors::create_rotor_1());

        let plain = enigma.encrypt(cipher).unwrap();

        assert_eq!(plain, cleaned_text.to_ascii_uppercase())
    }

    #[test]
    fn enigma_position_set_properly() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt(String::from("HelloWorld")).unwrap();

        enigma.set_left_rotor_position('A');
        enigma.set_middle_rotor_position('A');
        enigma.set_right_rotor_position('A');

        let plain = enigma.encrypt(encrypted).unwrap();

        assert_eq!(plain, "HELLOWORLD");
    }

    #[test]
    fn rotor_state_should_change() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let enigma = Enigma::new(left, middle, right, reflector);

        let text = String::from(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let _ = enigma.encrypt(text);

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

        let enigma = Enigma::new(left, middle, right, reflector);

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

        enigma.set_left_rotor_position('G');
        enigma.set_middle_rotor_position('I');
        enigma.set_right_rotor_position('I');

        let cipher = enigma.encrypt(String::from("internal")).unwrap();

        enigma.set_left_rotor_position('G');
        enigma.set_middle_rotor_position('I');
        enigma.set_right_rotor_position('I');

        let decipher = enigma.encrypt(cipher).unwrap();

        assert_eq!(decipher, "INTERNAL");
    }
}
