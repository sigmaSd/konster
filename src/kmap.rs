use crate::kvec::KVec;

/// Map like struct
///
/// This is an ordered Map, that's why it provides functions like `get_by_idx`
///
/// In-order to use it you need to wrap it in a custom type
///
/// ```rust
/// use konster::kvec::KVec;
/// use konster::kmap::KMap;
///
/// struct Map {
///     map: KMap<u8, usize, 200>,
/// }
/// impl Map {
///     const fn new() -> Self {
///         Self {
///             map: KMap {
///                 vec: KVec {
///                     buf: [(0, 0); 200],
///                     cursor: 0,
///                 },
///             },
///         }
///     }
/// }
/// impl Map {
///     const fn get(&self, key: &u8) -> Option<usize> {
///         let mut idx = 0;
///         while idx < self.map.len() {
///             let elem = self.map.vec.get_unchecked(idx);
///             if elem.0 == *key {
///                 return Some(elem.1);
///             }
///             idx += 1;
///         }
///         None
///     }
///     const fn insert(mut self, key: u8, value: usize) -> Self {
///         let mut idx = 0;
///         while idx < self.map.len() {
///             if self.map.vec.get_unchecked(idx).0 == key {
///                 self.map.vec.buf[idx].1 = value;
///                 return self;
///             }
///             idx += 1;
///         }
///         self.map.vec.buf[self.map.vec.cursor] = (key, value);
///         self.map.vec.cursor += 1;
///         self
///     }
/// }
/// ```
pub struct KMap<K, V, const N: usize> {
    /// The backing Vector of the Map
    pub vec: KVec<(K, V), N>,
}

impl<K, V, const N: usize> KMap<K, V, N> {
    /// Returns the number of elements in the Map, also referred to as its ‘length’.
    pub const fn len(&self) -> usize {
        self.vec.len()
    }
    /// Returns true if the Map contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the element at the index, or None if its empty.
    pub const fn get_by_idx(&self, index: usize) -> Option<&V> {
        match self.vec.get(index) {
            Some((_k, v)) => Some(v),
            None => None,
        }
    }
    /// Returns the element at the index, with no bounds check
    pub const fn get_by_idx_unchecked(&self, index: usize) -> &V {
        &self.vec.get_unchecked(index).1
    }
}
