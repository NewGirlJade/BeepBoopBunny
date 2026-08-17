use agb::hash_map::HashMap;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::LazyCell;
use core::ops::Deref;

//static bindings from english characters to morse code (source: https://morsecode.world/international/morse.html )
static MORSE_BINDINGS: SuperLazyCell<HashMap<char, Vec<MorseSegment>>> = SuperLazyCell::new(|| {
    let dot = MorseSegment::Dot;
    let dash = MorseSegment::Dash;
    let mut map = HashMap::new();
    map.insert('a', vec![dot, dash]);
    map.insert('b', vec![dash, dot, dot, dot]);
    map.insert('c', vec![dash, dot, dash, dot]);
    map.insert('d', vec![dash, dot, dot]);
    map.insert('e', vec![dot]);
    map.insert('f', vec![dot, dot, dash, dot]);
    map.insert('g', vec![dash, dash, dot]);
    map.insert('h', vec![dot, dot, dot, dot]);
    map.insert('i', vec![dot, dot]);
    map.insert('j', vec![dot, dash, dash, dash]);
    map.insert('k', vec![dash, dot, dash]);
    map.insert('l', vec![dot, dash, dot, dot]);
    map.insert('m', vec![dash, dash]);
    map.insert('n', vec![dash, dot]);
    map.insert('o', vec![dash, dash, dash]);
    map.insert('p', vec![dot, dash, dash, dot]);
    map.insert('q', vec![dash, dash, dot, dash]);
    map.insert('r', vec![dot, dash, dot]);
    map.insert('s', vec![dot, dot, dot]);
    map.insert('t', vec![dash]);
    map.insert('u', vec![dot, dot, dash]);
    map.insert('v', vec![dot, dot, dot, dash]);
    map.insert('w', vec![dot, dash, dash]);
    map.insert('x', vec![dash, dot, dot, dash]);
    map.insert('y', vec![dash, dot, dash, dash]);
    map.insert('z', vec![dash, dash, dot, dot]);
    map.insert(' ', vec![MorseSegment::WordEnd]);
    map.insert('0', vec![dash, dash, dash, dash, dash]);
    map.insert('1', vec![dot, dash, dash, dash, dash]);
    map.insert('2', vec![dot, dot, dash, dash, dash]);
    map.insert('3', vec![dot, dot, dot, dash, dash]);
    map.insert('4', vec![dot, dot, dot, dot, dash]);
    map.insert('5', vec![dot, dot, dot, dot, dot]);
    map.insert('6', vec![dash, dot, dot, dot, dot]);
    map.insert('7', vec![dash, dash, dot, dot, dot]);
    map.insert('8', vec![dash, dash, dash, dot, dot]);
    map.insert('9', vec![dash, dash, dash, dash, dot]);
    map.insert('.', vec![dot, dash, dot, dash, dot, dash]);
    map.insert('?', vec![dot, dot, dash, dash, dot, dot]);
    map.insert(',', vec![dash, dash, dot, dot, dash, dash]);
    map.insert('\'', vec![dot, dash, dash, dash, dash, dot]); // ' char
    map.insert('"', vec![dot, dash, dot, dot, dash, dot]);
    map.insert(':', vec![dash, dash, dash, dot, dot, dot]);
    map.insert('+', vec![dot, dash, dot, dash, dot]);
    map.insert('=', vec![dash, dot, dot, dot, dash]);
    map.insert('/', vec![dash, dot, dot, dash, dot]);
    map.insert('-', vec![dash, dot, dot, dot, dot, dash]);
    map.insert('(', vec![dash, dot, dash, dash, dot]);
    map.insert(')', vec![dash, dot, dash, dash, dot, dash]);
    map.insert('&', vec![dot, dash, dot, dot, dot]);
    map.insert('@', vec![dot, dash, dash, dot, dash, dot]);
    map
});

static REVERSE_MORSE_BINDINGS: SuperLazyCell<HashMap<Vec<MorseSegment>, char>> =
    SuperLazyCell::new(|| {
        let dot = MorseSegment::Dot;
        let dash = MorseSegment::Dash;
        let mut map = HashMap::new();
        map.insert(vec![dot, dash], 'a');
        map.insert(vec![dash, dot, dot, dot], 'b');
        map.insert(vec![dash, dot, dash, dot], 'c');
        map.insert(vec![dash, dot, dot], 'd');
        map.insert(vec![dot], 'e');
        map.insert(vec![dot, dot, dash, dot], 'f');
        map.insert(vec![dash, dash, dot], 'g');
        map.insert(vec![dot, dot, dot, dot], 'h');
        map.insert(vec![dot, dot], 'i');
        map.insert(vec![dot, dash, dash, dash], 'j');
        map.insert(vec![dash, dot, dash], 'k');
        map.insert(vec![dot, dash, dot, dot], 'l');
        map.insert(vec![dash, dash], 'm');
        map.insert(vec![dash, dot], 'n');
        map.insert(vec![dash, dash, dash], 'o');
        map.insert(vec![dot, dash, dash, dot], 'p');
        map.insert(vec![dash, dash, dot, dash], 'q');
        map.insert(vec![dot, dash, dot], 'r');
        map.insert(vec![dot, dot, dot], 's');
        map.insert(vec![dash], 't');
        map.insert(vec![dot, dot, dash], 'u');
        map.insert(vec![dot, dot, dot, dash], 'v');
        map.insert(vec![dot, dash, dash], 'w');
        map.insert(vec![dash, dot, dot, dash], 'x');
        map.insert(vec![dash, dot, dash, dash], 'y');
        map.insert(vec![dash, dash, dot, dot], 'z');
        map.insert(vec![MorseSegment::WordEnd], ' ');
        map.insert(vec![dash, dash, dash, dash, dash], '0');
        map.insert(vec![dot, dash, dash, dash, dash], '1');
        map.insert(vec![dot, dot, dash, dash, dash], '2');
        map.insert(vec![dot, dot, dot, dash, dash], '3');
        map.insert(vec![dot, dot, dot, dot, dash], '4');
        map.insert(vec![dot, dot, dot, dot, dot], '5');
        map.insert(vec![dash, dot, dot, dot, dot], '6');
        map.insert(vec![dash, dash, dot, dot, dot], '7');
        map.insert(vec![dash, dash, dash, dot, dot], '8');
        map.insert(vec![dash, dash, dash, dash, dot], '9');
        map.insert(vec![dot, dash, dot, dash, dot, dash], '.');
        map.insert(vec![dot, dot, dash, dash, dot, dot], '?');
        map.insert(vec![dash, dash, dot, dot, dash, dash], ',');
        map.insert(vec![dot, dash, dash, dash, dash, dot], '\'');
        map.insert(vec![dot, dash, dot, dot, dash, dot], '"');
        map.insert(vec![dash, dash, dash, dot, dot, dot], ':');
        map.insert(vec![dot, dash, dot, dash, dot], '+');
        map.insert(vec![dash, dot, dot, dot, dash], '=');
        map.insert(vec![dash, dot, dot, dash, dot], '/');
        map.insert(vec![dash, dot, dot, dot, dot, dash], '-');
        map.insert(vec![dash, dot, dash, dash, dot], '(');
        map.insert(vec![dash, dot, dash, dash, dot, dash], ')');
        map.insert(vec![dot, dash, dot, dot, dot], '&');
        map.insert(vec![dot, dash, dash, dot, dash, dot], '@');
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
            agb::println!("error: variable {:?} is invalid.", variable);
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

fn to_string(segments: &Vec<MorseSegment>) -> String {
    let mut new_string = String::new();
    let mut char_segments: Vec<MorseSegment> = Vec::new();
    for segment in segments {
        match segment {
            MorseSegment::Dot | MorseSegment::Dash => char_segments.push(*segment),
            MorseSegment::LetterEnd | MorseSegment::WordEnd => {
                flush_char_segments_to_string(&mut char_segments, &mut new_string);
            }
        }
        if *segment == MorseSegment::WordEnd {
            new_string.push(' ');
        }
    }
    new_string
}

fn flush_char_segments_to_string(char_segments: &mut Vec<MorseSegment>, new_string: &mut String) {
    if char_segments.is_empty() {
        return;
    };
    if let Some(found_char) = REVERSE_MORSE_BINDINGS.get(char_segments) {
        new_string.push(*found_char);
    } else {
        agb::println!(
            "\nNonfatal error: {:?} is not a valid morse sequence. Skipping.\n",
            *char_segments
        );
    };
    *char_segments = Vec::new();
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
    fn stringify(&self) -> String {
        let segments = self.unpack();
        to_string(&segments)
    }
}

fn string_to_morse(input: alloc::string::String) -> MorseString {
    let mut temp: Vec<MorseSegment> = Vec::new();
    for char in input.chars() {
        let Some(chardata) = MORSE_BINDINGS.get(&char.to_ascii_lowercase()) else {
            agb::println!(
                "\nNonfatal error: {:?} is not a valid character. Skipping\n",
                char
            );
            continue;
        };
        temp.append(&mut chardata.clone());
        if char != ' ' {
            temp.push(MorseSegment::LetterEnd);
        }
    }
    temp.push(MorseSegment::WordEnd);
    pack_morse_string(&temp)
}

///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use agb::println;
    use alloc::borrow::ToOwned;

    struct TestContext {
        cluster: MorseCluster,
        segment_vec: Vec<MorseSegment>,
        morse_string: MorseString,
        wife_vec: Vec<MorseSegment>,
    }
    impl TestContext {
        fn new() -> TestContext {
            let dot: MorseSegment = MorseSegment::Dot;
            let dash: MorseSegment = MorseSegment::Dash;
            let lend: MorseSegment = MorseSegment::LetterEnd;
            let wend: MorseSegment = MorseSegment::WordEnd;
            TestContext {
                cluster: MorseCluster(0b00011011),
                segment_vec: vec![
                    dot, dot, lend, dot, dash, dash, lend, dash, dot, dash, dot, lend, wend,
                ],
                morse_string: pack_morse_string(&vec![
                    dot, dash, lend, dot, lend, wend, dash, dash, dash, dash, dash, lend, dot,
                    lend, dash, dot, dash, lend, wend,
                ]),
                wife_vec: vec![
                    dot, dot, dot, dot, lend, dot, dot, lend, wend, dash, dot, dash, lend, dot,
                    dash, lend, dash, dot, dash, dash, lend, dot, dash, dot, dot, lend, dot, dash,
                    lend, wend, dash, dash, dash, dot, dot, dot, lend, dot, dot, dot, dash, dash,
                    lend, wend,
                ],
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
        let expected_result = &vec![MorseSegment::Dot, MorseSegment::Dash];
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
    #[test_case]
    fn test_wife_greeting(_gba: &mut agb::Gba) {
        let ctx = TestContext::new();
        let printable = string_to_morse("hi Kayla :3".to_owned()).unpack();
        println!("\n{:?}\n", printable);
        assert_eq!(printable, ctx.wife_vec);
    }

    #[test_case]
    fn test_empty_ms_pack(_gba: &mut agb::Gba) {
        let empty = pack_morse_string(&Vec::new());
        let also_empty = MorseString::new();
        assert_eq!(empty.len, also_empty.len);
        assert_eq!(empty.data, also_empty.data);
    }
}

/*
~~~test template~~~
#[test_case]
fn test(_gba:&mut agb::Gba){
let ctx = TestContext::new();
}

println!("\n{:?"}\n",   );
*/
