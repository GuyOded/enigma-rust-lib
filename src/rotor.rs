use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use crate::{consts, error::Error};
use phf::Map;
pub mod rotors;

#[derive(Clone, Copy)]
pub(super) struct RotorProps {
    permutation: &'static Map<char, char>,
    inverse: &'static Map<char, char>,
    step_position: u8,
    name: &'static str,
}

impl Debug for RotorProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RotorProps")
            .field("step_position", &self.step_position)
            .field("name", &self.name)
            .finish()
    }
}

impl RotorProps {
    fn new(
        permutation: &'static Map<char, char>,
        inverse: &'static Map<char, char>,
        step_position: char,
        name: &'static str,
    ) -> Self {
        let step_position = step_position as u8 - consts::FIRST_LETTER as u8;

        Self {
            permutation,
            inverse,
            step_position,
            name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rotor {
    rotor_props: RotorProps,
    position: i8,
    ring_setting: i8,
    next_rotor: Option<Rc<RefCell<Rotor>>>,
}

impl Rotor {
    pub(super) fn new(
        props: RotorProps,
        position: char,
        ring_setting: char,
        next_rotor: Option<Rc<RefCell<Rotor>>>,
    ) -> Self {
        if !ring_setting.is_alphabetic() || !position.is_alphabetic() {
            panic!("Position and ring setting must letters")
        }

        let position = position.to_ascii_uppercase();
        let ring_setting = ring_setting.to_ascii_uppercase();

        let position = position as i8 - consts::FIRST_LETTER as i8;
        let ring_setting = ring_setting as i8 - consts::FIRST_LETTER as i8;

        Self {
            rotor_props: props,
            position,
            ring_setting,
            next_rotor,
        }
    }

    pub fn map_letter(&self, letter: char) -> Result<char, Error> {
        self.calculate_mapped_letter_by_ring_setting(
            letter,
            &self.rotor_props.permutation,
            self.ring_setting,
        )
    }

    pub fn inverse_map_letter(&self, letter: char) -> Result<char, Error> {
        self.calculate_mapped_letter_by_ring_setting(
            letter,
            &self.rotor_props.inverse,
            -self.ring_setting,
        )
    }

    pub fn increment(&mut self) {
        self.position += 1;
        self.position %= consts::ALPHABET_SIZE as i8;
        if let Some(next_rotor) = &mut self.next_rotor
            && self.position == self.rotor_props.step_position as i8
        {
            next_rotor.borrow_mut().increment();
        }
    }

    pub fn increment_and_map(&mut self, letter: char) -> Result<char, Error> {
        self.increment();
        self.map_letter(letter)
    }

    pub fn set_position(&mut self, position: char) {
        if !position.is_alphabetic() {
            panic!("Unable to set current position to non-alphabetic character (got {position})");
        }

        self.position = position.to_ascii_uppercase() as i8 - consts::FIRST_LETTER as i8;
    }

    pub fn get_position(&self) -> char {
        (self.position as u8 + consts::FIRST_LETTER as u8) as char
    }

    pub fn set_position_from_int(&mut self, position: u8) {
        if position >= consts::ALPHABET_SIZE as u8 {
            panic!(
                "Position must be a valid letter index (between 0 and {})",
                consts::ALPHABET_SIZE - 1
            );
        }

        self.position = position as i8;
    }

    pub(super) fn set_next_rotor(&mut self, rotor: Rc<RefCell<Rotor>>) {
        self.next_rotor = Some(rotor);
    }

    fn calculate_mapped_letter_by_ring_setting(
        &self,
        letter: char,
        letter_map: &Map<char, char>,
        ring_setting_number: i8,
    ) -> Result<char, Error> {
        let letter = match letter.is_alphabetic() {
            true => letter.to_ascii_uppercase(),
            false => return Err(Error::NonAlphabetic),
        };

        let position_reduced_by_ring_setting: u8 =
            (self.position - ring_setting_number).rem_euclid(consts::ALPHABET_SIZE as i8) as u8;

        let input_index = (letter as i8 - consts::FIRST_LETTER as i8
            + position_reduced_by_ring_setting as i8)
            .rem_euclid(consts::ALPHABET_SIZE as i8) as u8;
        let input_letter = (input_index + consts::FIRST_LETTER as u8) as char;

        let mapped_letter = letter_map.get(&input_letter).unwrap();

        let mapped_letter_increased_by_ring_setting = ((*mapped_letter as i8
            - consts::FIRST_LETTER as i8
            - position_reduced_by_ring_setting as i8)
            .rem_euclid(consts::ALPHABET_SIZE as i8)
            + consts::FIRST_LETTER as i8)
            as u8;
        Ok((mapped_letter_increased_by_ring_setting) as char)
    }
}
