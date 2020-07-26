pub struct Bitset {
    b: Vec<u64>,
    n: usize,
}

impl Bitset {
    pub fn new(n: usize) -> Self {
        let s = (n + 63) / 64;
        Self { b: std::iter::repeat(0).take(s).collect(), n }
    }

    pub fn set(&mut self, i: usize, v: bool) {
        assert!(i < self.n);
        match v {
            true => self.b[i / 64] |= 1u64 << (i & 63),
            false => self.b[i / 64] &= !(1u64 << (i & 63)),
        }
    }

    pub fn get(&mut self, i: usize) -> bool {
        assert!(i < self.n);
        (self.b[i / 64] & (1u64 << (i & 63))) > 0
    }
}
