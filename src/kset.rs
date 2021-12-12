use crate::kvec::KVec;

/// Set like struct
///
/// This is an ordered Set
///
/// In-order to use it you need to wrap it in a custom type
///
/// ```rust
/// use konster::kvec::KVec;
/// use konster::kset::KSet;
///
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// struct CKSet<T> {
///     set: KSet<T, 50>,
/// }
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
/// enum Attribute {
///     Bold,
///     Hidden,
/// }
/// impl CKSet<Attribute> {
///     const fn new() -> Self {
///         Self {
///             set: KSet {
///                 vec: KVec {
///                     buf: [Attribute::Hidden; 50],
///                     cursor: 0,
///                 },
///             },
///         }
///     }
///     #[must_use]
///     const fn insert(mut self, attribute: Attribute) -> Self {
///         let mut idx = 0;
///         while idx < self.set.len() {
///             let elem = *self.set.get_by_idx_unchecked(idx);
///             if attribute as u8 == elem as u8 {
///                 self.set.vec.buf[idx] = attribute;
///                 return self;
///             }
///             idx += 1;
///         }
///         self.set.vec.buf[self.set.vec.cursor] = attribute;
///         self.set.vec.cursor += 1;
///         self
///     }
/// }
/// ```
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct KSet<T, const N: usize> {
    /// The backing Vector of the Map
    pub vec: KVec<T, N>,
}
impl<T, const N: usize> KSet<T, N> {
    /// Returns the number of elements in the Map, also referred to as its ‘length’.
    pub const fn len(&self) -> usize {
        self.vec.len()
    }
    /// Returns true if the Map contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the element at the index, or None if its empty.
    pub const fn get_by_idx(&self, index: usize) -> Option<&T> {
        match self.vec.get(index) {
            Some(v) => Some(v),
            None => None,
        }
    }
    /// Returns the element at the index, with no bounds check
    pub const fn get_by_idx_unchecked(&self, index: usize) -> &T {
        self.vec.get_unchecked(index)
    }
}
