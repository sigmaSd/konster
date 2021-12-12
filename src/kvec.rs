/// Vector like struct usable in const context
///
/// Its generic over its inner buffer size
#[derive(Clone, Copy, PartialEq)]
pub struct KVec<T, const N: usize> {
    /// The Vector backing buffer
    pub buf: [T; N],
    /// The cursor that determines the actual Vector length
    pub cursor: usize,
}

impl<T, const N: usize> KVec<T, N> {
    /// Returns the number of elements in the Vector, also referred to as its ‘length’.
    pub const fn len(&self) -> usize {
        self.cursor
    }
    /// Returns true if the Vector contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.cursor == 0
    }
    /// Returns a new Vector with elements cleared
    #[must_use]
    pub const fn clear(mut self) -> Self {
        self.cursor = 0;
        self
    }
    /// Returns the element at the index, with no bounds check
    pub const fn get_unchecked(&self, elem_idx: usize) -> &T {
        &self.buf[elem_idx]
    }
    /// Returns the element at the index, or None if its empty.
    pub const fn get(&self, elem_idx: usize) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        if elem_idx >= self.cursor {
            return None;
        }
        Some(&self.buf[elem_idx])
    }
    /// Returns the last element of the slice, or None if it is empty.
    pub const fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(&self.buf[self.cursor - 1])
    }
}
