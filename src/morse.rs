//what do I possibly need to do with morse code?
// turn strings into morse code
// turn morse code into strings?
// turn morse code data into signals for the console to make sounds and/or toggle sprites
// parse user input into morse code / string data
// relate a character to a sequence of beeps and boops.
// speed up and slow down

use agb::hash_map::HashMap;
use alloc::vec::Vec;
use core::cell::LazyCell;
use core::ops::Deref;

static MORSE_BINDINGS: SuperLazyCell<HashMap<char, Vec<MorseSegment>>> = SuperLazyCell::new(|| {
    let dot = MorseSegment::Dot;
    let dash = MorseSegment::Dash;
    let mut map = HashMap::new();
    map.insert('a', alloc::vec![dot, dash]);
    map.insert('b', alloc::vec![dash, dot, dot, dot]);
    map.insert('c', alloc::vec![dash, dot, dash, dot]);
    map.insert('d', alloc::vec![dash, dot, dot]);
    map.insert('e', alloc::vec![dot]);
    map.insert('f', alloc::vec![dot, dot, dash, dot]);
    map.insert('g', alloc::vec![dash, dash, dot]);
    map.insert('h', alloc::vec![dot, dot, dot, dot]);
    map.insert('i', alloc::vec![dot, dot]);
    map.insert('j', alloc::vec![dot, dash, dash, dash]);
    map.insert('k', alloc::vec![dash, dot, dash]);
    map.insert('l', alloc::vec![dot, dash, dot, dot]);
    map.insert('m', alloc::vec![dash, dash]);
    map.insert('n', alloc::vec![dash, dot]);
    map.insert('o', alloc::vec![dash, dash, dash]);
    map.insert('p', alloc::vec![dot, dash, dash, dot]);
    map.insert('q', alloc::vec![dash, dash, dot, dash]);
    map.insert('r', alloc::vec![dot, dash, dot]);
    map.insert('s', alloc::vec![dot, dot, dot]);
    map.insert('t', alloc::vec![dash]);
    map.insert('u', alloc::vec![dot, dot, dash]);
    map.insert('v', alloc::vec![dot, dot, dot, dash]);
    map.insert('w', alloc::vec![dot, dash, dash]);
    map.insert('x', alloc::vec![dash, dot, dot, dash]);
    map.insert('y', alloc::vec![dash, dot, dash, dash]);
    map.insert('z', alloc::vec![dash, dash, dot, dot]);
    map.insert(' ', alloc::vec![MorseSegment::WordEnd]);
    map.insert('0', alloc::vec![dash, dash, dash, dash, dash]);
    map.insert('1', alloc::vec![dot, dash, dash, dash, dash]);
    map.insert('2', alloc::vec![dot, dot, dash, dash, dash]);
    map.insert('3', alloc::vec![dot, dot, dot, dash, dash]);
    map.insert('4', alloc::vec![dot, dot, dot, dot, dash]);
    map.insert('5', alloc::vec![dot, dot, dot, dot, dot]);
    map.insert('6', alloc::vec![dash, dot, dot, dot, dot]);
    map.insert('7', alloc::vec![dash, dash, dot, dot, dot]);
    map.insert('8', alloc::vec![dash, dash, dash, dot, dot]);
    map.insert('9', alloc::vec![dash, dash, dash, dash, dot]);
    map.insert('.', alloc::vec![dot, dash, dot, dash, dot, dash]);
    map.insert('?', alloc::vec![dot, dot, dash, dash, dot, dot]);
    map.insert(',', alloc::vec![dash, dash, dot, dot, dash, dash]);
    map.insert('\'', alloc::vec![dot, dash, dash, dash, dash, dot]); // ' char
    map.insert('"', alloc::vec![dot, dash, dot, dot, dash, dot]);
    map.insert(':', alloc::vec![dash, dash, dash, dot, dot, dot]);
    map.insert('+', alloc::vec![dot, dash, dot, dash, dot]);
    map.insert('=', alloc::vec![dash, dot, dot, dot, dash]);
    map.insert('/', alloc::vec![dash, dot, dot, dash, dot]);
    map.insert('-', alloc::vec![dash, dot, dot, dot, dot, dash]);
    map.insert('(', alloc::vec![dash, dot, dash, dash, dot]);
    map.insert(')', alloc::vec![dash, dot, dash, dash, dot, dash]);
    map.insert('&', alloc::vec![dot, dash, dot, dot, dot]);
    map.insert('@', alloc::vec![dot, dash, dash, dot, dash, dot]);
    map
});
//"error" in morse is <HH>

//These super-cursed abominations of sins (the superLazyCell struct and related functions) are just so the compiler will stop yelling about LazyCell not being threadsafe... on the single threaded GBA. Yip-E. I really wish core had LazyLock >.<

struct SuperLazyCell<T, F = fn() -> T>(LazyCell<T, F>);
unsafe impl<T, F: FnOnce() -> T> Sync for SuperLazyCell<T, F> {}
impl<T, F: FnOnce() -> T> SuperLazyCell<T, F> {
    pub const fn new(f: F) -> SuperLazyCell<T, F> {
        SuperLazyCell(LazyCell::new(f))
    }
}
impl<T, F: FnOnce() -> T> Deref for SuperLazyCell<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const DOT: u8 = 0;
const DASH: u8 = 1;
const LETTEREND: u8 = 2;
const WORDEND: u8 = 3;

#[repr(u8)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
enum MorseSegment {
    Dot = DOT,
    Dash = DASH,
    LetterEnd = LETTEREND,
    WordEnd = WORDEND,
}

type MorseCluster = u8;

//Extracts a single morse segment from a cluster of 4
fn extract_morse_segment(morse_cluster: u8, position: usize) -> MorseSegment {
    let mut temp = morse_cluster >> (position * 2); //shifts the given morse_cluster u8 till the 2 bits we care about are the rightmost 2
    temp &= 0b11; //then we mask off just those bits with a bitwise and
    u8_to_morse_segment(temp)
}

fn u8_to_morse_segment(variable: u8) -> MorseSegment {
    match variable {
        DOT => MorseSegment::Dot,
        DASH => MorseSegment::Dash,
        LETTEREND => MorseSegment::LetterEnd,
        WORDEND => MorseSegment::WordEnd,
        _ => unreachable!(),
    }
}

//unpacks all 4 quarters of a morse cluster into a new vec. Note that this function assumes all 4 quarters have valid data.
fn unpack_morse_cluster(morse_cluster: u8) -> Vec<MorseSegment> {
    let temp = alloc::vec![
        &morse_cluster & 0b11,
        &morse_cluster & 0b1100,
        &morse_cluster & 0b110000,
        &morse_cluster & 0b11000000,
    ];
    let mut output = Vec::new();
    for segment in temp.iter() {
        output.push(u8_to_morse_segment(*segment));
    }
    output
}

fn pack_morse_string(segments: Vec<MorseSegment>) -> MorseString {
    let mut new_morse_string = MorseString::new();
    for item in segments {
        new_morse_string.pack_segment(item);
    }
    new_morse_string
}

struct MorseString {
    len: usize,
    data: Vec<MorseCluster>,
}

impl MorseString {
    fn new() -> MorseString {
        MorseString {
            len: 0,
            data: Vec::new(),
        }
    }
    fn pack_segment(&mut self, segment: MorseSegment) {
        let position = self.len % 4;
        let current_segment = self.current_segment();
        match position {
            0 => *current_segment |= segment as u8,
            1 => *current_segment |= (segment as u8) << 2,
            2 => *current_segment |= (segment as u8) << 4,
            3 => *current_segment |= (segment as u8) << 6,
            _ => unreachable!(),
        }
        self.increment_length();
    }

    fn current_segment(&mut self) -> &mut MorseCluster {
        &mut self.data[self.len / 4]
    }
    fn increment_length(&mut self) {
        self.len += 1;
        if self.len.is_multiple_of(4) && self.len / 4 == self.data.len() {
            self.data.push(0b00);
        }
    }
    fn to_text(&self) -> Vec<char> {
        Vec::new()
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use agb::println;
    use core::assert_matches;

    use super::*;

    #[test_case]
    fn test_extract_single_segment(_gba: &mut agb::Gba) {
        let data = 0b00011011;
        assert_eq!(extract_morse_segment(data, 3), MorseSegment::Dot);
        assert_eq!(extract_morse_segment(data, 2), MorseSegment::Dash);
        assert_eq!(extract_morse_segment(data, 1), MorseSegment::LetterEnd);
        assert_eq!(extract_morse_segment(data, 0), MorseSegment::WordEnd);
    }

    #[test_case]
    fn test_read_morse_binding(_gba: &mut agb::Gba) {
        let Some(result) = MORSE_BINDINGS.get(&'a') else {
            panic!("No bindings found");
        };
        let expected_result = &alloc::vec![MorseSegment::Dot, MorseSegment::Dash];
        // println!("{:?}-{:?}", result, expected_result);
        assert_eq!(result, expected_result);
    }
    #[test_case]
    fn test_bindings_length(_gba: &mut agb::Gba) {
        assert_eq!(MORSE_BINDINGS.len(), 51);
    }
}
