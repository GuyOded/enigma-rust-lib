pub mod enigma;

#[cfg(test)]
mod tests {
    use crate::enigma::rotor::rotors;
    use crate::enigma::{Enigma, reflectors};

    #[test]
    fn enigma_encrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt(String::from("HelloWorld"));

        assert_eq!(encrypted.as_str(), "MFNCZBBFZM");
    }

    #[test]
    fn enigma_decrypts() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let enigma = Enigma::new(left, middle, right, reflector);

        let deciphered = enigma.encrypt(String::from("MFNCZBBFZM"));

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

        let cipher = enigma.encrypt(cleaned_text.clone());
        enigma.set_left_rotor(rotors::create_rotor_3());
        enigma.set_middle_rotor(rotors::create_rotor_2());
        enigma.set_right_rotor(rotors::create_rotor_1());

        let plain = enigma.encrypt(cipher);

        assert_eq!(plain, cleaned_text.to_ascii_uppercase())
    }

    #[test]
    fn enigma_position_set_properly() {
        let left = rotors::create_rotor_3();
        let middle = rotors::create_rotor_2();
        let right = rotors::create_rotor_1();
        let reflector = reflectors::create_reflector_b();

        let mut enigma = Enigma::new(left, middle, right, reflector);

        let encrypted = enigma.encrypt(String::from("HelloWorld"));

        enigma.set_left_rotor_position('A');
        enigma.set_middle_rotor_position('A');
        enigma.set_right_rotor_position('A');

        let plain = enigma.encrypt(encrypted);

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
        enigma.encrypt(text);

        println!("{:#?}", enigma);

        assert_ne!(enigma.get_left_rotor_position(), 'A');
        assert_ne!(enigma.get_middle_rotor_position(), 'A');
        assert_ne!(enigma.get_right_rotor_position(), 'A');
    }
}
