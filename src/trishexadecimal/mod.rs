pub mod base36 {
    pub struct Base36;

    impl Base36 {
        pub fn new() -> Base36 {
            Base36
        }
        pub fn test(&self){
            println!("Testing base36 ...");
        }
    }
    const BASE: i32 = 36;
    const MAX_DIGITS: i8 = 3;
    #[warn(dead_code)]
    pub enum ValuesLetters {
        A = 10,
        B = 11,
        C = 12,
        D = 13,
        E = 14,
        F = 15,
        G = 16,
        H = 17,
        I = 18,
        J = 19,
        K = 20,
        L = 21,
        M = 22,
        N = 23,
        O = 24,
        P = 25,
        Q = 26,
        R = 27,
        S = 28,
        T = 29,
        U = 30,
        V = 31,
        W = 32,
        X = 33,
        Y = 34,
        Z = 35,
    }
    pub fn num_to_base36(mut num: u64) -> String {
        if num >  base36_to_num("zzz".to_string()) {
            panic!("Out of bounds!");
        }
        let mut values: [u64; MAX_DIGITS as usize] = [0, 0, 0];
        let mut result: String = "".to_string();

        values[2] = num % BASE as u64;
        num = num / BASE as u64;
        values[1] = num % BASE as u64;
        num = num / BASE as u64;
        values[0] = num;

        for i in 0..MAX_DIGITS
        {
            result += base10_to_base36(values[i as usize]).to_string().as_str();
        }

        result
    }

    pub fn base36_to_num(num: String) -> u64 {
        let mut result: u64 = 0;
        let mut position: i8 = num.len() as i8;
        let mut pow_value: u64 = 0;
        let mut value: u64 = 0;

        if num.chars().count() > 3 {
            panic!("Too many digits");
        }
        for i in num.to_uppercase().chars() {
            value = base36_to_base10(i);
            pow_value = BASE.pow(position as u32 - 1) as u64;
            result += pow_value * value;

            //println!("{}", value); println!("-----------------"); println!("||{}", pow_value);
            //println!("-----------------"); println!(" ||||  {}", pow_value * value); println!("-----------------");

            position -= 1;
        }

        result
    }

    fn base36_to_base10(num: char) -> u64 {
        match num {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => ValuesLetters::A as u64,
            'B' => ValuesLetters::B as u64,
            'C' => ValuesLetters::C as u64,
            'D' => ValuesLetters::D as u64,
            'E' => ValuesLetters::E as u64,
            'F' => ValuesLetters::F as u64,
            'G' => ValuesLetters::G as u64,
            'H' => ValuesLetters::H as u64,
            'I' => ValuesLetters::I as u64,
            'J' => ValuesLetters::J as u64,
            'K' => ValuesLetters::K as u64,
            'L' => ValuesLetters::L as u64,
            'M' => ValuesLetters::M as u64,
            'N' => ValuesLetters::N as u64,
            'O' => ValuesLetters::O as u64,
            'P' => ValuesLetters::P as u64,
            'Q' => ValuesLetters::Q as u64,
            'R' => ValuesLetters::R as u64,
            'S' => ValuesLetters::S as u64,
            'T' => ValuesLetters::T as u64,
            'U' => ValuesLetters::U as u64,
            'V' => ValuesLetters::V as u64,
            'W' => ValuesLetters::W as u64,
            'X' => ValuesLetters::X as u64,
            'Y' => ValuesLetters::Y as u64,
            'Z' => ValuesLetters::Z as u64,
            _ => 0,
        }
    }

    fn base10_to_base36(num: u64) -> char {
        match num {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            16 => 'G',
            17 => 'H',
            18 => 'I',
            19 => 'J',
            20 => 'K',
            21 => 'L',
            22 => 'M',
            23 => 'N',
            24 => 'O',
            25 => 'P',
            26 => 'Q',
            27 => 'R',
            28 => 'S',
            29 => 'T',
            30 => 'U',
            31 => 'V',
            32 => 'W',
            33 => 'X',
            34 => 'Y',
            35 => 'Z',
            _ => ' ',
        }
    }
}
