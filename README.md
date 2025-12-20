# Enigma Rust Library

This library provides a simple implementation for the enigma machine in rust.

## Examples

```rust
use enigma::Enigma;
use enigma::rotor::rotors;
use enigma::reflectors;

// Create rotors & reflector using provided constructors
let left = rotors::create_rotor_3();
let middle = rotors::create_rotor_2();
let right = rotors::create_rotor_1();
let reflector = reflectors::create_reflector_b();

// Build the machine
let mut enigma = Enigma::new(left, middle, right, reflector);

// (Optional) set starting rotor positions
enigma.set_left_rotor_position('A');
enigma.set_middle_rotor_position('A');
enigma.set_right_rotor_position('A');

// Encrypt a message (returns Result<String, Error>)
let ciphertext = enigma.encrypt(String::from("HELLOWORLD")).unwrap();
println!("Ciphertext: {}", ciphertext);

// To decrypt, reset rotor positions to the same starting state and re-run encrypt
// (or recreate the machine with the same initial settings)
let mut enigma_for_decrypt = Enigma::new(left, middle, right, reflector);
enigma_for_decrypt.set_left_rotor_position('A');
enigma_for_decrypt.set_middle_rotor_position('A');
enigma_for_decrypt.set_right_rotor_position('A');
let plaintext = enigma_for_decrypt.encrypt(ciphertext).unwrap();
assert_eq!(plaintext, "HELLOWORLD");
```

## Notes

- Rotors advance as you encrypt; to decrypt you must restore the same rotor positions (and plugboard/transpositions).
- Use `encrypt_char` for single-character encryption (returns `Result<char, Error>`).
- Non-alphabetic input will return an error.
