struct Permutations<T> {
    a: Vec<T>,
    first: bool,
}

impl<T> Permutations<T> {
    pub fn new(a: Vec<T>) -> Self {
        Self { a, first: true }
    }
}

impl<T: Clone + Ord> std::iter::Iterator for Permutations<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.a.clone());
        }
        let mut k = self.a.len() - 1;
        for i in (0..self.a.len() - 1).rev() {
            if self.a[i] < self.a[i + 1] {
                k = i;
                break;
            }
        }
        if k == self.a.len() - 1 {
            return None
        }
        let mut l = k + 1;
        for i in (0..self.a.len()).rev() {
            if self.a[k] < self.a[i] {
                l = i;
                break;
            }
        }
        self.a.swap(k, l);
        self.a[(k + 1..)].reverse();
        Some(self.a.clone())
    }
}
