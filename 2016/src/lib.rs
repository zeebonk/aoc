use std::io;
use std::io::Read;

pub fn read_from_stdin() -> Option<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let result = handle.read_to_string(&mut buffer);

    match result {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

pub mod day_1 {
    pub enum Heading {
        North = 1,
        East = 2,
        South = 3,
        West = 4,
    }

    pub struct Santa {
        pub x: i32,
        pub y: i32,
        heading: Heading,
    }

    impl Santa {
        pub fn new() -> Santa {
            Santa {
                x: 0,
                y: 0,
                heading: Heading::North,
            }
        }

        pub fn turn_left(&mut self) {
            self.heading = match self.heading {
                Heading::North => Heading::West,
                Heading::East => Heading::North,
                Heading::South => Heading::East,
                Heading::West => Heading::South,
            };
        }

        pub fn turn_right(&mut self) {
            self.heading = match self.heading {
                Heading::North => Heading::East,
                Heading::East => Heading::South,
                Heading::South => Heading::West,
                Heading::West => Heading::North,
            };
        }

        pub fn move_forwards(&mut self, distance: i32) {
            match self.heading {
                Heading::North => self.y += distance,
                Heading::East => self.x += distance,
                Heading::South => self.y -= distance,
                Heading::West => self.x -= distance,
            };
        }
    }
}

pub mod day_3 {
    pub fn valid_triangle(x: i32, y: i32, z: i32) -> bool {
        (x + y > z) && (y + z > x) && (x + z > y)
    }
}


pub mod day_5 {
    pub fn byte_to_hex_char(value: u8) -> char {
        match value {
            v if v < 10 => (48 + v) as char,
            v if v < 16 => (97 - 10 + v) as char,
            _ => panic!("Unsupported byte value"),
        }
    }

    pub fn has_5_leading_zeros_in_hex(buf: &[u8]) -> bool {
        return buf[0] == 0 && buf[1] == 0 && buf[2] & 0b1111_0000 == 0;
    }
}
