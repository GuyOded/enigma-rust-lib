use std::fmt::Debug;

use crate::consts::ALPHABET_SIZE;
use crate::letter_permutation::LetterPermutation;
use crate::{consts, error::Error};

type PositionType = usize;

#[derive(Debug, Clone)]
pub enum RotorType {
    Rotor1,
    Rotor2,
    Rotor3,
    Rotor4,
    Rotor5,
}

#[derive(Clone)]
pub struct RotorProps {
    permutation: LetterPermutation<'static>,
    inverse: LetterPermutation<'static>,
    step_position: PositionType,
    typ: RotorType,
}

impl Debug for RotorProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RotorProps")
            .field("step_position", &self.step_position)
            .field("typ", &self.typ)
            .finish()
    }
}

impl RotorProps {
    pub(crate) fn new(
        permutation: LetterPermutation<'static>,
        inverse: LetterPermutation<'static>,
        step_position: char,
        typ: RotorType,
    ) -> Self {
        let step_position = step_position as PositionType - consts::FIRST_LETTER as PositionType;

        Self {
            permutation,
            inverse,
            step_position,
            typ,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rotor {
    rotor_props: RotorProps,
    position: PositionType,
    ring_setting: PositionType,
}

impl Rotor {
    pub(super) fn new(props: RotorProps, position: char, ring_setting: char) -> Self {
        if !ring_setting.is_alphabetic() || !position.is_alphabetic() {
            panic!("Position and ring setting must letters")
        }

        let position = position.to_ascii_uppercase();
        let ring_setting = ring_setting.to_ascii_uppercase();

        let position = (position as u8 - consts::FIRST_LETTER as u8) as PositionType;
        let ring_setting = (ring_setting as u8 - consts::FIRST_LETTER as u8) as PositionType;

        Self {
            rotor_props: props,
            position,
            ring_setting,
        }
    }

    pub fn map_letter(&self, letter: char) -> Result<char, Error> {
        self.calculate_mapped_letter_by_ring_setting(
            letter,
            self.rotor_props.permutation,
            self.ring_setting,
        )
    }

    pub fn inverse_map_letter(&self, letter: char) -> Result<char, Error> {
        self.calculate_mapped_letter_by_ring_setting(
            letter,
            self.rotor_props.inverse,
            0 - self.ring_setting,
        )
    }

    pub(crate) fn increment(&mut self) -> bool {
        self.position += 1;
        self.position %= consts::ALPHABET_SIZE;
        self.position == self.rotor_props.step_position
    }

    pub(crate) fn increment_by(&mut self, amount: PositionType) -> usize {
        if amount == 0 {
            return 0;
        }

        let incremented_position = (self.position + amount) % ALPHABET_SIZE;

        let mut next_rotor_increment_amount = 0;
        let num_of_steps_to_next_stepover =
            match (self.rotor_props.step_position as isize - self.position as isize)
                .rem_euclid(ALPHABET_SIZE as isize) as usize
            {
                0 => ALPHABET_SIZE,
                x => x,
            };
        if amount % ALPHABET_SIZE >= num_of_steps_to_next_stepover {
            next_rotor_increment_amount += 1;
        }

        next_rotor_increment_amount += amount / ALPHABET_SIZE;

        self.set_position_from_int(incremented_position);

        next_rotor_increment_amount
    }

    pub fn set_position(&mut self, position: char) {
        if !position.is_alphabetic() {
            panic!("Unable to set current position to non-alphabetic character (got {position})");
        }

        self.position =
            position.to_ascii_uppercase() as PositionType - consts::FIRST_LETTER as PositionType;
    }

    pub fn get_position(&self) -> char {
        (self.position as u8 + consts::FIRST_LETTER as u8) as char
    }

    /// Sets position from an integer. Note that the 0 corresponds to 'A' and 25 corresponds to 'Z'.
    pub fn set_position_from_int(&mut self, position: PositionType) {
        if position >= consts::ALPHABET_SIZE {
            panic!(
                "Position must be a valid letter index (between 0 and {})",
                consts::ALPHABET_SIZE - 1
            );
        }

        self.position = position;
    }

    fn calculate_mapped_letter_by_ring_setting(
        &self,
        letter: char,
        letter_map: LetterPermutation,
        ring_setting_number: PositionType,
    ) -> Result<char, Error> {
        let letter = match letter.is_alphabetic() {
            true => letter.to_ascii_uppercase(),
            false => return Err(Error::NonAlphabetic),
        };

        let position_reduced_by_ring_setting =
            (self.position - ring_setting_number).rem_euclid(consts::ALPHABET_SIZE);

        let input_index = (letter as i8 - consts::FIRST_LETTER as i8
            + position_reduced_by_ring_setting as i8)
            .rem_euclid(consts::ALPHABET_SIZE as i8) as u8;
        let input_letter = (input_index + consts::FIRST_LETTER as u8) as char;

        let mapped_letter = letter_map.get(input_letter).unwrap();

        let mapped_letter_increased_by_ring_setting = ((mapped_letter as i8
            - consts::FIRST_LETTER as i8
            - position_reduced_by_ring_setting as i8)
            .rem_euclid(consts::ALPHABET_SIZE as i8)
            + consts::FIRST_LETTER as i8)
            as u8;
        Ok((mapped_letter_increased_by_ring_setting) as char)
    }
}

#[cfg(test)]
mod tests {
    use crate::rotors;

    #[test]
    #[should_panic]
    fn set_position_from_int_should_panic_when_out_of_bounds() {
        let mut rotor = rotors::create_rotor_1();
        rotor.set_position_from_int(27);
    }

    #[test]
    #[should_panic]
    fn set_position_panics_when_not_alphabetic() {
        let mut rotor = rotors::create_rotor_1();
        rotor.set_position('=');
    }

    #[test]
    fn set_position_from_int_should_work() {
        let mut rotor = rotors::create_rotor_1();
        rotor.set_position_from_int(25);

        assert_eq!(rotor.get_position(), 'Z');
    }
}
