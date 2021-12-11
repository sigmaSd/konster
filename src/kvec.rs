#[derive(Clone, Copy, PartialEq)]
pub struct GKVec<T, const N: usize> {
    pub buf: [T; N],
    pub idx: usize,
}

pub type KVec<T> = GKVec<T, 50>;

impl<T, const N: usize> GKVec<T, N> {
    pub const fn len(&self) -> usize {
        self.idx
    }
    pub const fn is_empty(&self) -> bool {
        self.idx == 0
    }
    #[must_use]
    pub const fn clear(mut self) -> Self {
        self.idx = 0;
        self
    }
    pub const fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(&self.buf[self.idx - 1])
    }
    pub const fn get(&self, elem_idx: usize) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        if elem_idx >= self.idx {
            return None;
        }
        Some(&self.buf[elem_idx])
    }
}
