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

static MorseBindings: SuperLazyCell<HashMap<char, Vec<MorseSegment>>> = SuperLazyCell::new(|| {
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

    map.insert('q', alloc::vec![]);
    map.insert('r', alloc::vec![]);
    map.insert('s', alloc::vec![]);
    map.insert('t', alloc::vec![]);
    map.insert('u', alloc::vec![]);
    map.insert('v', alloc::vec![]);
    map.insert('w', alloc::vec![]);
    map.insert('x', alloc::vec![]);
    map.insert('y', alloc::vec![]);
    map.insert('z', alloc::vec![]);
    map.insert(' ', alloc::vec![MorseSegment::WordEnd]);
    map.insert('0', alloc::vec![]);
    map.insert('1', alloc::vec![]);
    map.insert('2', alloc::vec![]);
    map.insert('3', alloc::vec![]);
    map.insert('4', alloc::vec![]);
    map.insert('5', alloc::vec![]);
    map.insert('6', alloc::vec![]);
    map.insert('7', alloc::vec![]);
    map.insert('8', alloc::vec![]);
    map.insert('9', alloc::vec![]);
    map.insert('.', alloc::vec![]);
    map.insert('?', alloc::vec![]);
    map.insert(',', alloc::vec![]);
    map.insert('\'', alloc::vec![]);
    map.insert('"', alloc::vec![]);
    map.insert(':', alloc::vec![]);
    map.insert('+', alloc::vec![]);
    map.insert('=', alloc::vec![]);
    map.insert('/', alloc::vec![]);
    map.insert('-', alloc::vec![]);
    map.insert('(', alloc::vec![]);
    map.insert(')', alloc::vec![]);
    map.insert('&', alloc::vec![]);
    map.insert('@', alloc::vec![]);
    map
});

//This super-cursed abomination of sins is just so the compiler will stop yelling about LazyCell not being threadsafe... on the single threaded GBA. Yip-E. I really wish core had LazyLock >.<
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
        &*self.0 //Just learned how much this is doing all at once. The * operator is exactly the same as saying self.deref(), which is to say "get the thing out of self", and * is going to be a different function based on what type self is. A Trait/interface/typeclass is a way of saying "if it has this trait, it will implement the corresponding function"
    }
}

const DOT: u8 = 0;
const DASH: u8 = 1;
const LETTEREND: u8 = 2;
const WORDEND: u8 = 3;

#[repr(u8)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone)]
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
    match temp {
        DOT => MorseSegment::Dot,
        DASH => MorseSegment::Dash,
        LETTEREND => MorseSegment::LetterEnd,
        WORDEND => MorseSegment::WordEnd,
        _ => unreachable!(),
    }
}

fn pack_morse_cluster(
    q0: MorseSegment,
    q1: Option<MorseSegment>,
    q2: Option<MorseSegment>,
    q3: Option<MorseSegment>,
) -> MorseCluster {
    let mut proto_cluster = q0 as u8;

    //if q1 is some, declare a variable called segment which is the value within the q1 option
    if let Some(segment) = q1 {
        proto_cluster |= (segment as u8) << 2;
        // pack the segment
    }

    if let Some(segment) = q2 {
        proto_cluster |= (segment as u8) << 4;
        // pack the segment
    }

    if let Some(segment) = q3 {
        proto_cluster |= (segment as u8) << 6;
        // pack the segment
    }
    proto_cluster
} //try rewriting this with let/else syntax after verifying that it works - would be an easy 

fn pack_morse_segments(segments: Vec<MorseSegment>) -> MorseString {}

struct MorseString {
    length: usize,
    data: Vec<MorseCluster>,
}

impl MorseString {
    fn string_to_morse(inputstring: &str) -> MorseString {
        let mut output = Vec::new();
        for char in inputstring {}
    }
}

#[cfg(test)]
mod tests {
    use core::assert_matches;

    use super::*;

    #[test_case]
    fn it_works(_gba: &mut agb::Gba) {
        let result = extract_morse_segment(0b00111111, 3);
        assert_matches!(result, MorseSegment::Dot);
    }
}

// MorseSegment
//   pack
// MorseCluster = <= 4 MorseSegments
// MorseString = length (segment count) + bunch of MorseClusters
