use crate::error::Error;
use crate::rotor::Rotor;

#[derive(Debug)]
pub struct RotorsController {
    left: Rotor,
    middle: Rotor,
    right: Rotor,
}

impl RotorsController {
    pub fn new(left: Rotor, middle: Rotor, right: Rotor) -> Self {
        Self {
            left,
            middle,
            right,
        }
    }

    pub fn increment_and_map(&mut self, letter: char) -> Result<char, Error> {
        self.increment();

        self.map_letter(letter)
    }

    pub fn increment(&mut self) {
        let increment_middle = self.right.increment();
        if increment_middle {
            let increment_left = self.middle.increment();
            if increment_left {
                self.left.increment();
            }
        }
    }

    pub fn increment_by(&mut self, amount: usize) {
        let steps_to_increment_middle = self.right.increment_by(amount);
        if steps_to_increment_middle > 0 {
            let steps_to_increment_left = self.middle.increment_by(steps_to_increment_middle);

            if steps_to_increment_left > 0 {
                self.left.increment_by(steps_to_increment_left);
            }
        }
    }

    pub fn set_left_rotor_position_from_char(&mut self, letter: char) {
        self.left.set_position(letter);
    }

    pub fn set_middle_rotor_position_from_char(&mut self, letter: char) {
        self.middle.set_position(letter);
    }

    pub fn set_right_rotor_position_from_char(&mut self, letter: char) {
        self.right.set_position(letter);
    }

    pub fn set_left_rotor_position_from_int(&mut self, letter: usize) {
        self.left.set_position_from_int(letter);
    }

    pub fn set_middle_rotor_position_from_int(&mut self, letter: usize) {
        self.middle.set_position_from_int(letter);
    }

    pub fn set_right_rotor_position_from_int(&mut self, letter: usize) {
        self.right.set_position_from_int(letter);
    }

    pub fn set_right_rotor(&mut self, rotor: Rotor) {
        self.right = rotor;
    }

    pub fn set_middle_rotor(&mut self, rotor: Rotor) {
        self.middle = rotor;
    }

    pub fn set_left_rotor(&mut self, rotor: Rotor) {
        self.left = rotor;
    }

    pub fn get_left_position(&self) -> char {
        self.left.get_position()
    }

    pub fn get_middle_position(&self) -> char {
        self.middle.get_position()
    }

    pub fn get_right_position(&self) -> char {
        self.right.get_position()
    }

    pub fn map_letter(&self, letter: char) -> Result<char, Error> {
        let letter = self.map_char_from_right(letter)?;
        let letter = self.map_char_from_middle(letter)?;
        self.map_char_from_left(letter)
    }

    pub fn map_char_from_right(&self, letter: char) -> Result<char, Error> {
        self.right.map_letter(letter)
    }

    pub fn map_char_from_middle(&self, letter: char) -> Result<char, Error> {
        self.middle.map_letter(letter)
    }

    pub fn map_char_from_left(&self, letter: char) -> Result<char, Error> {
        self.left.map_letter(letter)
    }

    pub fn inverse_map_letter(&self, letter: char) -> Result<char, Error> {
        let letter = self.inverse_map_char_from_left(letter)?;
        let letter = self.inverse_map_char_from_middle(letter)?;
        self.inverse_map_char_from_right(letter)
    }

    pub fn inverse_map_char_from_right(&self, letter: char) -> Result<char, Error> {
        self.right.inverse_map_letter(letter)
    }

    pub fn inverse_map_char_from_middle(&self, letter: char) -> Result<char, Error> {
        self.middle.inverse_map_letter(letter)
    }

    pub fn inverse_map_char_from_left(&self, letter: char) -> Result<char, Error> {
        self.left.inverse_map_letter(letter)
    }
}

#[cfg(test)]
mod tests {
    use crate::{rotors, rotors_controller::RotorsController};

    #[test]
    fn increment_by_should_work() {
        let right = rotors::create_rotor_1();
        let middle = rotors::create_rotor_2();
        let left = rotors::create_rotor_3();
        let mut controller = RotorsController::new(left, middle, right);

        const INCREMENT_AMOUNT: usize = 1052;
        controller.increment_by(INCREMENT_AMOUNT);
        let first_rotor_position_after_inc_by = controller.get_right_position();
        let second_rotor_position_after_inc_by = controller.get_middle_position();

        controller.set_right_rotor_position_from_char('A');
        controller.set_middle_rotor_position_from_char('A');

        for _ in 0..INCREMENT_AMOUNT {
            controller.increment();
        }

        assert_eq!(
            controller.get_right_position(),
            first_rotor_position_after_inc_by
        );

        assert_eq!(
            controller.get_middle_position(),
            second_rotor_position_after_inc_by
        );
    }

    #[test]
    fn increment_by_should_increment_next_once_when_first_rotor_step_is_hit() {
        let right = rotors::create_rotor_1();
        let middle = rotors::create_rotor_2();
        let left = rotors::create_rotor_3();
        let mut controller = RotorsController::new(left, middle, right);

        controller.set_right_rotor_position_from_char('R');
        controller.set_middle_rotor_position_from_char('S');

        const INCREMENT_AMOUNT: usize = 23;
        controller.increment_by(INCREMENT_AMOUNT);
        let second_rotor_position_after_inc_by = controller.get_middle_position();

        controller.set_right_rotor_position_from_char('R');
        controller.set_middle_rotor_position_from_char('S');

        for _ in 0..INCREMENT_AMOUNT {
            controller.increment();
        }

        assert_eq!('S', second_rotor_position_after_inc_by);
    }

    #[test]
    fn increment_by_should_not_overflow_when_incremented_position_is_less_then_current() {
        let right = rotors::create_rotor_1();
        let middle = rotors::create_rotor_2();
        let left = rotors::create_rotor_3();
        let mut controller = RotorsController::new(left, middle, right);

        controller.set_right_rotor_position_from_char('Z');
        controller.set_middle_rotor_position_from_char('S');

        const INCREMENT_AMOUNT: usize = 3;
        controller.increment_by(INCREMENT_AMOUNT);
        let first_rotor_position_after_inc_by = controller.get_right_position();

        controller.set_right_rotor_position_from_char('Z');
        controller.set_middle_rotor_position_from_char('S');

        for _ in 0..INCREMENT_AMOUNT {
            controller.increment();
        }

        assert_eq!(
            controller.get_right_position(),
            first_rotor_position_after_inc_by
        );
    }
}
