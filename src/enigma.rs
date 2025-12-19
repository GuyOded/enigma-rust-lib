mod consts;
mod map_utils;
pub mod reflectors;
pub mod rotor;

use reflectors::Reflector;
use rotor::Rotor;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

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

    pub fn encrypt_char(&self, letter: char) -> char {
        let enciphered = self.transpositions.get(&letter).unwrap_or(&letter);

        let enciphered = self.right_rotor.borrow_mut().increment_and_map(*enciphered);
        let enciphered = self.middle_rotor.borrow().map_letter(enciphered);
        let enciphered = self.left_rotor.borrow().map_letter(enciphered);

        let enciphered = self.reflector.map.get(&enciphered).unwrap_or(&enciphered);

        let enciphered = self.left_rotor.borrow().inverse_map_letter(*enciphered);
        let enciphered = self.middle_rotor.borrow().inverse_map_letter(enciphered);
        let enciphered = self.right_rotor.borrow().inverse_map_letter(enciphered);

        *self.transpositions.get(&enciphered).unwrap_or(&enciphered)
    }

    pub fn encrypt(&self, text: String) -> String {
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
}
