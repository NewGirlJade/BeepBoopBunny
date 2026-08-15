use agb::hash_map::HashMap;
use alloc::vec::Vec;
use core::cell::LazyCell;
use core::ops::Deref;

//static bindings from english characters to morse code (source: https://morsecode.world/international/morse.html )
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

// SuperLazyCell and related functions are just so the compiler will stop yelling about LazyCell not being threadsafe. The GBA is single threaded- it's fine.
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

// constants for the MorseSegment Enum to use
const DOT: u8 = 0;
const DASH: u8 = 1;
const LETTEREND: u8 = 2;
const WORDEND: u8 = 3;

//enum to represent all the data one needs to represent morse code compactly
#[repr(u8)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
enum MorseSegment {
    Dot = DOT,
    Dash = DASH,
    LetterEnd = LETTEREND,
    WordEnd = WORDEND,
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
// Four Morse Segments in a trenchcoat
struct MorseCluster(u8);
impl MorseCluster {
    // Extracts 4 morse segments from a full cluster
    fn unpack(&self) -> [MorseSegment; 4] {
        let morse_cluster = self.0;
        [
            u8_to_morse_segment(morse_cluster & 0b11),
            u8_to_morse_segment((morse_cluster & 0b1100) >> 2),
            u8_to_morse_segment((morse_cluster & 0b110000) >> 4),
            u8_to_morse_segment((morse_cluster & 0b11000000) >> 6),
        ]
    }
}
// Converts a u8 intermediate representation to a MorseSegment
fn u8_to_morse_segment(variable: u8) -> MorseSegment {
    match variable {
        DOT => MorseSegment::Dot,
        DASH => MorseSegment::Dash,
        LETTEREND => MorseSegment::LetterEnd,
        WORDEND => MorseSegment::WordEnd,
        _ => {
            agb::println!("error: variable {:?} is invalid", variable);
            unreachable!();
        }
    }
}

// Takes a vector or morse segments and packs them into a MorseString
fn pack_morse_string(segments: &Vec<MorseSegment>) -> MorseString {
    let mut new_morse_string = MorseString::new();
    for item in segments {
        new_morse_string.pack_segment(*item);
    }
    new_morse_string
}

#[cfg_attr(test, derive(Debug))]
// A bundle of morse data
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
        let current_cluster = &mut self.current_cluster().0;
        match position {
            0 => *current_cluster |= segment as u8,
            1 => *current_cluster |= (segment as u8) << 2,
            2 => *current_cluster |= (segment as u8) << 4,
            3 => *current_cluster |= (segment as u8) << 6,
            _ => unreachable!(),
        }
        self.len += 1;
    }

    fn current_cluster(&mut self) -> &mut MorseCluster {
        self.expand_if_full();
        &mut self.data[self.len / 4]
    }
    fn expand_if_full(&mut self) {
        if self.len.is_multiple_of(4) && self.len / 4 == self.data.len() {
            self.data.push(MorseCluster(0b00));
        }
    }
    fn unpack(&self) -> Vec<MorseSegment> {
        let mut segments: Vec<MorseSegment> = Vec::new();
        for cluster in &self.data {
            let bits = cluster.unpack();
            for bit in bits {
                segments.push(bit);
            }
        }
        while segments.len() > self.len {
            segments.pop();
        }
        segments
    }
    fn to_text(&self) -> Vec<char> {
        Vec::new()
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use agb::println;

    use super::*;

    struct TestContext {
        cluster: MorseCluster,
        segment_vec: Vec<MorseSegment>,
        morse_string: MorseString,
    }
    impl TestContext {
        fn new() -> TestContext {
            let dot: MorseSegment = MorseSegment::Dot;
            let dash: MorseSegment = MorseSegment::Dash;
            let lend: MorseSegment = MorseSegment::LetterEnd;
            let wend: MorseSegment = MorseSegment::WordEnd;
            TestContext {
                cluster: MorseCluster(0b00011011),
                segment_vec: alloc::vec![
                    dot, dot, lend, dot, dash, dash, lend, dash, dot, dash, dot, lend, wend,
                ],
                morse_string: pack_morse_string(&alloc::vec![
                    dot, dash, lend, dot, wend, dash, dash, dash, dash, dash, lend, dot, lend,
                    dash, dot, dash, wend
                ]),
            }
        }
    }

    #[test_case]
    fn test_extract_single_segment(_gba: &mut agb::Gba) {
        let ctx = TestContext::new();
        let unpacked = ctx.cluster.unpack();
        assert_eq!(MorseSegment::Dot, unpacked[3]);
        assert_eq!(MorseSegment::Dash, unpacked[2]);
        assert_eq!(MorseSegment::LetterEnd, unpacked[1]);
        assert_eq!(MorseSegment::WordEnd, unpacked[0]);
    }

    #[test_case]
    fn test_read_morse_binding(_gba: &mut agb::Gba) {
        let Some(result) = MORSE_BINDINGS.get(&'a') else {
            panic!("No bindings found");
        };
        let expected_result = &alloc::vec![MorseSegment::Dot, MorseSegment::Dash];
        assert_eq!(result, expected_result);
    }
    #[test_case]
    fn test_bindings_length(_gba: &mut agb::Gba) {
        assert_eq!(MORSE_BINDINGS.len(), 51);
    }
    #[test_case]
    fn test_pack_unpack(_gba: &mut agb::Gba) {
        let ctx = TestContext::new();
        let packed = pack_morse_string(&ctx.segment_vec);
        let unpacked = packed.unpack();
        assert_eq!(unpacked, ctx.segment_vec);
    }

    #[test_case]
    fn test_unpack_pack(_gba: &mut agb::Gba) {
        let ctx = TestContext::new();
        let data = ctx.cluster;
        let intermediate = data.unpack().to_vec();
        let repacked = pack_morse_string(&intermediate);
        assert_eq!(data, repacked.data[0]);
    }
    #[test_case]
    fn test_current_cluster_expands_correctly(_gba: &mut agb::Gba) {
        let mut new = MorseString::new();
        new.current_cluster();
        assert_eq!(new.current_cluster(), MorseString::new().current_cluster());
        new.pack_segment(MorseSegment::Dot);
        assert_eq!(new.len, 1);
    }

    #[test_case]
    fn test_unpack_length(_gba: &mut agb::Gba) {
        let ctx = TestContext::new();
        let m_str = &ctx.morse_string;
        assert_eq!(m_str.unpack().len(), ctx.morse_string.len);
    }
}
/*
~~~test template~~~
#[test_case]
fn test(_gba:&mut agb::Gba){}
let ctx = TestContext::new();
*/
