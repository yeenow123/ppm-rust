enum MagicNumber {
    P3,
    P6,
}

pub struct PPM {
    magic_number: MagicNumber,
    pub width: u32,
    pub height: u32,
    max_color_value: u32,
    pub data: Vec<RGB>,
}

#[derive(Debug)]
pub struct RGB(pub u8, pub u8, pub u8);

impl PPM {
    pub fn new(buffer: Vec<u8>) -> Self {
        let first_char = buffer[0] as char;
        let second_char = buffer[1] as char;

        let magic_number = match (first_char, second_char) {
            ('P', '6') => MagicNumber::P6,
            ('P', '3') => MagicNumber::P3,
            _ => panic!("Invalid PPM format"),
        };

        // Go past linefeed
        let mut idx = 2;
        while PPM::is_whitespace(buffer[idx]) {
            idx += 1;
        }

        let mut width = 0u32;
        while !PPM::is_whitespace(buffer[idx]) {
            width *= 10;
            width += (buffer[idx] as char).to_digit(10).unwrap();
            idx += 1;
        }

        while PPM::is_whitespace(buffer[idx]) {
            idx += 1;
        }

        let mut height = 0u32;
        while !PPM::is_whitespace(buffer[idx]) {
            height *= 10;
            height += (buffer[idx] as char).to_digit(10).unwrap();
            idx += 1;
        }

        while PPM::is_whitespace(buffer[idx]) {
            idx += 1;
        }

        let mut max_color_value = 0u32;
        while !PPM::is_whitespace(buffer[idx]) {
            max_color_value *= 10;
            max_color_value += (buffer[idx] as char).to_digit(10).unwrap();
            idx += 1;
        }

        while PPM::is_whitespace(buffer[idx]) {
            idx += 1;
        }

        // Get data
        let total_size = (height * width) as usize;
        let mut data: Vec<RGB> = (0..total_size).map(|i| RGB(0, 0, 0)).collect();

        let buffer_size = buffer.len();

        // TODO: Clean up this dirty code
        for i in 0..(total_size) {
            if idx >= buffer_size {
                break;
            } else {
                let rgb = RGB(buffer[idx], buffer[idx + 1], buffer[idx + 2]);
                data[i] = rgb;
                idx += 3;
            }
        }

        PPM {
            magic_number,
            width,
            height,
            max_color_value,
            data,
        }
    }

    fn is_whitespace(byte: u8) -> bool {
        match byte {
            9 | 10 | 11 | 13 | 32 => true,
            _ => false,
        }
    }
}
