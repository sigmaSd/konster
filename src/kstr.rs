use crate::kvec::GKVec;

#[derive(Clone, Copy, PartialEq)]
pub struct GKStr<const N: usize> {
    vec: GKVec<u8, N>,
}

pub type KStr = GKStr<50>;

impl<const N: usize> GKStr<N> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vec: GKVec {
                buf: [0; N],
                idx: 0,
            },
        }
    }
    #[must_use]
    pub const fn push_kstr(mut self, string: &Self) -> Self {
        let mut idx = 0;
        while idx < string.len() {
            match string.get(idx) {
                Some(val) => self = self.push(*val),
                _ => unreachable!(),
            }
            idx += 1;
        }
        self
    }
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
            vec: GKVec { buf, idx },
        }
    }
    #[must_use]
    pub const fn push(mut self, elem: u8) -> Self {
        self.vec.buf[self.vec.idx] = elem;
        self.vec.idx += 1;
        self
    }
    #[must_use]
    pub const fn pop(mut self) -> Option<(Self, u8)> {
        if self.is_empty() {
            return None;
        }
        self.vec.idx -= 1;
        let val = self.vec.buf[self.vec.idx];
        Some((self, val))
    }
    pub const fn parse(self) -> u16 {
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
    pub const fn clear(mut self) -> Self {
        self.vec.idx = 0;
        self
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub const fn len(&self) -> usize {
        self.vec.len()
    }
    pub const fn get(&self, elem_idx: usize) -> Option<&u8> {
        self.vec.get(elem_idx)
    }
    pub const fn last(&self) -> Option<&u8> {
        self.vec.last()
    }
}

// Runtime methods
impl<const N: usize> GKStr<N> {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.vec.buf[..self.vec.idx]).unwrap()
    }
}
impl<const N: usize> std::fmt::Debug for GKStr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod kost_test {
    use super::*;

    const _: () = {
        let mut str = GKStr::<20>::new();
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
