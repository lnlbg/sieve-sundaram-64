//! Structure for 1-bit storage.

/// A compact bit storage structure using a vector of u64 words.
pub(crate) struct BitSet {
    bits: Vec<u64>,
}

impl BitSet {
    /// Creates a new BitSet with the specified capacity in bits.
    ///
    /// # Arguments
    ///
    /// * 'size' - The number of bits the BitSet needs to hold.
    pub(crate) fn new(size: usize) -> Self {
        let num_words = (size + 63) / 64;
        Self {
            bits: vec![0; num_words],
        }
    }

    /// Checks if the bit at the given index is set (equals 1).
    #[inline]
    pub(crate) fn is_set(&self, index: usize) -> bool {
        (self.bits[index / 64] & (1 << (index % 64))) != 0
    }

    /// Sets the bit at the given index to 1.
    #[inline]
    pub(crate) fn set(&mut self, index: usize) {
        self.bits[index / 64] |= 1 << (index % 64);
    }
}
