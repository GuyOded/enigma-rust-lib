use phf::{Map, phf_map};

#[derive(Debug)]
pub struct Reflector {
    pub map: &'static Map<char, char>,
    pub name: &'static str,
}

static REFLECTOR_A_MAP: Map<char, char> = phf_map!(
    'A' => 'E',
    'B' => 'J',
    'C' => 'M',
    'D' => 'Z',
    'E' => 'A',
    'F' => 'L',
    'G' => 'Y',
    'H' => 'X',
    'I' => 'V',
    'J' => 'B',
    'K' => 'W',
    'L' => 'F',
    'M' => 'C',
    'N' => 'R',
    'O' => 'Q',
    'P' => 'U',
    'Q' => 'O',
    'R' => 'N',
    'S' => 'T',
    'T' => 'S',
    'U' => 'P',
    'V' => 'I',
    'W' => 'K',
    'X' => 'H',
    'Y' => 'G',
    'Z' => 'D',
);

static REFLECTOR_B_MAP: Map<char, char> = phf_map!(
    'A' => 'Y',
    'B' => 'R',
    'C' => 'U',
    'D' => 'H',
    'E' => 'Q',
    'F' => 'S',
    'G' => 'L',
    'H' => 'D',
    'I' => 'P',
    'J' => 'X',
    'K' => 'N',
    'L' => 'G',
    'M' => 'O',
    'N' => 'K',
    'O' => 'M',
    'P' => 'I',
    'Q' => 'E',
    'R' => 'B',
    'S' => 'F',
    'T' => 'Z',
    'U' => 'C',
    'V' => 'W',
    'W' => 'V',
    'X' => 'J',
    'Y' => 'A',
    'Z' => 'T',
);

static REFLECTOR_C_MAP: Map<char, char> = phf_map!(
    'A' => 'F',
    'B' => 'V',
    'C' => 'P',
    'D' => 'J',
    'E' => 'I',
    'F' => 'A',
    'G' => 'O',
    'H' => 'Y',
    'I' => 'E',
    'J' => 'D',
    'K' => 'R',
    'L' => 'Z',
    'M' => 'X',
    'N' => 'W',
    'O' => 'G',
    'P' => 'C',
    'Q' => 'T',
    'R' => 'K',
    'S' => 'U',
    'T' => 'Q',
    'U' => 'S',
    'V' => 'B',
    'W' => 'N',
    'X' => 'M',
    'Y' => 'H',
    'Z' => 'L',
);

static REFLECTOR_BETA_MAP: Map<char, char> = phf_map!(
    'A' => 'L',
    'B' => 'E',
    'C' => 'Y',
    'D' => 'J',
    'E' => 'V',
    'F' => 'C',
    'G' => 'N',
    'H' => 'I',
    'I' => 'X',
    'J' => 'W',
    'K' => 'P',
    'L' => 'B',
    'M' => 'Q',
    'N' => 'M',
    'O' => 'D',
    'P' => 'R',
    'Q' => 'T',
    'R' => 'A',
    'S' => 'K',
    'T' => 'Z',
    'U' => 'G',
    'V' => 'F',
    'W' => 'U',
    'X' => 'H',
    'Y' => 'O',
    'Z' => 'S',
);

static REFLECTOR_GAMMA_MAP: Map<char, char> = phf_map!(
    'A' => 'F',
    'B' => 'S',
    'C' => 'O',
    'D' => 'K',
    'E' => 'A',
    'F' => 'N',
    'G' => 'U',
    'H' => 'E',
    'I' => 'R',
    'J' => 'H',
    'K' => 'M',
    'L' => 'B',
    'M' => 'T',
    'N' => 'I',
    'O' => 'Y',
    'P' => 'C',
    'Q' => 'W',
    'R' => 'L',
    'S' => 'Q',
    'T' => 'P',
    'U' => 'Z',
    'V' => 'X',
    'W' => 'V',
    'X' => 'G',
    'Y' => 'J',
    'Z' => 'D',
);

pub fn create_reflector_a() -> Reflector {
    Reflector {
        map: &REFLECTOR_A_MAP,
        name: "Reflector A",
    }
}

pub fn create_reflector_b() -> Reflector {
    Reflector {
        map: &REFLECTOR_B_MAP,
        name: "Reflector B",
    }
}

pub fn create_reflector_c() -> Reflector {
    Reflector {
        map: &REFLECTOR_C_MAP,
        name: "Reflector C",
    }
}

pub fn create_reflector_beta() -> Reflector {
    Reflector {
        map: &REFLECTOR_BETA_MAP,
        name: "Reflector Beta",
    }
}

pub fn create_reflector_gamma() -> Reflector {
    Reflector {
        map: &REFLECTOR_GAMMA_MAP,
        name: "Reflector Gamma",
    }
}
