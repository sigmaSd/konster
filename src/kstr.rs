use crate::kvec::KVec;

/// String like struct usable on const context
///
/// Currently only handles ASCII strings
///
/// Its generic over its inner buffer size
#[derive(Clone, Copy, PartialEq)]
pub struct KStr<const N: usize> {
    vec: KVec<u8, N>,
}

impl<const N: usize> KStr<N> {
    /// Constructs a new, empty String
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vec: KVec {
                buf: [0; N],
                cursor: 0,
            },
        }
    }
    /// Create a new String from std::primitive::str
    #[must_use]
    pub const fn from_str(string: &str) -> Self {
        let string = string.as_bytes();
        let mut buf = [0; N];
        let mut idx = 0;
        while idx < string.len() {
            buf[idx] = string[idx];
            idx += 1;
        }
        Self {
            vec: KVec { buf, cursor: idx },
        }
    }
    /// Returns a new String with a given String appended onto its end.
    #[must_use]
    pub const fn push_str(mut self, other: &Self) -> Self {
        let mut idx = 0;
        while idx < other.len() {
            match other.get(idx) {
                Some(val) => self = self.push(*val),
                _ => unreachable!(),
            }
            idx += 1;
        }
        self
    }
    /// Returns a new String with a given elem appended to it.
    #[must_use]
    pub const fn push(mut self, elem: u8) -> Self {
        self.vec.buf[self.vec.cursor] = elem;
        self.vec.cursor += 1;
        self
    }
    /// Returns an option of a tuple of:
    /// - new String without the last element
    /// - the last element
    /// Returns None if its empty.
    #[must_use]
    pub const fn pop(mut self) -> Option<(Self, u8)> {
        if self.is_empty() {
            return None;
        }
        self.vec.cursor -= 1;
        let val = self.vec.buf[self.vec.cursor];
        Some((self, val))
    }
    /// Parses the String into a usize.
    pub const fn parse_usize(self) -> usize {
        let mut idx = 0;
        let mut result = 0;
        let mut pow = self.len() - 1;

        loop {
            result += (self.vec.buf[idx] as usize - 48) * 10_usize.pow(pow as _);
            if pow == 0 {
                break;
            }
            pow -= 1;
            idx += 1;
        }
        result as _
    }
    // Forword kvec methods
    /// Returns a new String with elements cleared.
    pub const fn clear(mut self) -> Self {
        self.vec.cursor = 0;
        self
    }
    /// Returns true if the String contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of elements in the String, also referred to as its ‘length’.
    pub const fn len(&self) -> usize {
        self.vec.len()
    }
    /// Returns the element at the index, or None if its empty.
    pub const fn get(&self, elem_idx: usize) -> Option<&u8> {
        self.vec.get(elem_idx)
    }
    /// Returns the last element of the String, or None if it is empty.
    pub const fn last(&self) -> Option<&u8> {
        self.vec.last()
    }
}

// Runtime methods
impl<const N: usize> KStr<N> {
    /// [Runtime method] Create an std::primitive::str from this String
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.vec.buf[..self.vec.cursor]).unwrap()
    }
}
impl<const N: usize> std::fmt::Debug for KStr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod kost_test {
    use super::*;

    const _: () = {
        let mut str = KStr::<20>::new();
        str = str.push(4);
        str = str.push(5);
        match str.pop() {
            Some((str, val)) => {
                if !matches!(val, 5) {
                    panic!("val is different then 5");
                }
                if !matches!(str.last(), Some(4)) {
                    panic!("val is different then 4");
                }
            }
            None => unreachable!(),
        }
    };
}
