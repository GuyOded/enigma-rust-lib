use phf::{Map, phf_map};

use crate::rotor::{Rotor, RotorProps};

macro_rules! bidir_map {
    ($forward:ident, $inverse:ident, {$($k:expr => $v:expr),*$(,)?}) => {
        static $forward: Map<char, char> = phf_map!{
            $($k => $v),*
        };

        static $inverse: Map<char, char> = phf_map!{
            $($v => $k),*
        };
    };
}

bidir_map!(ROTOR_1_PERMUTATION, ROTOR_1_INVERSE,
{
    'A' => 'E',
    'B' => 'K',
    'C' => 'M',
    'D' => 'F',
    'E' => 'L',
    'F' => 'G',
    'G' => 'D',
    'H' => 'Q',
    'I' => 'V',
    'J' => 'Z',
    'K' => 'N',
    'L' => 'T',
    'M' => 'O',
    'N' => 'W',
    'O' => 'Y',
    'P' => 'H',
    'Q' => 'X',
    'R' => 'U',
    'S' => 'S',
    'T' => 'P',
    'U' => 'A',
    'V' => 'I',
    'W' => 'B',
    'X' => 'R',
    'Y' => 'C',
    'Z' => 'J',
});

bidir_map!(ROTOR_2_PERMUTATION, ROTOR_2_INVERSE,
{
    'A' => 'A',
    'B' => 'J',
    'C' => 'D',
    'D' => 'K',
    'E' => 'S',
    'F' => 'I',
    'G' => 'R',
    'H' => 'U',
    'I' => 'X',
    'J' => 'B',
    'K' => 'L',
    'L' => 'H',
    'M' => 'W',
    'N' => 'T',
    'O' => 'M',
    'P' => 'C',
    'Q' => 'Q',
    'R' => 'G',
    'S' => 'Z',
    'T' => 'N',
    'U' => 'P',
    'V' => 'Y',
    'W' => 'F',
    'X' => 'V',
    'Y' => 'O',
    'Z' => 'E',
});

bidir_map!(ROTOR_3_PERMUTATION, ROTOR_3_INVERSE,
{
    'A' => 'B',
    'B' => 'D',
    'C' => 'F',
    'D' => 'H',
    'E' => 'J',
    'F' => 'L',
    'G' => 'C',
    'H' => 'P',
    'I' => 'R',
    'J' => 'T',
    'K' => 'X',
    'L' => 'V',
    'M' => 'Z',
    'N' => 'N',
    'O' => 'Y',
    'P' => 'E',
    'Q' => 'I',
    'R' => 'W',
    'S' => 'G',
    'T' => 'A',
    'U' => 'K',
    'V' => 'M',
    'W' => 'U',
    'X' => 'S',
    'Y' => 'Q',
    'Z' => 'O',
});

bidir_map!(ROTOR_4_PERMUTATION, ROTOR_4_INVERSE,
{
    'A' => 'E',
    'B' => 'S',
    'C' => 'O',
    'D' => 'V',
    'E' => 'P',
    'F' => 'Z',
    'G' => 'J',
    'H' => 'A',
    'I' => 'Y',
    'J' => 'Q',
    'K' => 'U',
    'L' => 'I',
    'M' => 'R',
    'N' => 'H',
    'O' => 'X',
    'P' => 'L',
    'Q' => 'N',
    'R' => 'F',
    'S' => 'T',
    'T' => 'G',
    'U' => 'K',
    'V' => 'D',
    'W' => 'C',
    'X' => 'M',
    'Y' => 'W',
    'Z' => 'B',
});

bidir_map!(ROTOR_5_PERMUTATION, ROTOR_5_INVERSE, {
    'A' => 'V',
    'B' => 'Z',
    'C' => 'B',
    'D' => 'R',
    'E' => 'G',
    'F' => 'I',
    'G' => 'T',
    'H' => 'Y',
    'I' => 'U',
    'J' => 'P',
    'K' => 'S',
    'L' => 'D',
    'M' => 'N',
    'N' => 'H',
    'O' => 'L',
    'P' => 'X',
    'Q' => 'A',
    'R' => 'W',
    'S' => 'M',
    'T' => 'J',
    'U' => 'Q',
    'V' => 'O',
    'W' => 'F',
    'X' => 'E',
    'Y' => 'C',
    'Z' => 'K',
});

pub fn create_rotor_1() -> Rotor {
    let props = RotorProps::new(&ROTOR_1_PERMUTATION, &ROTOR_1_INVERSE, 'R', "Rotor 1");
    Rotor::new(props, 'A', 'A', None)
}

pub fn create_rotor_2() -> Rotor {
    let props = RotorProps::new(&ROTOR_2_PERMUTATION, &ROTOR_2_INVERSE, 'F', "Rotor 2");
    Rotor::new(props, 'A', 'A', None)
}

pub fn create_rotor_3() -> Rotor {
    let props = RotorProps::new(&ROTOR_3_PERMUTATION, &ROTOR_3_INVERSE, 'W', "Rotor 3");
    Rotor::new(props, 'A', 'A', None)
}

pub fn create_rotor_4() -> Rotor {
    let props = RotorProps::new(&ROTOR_4_PERMUTATION, &ROTOR_4_INVERSE, 'K', "Rotor 4");
    Rotor::new(props, 'A', 'A', None)
}

pub fn create_rotor_5() -> Rotor {
    let props = RotorProps::new(&ROTOR_5_PERMUTATION, &ROTOR_5_INVERSE, 'A', "Rotor 5");
    Rotor::new(props, 'A', 'A', None)
}
