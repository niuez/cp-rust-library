pub trait ToVec {
    type Item;
    fn tovec(self) -> Vec<Self::Item>;
}

impl<I: std::iter::Iterator> ToVec for I {
    type Item = <Self as std::iter::Iterator>::Item;
    fn tovec(self) -> Vec<Self::Item> {
        self.collect::<Vec<Self::Item>>()
    }
}
