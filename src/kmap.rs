use std::mem::ManuallyDrop;

use crate::kvec::KVec;

pub struct KMap<K, V, const N: usize> {
    vec: KVec<(K, V), N>,
}

impl<K, V, const N: usize> KMap<K, V, N> {
    pub const fn insert(mut self, key: K, value: V) -> Self {
        let mut idx = 0;
        while idx < self.vec.len() {
            if matches!(&self.vec.get_unchecked(idx).0, key) {
                let mut buf = ManuallyDrop::new(self.vec.buf);
                //buf[self.vec.cursor - 1].1 = value;
                self.vec.buf[self.vec.cursor - 1].1 = value;
                return self;
            }
            idx += 1;
        }
        //self.vec.buf[self.vec.cursor] = (key, value);
        self
    }
}
