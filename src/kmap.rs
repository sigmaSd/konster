use crate::kvec::KVec;

/// HashMap like struct
///
/// Inorder to use it you need to wrap it in a custom type
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
///         while idx < self.map.vec.len() {
///             if self.map.vec.get_unchecked(idx).0 == *key {
///                 return Some(self.map.vec.buf[idx].1);
///             }
///             idx += 1;
///         }
///         None
///     }
///     const fn insert(mut self, key: u8, value: usize) -> Self {
///         let mut idx = 0;
///         while idx < self.map.vec.len() {
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
