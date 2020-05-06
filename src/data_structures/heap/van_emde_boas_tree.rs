pub trait VanEmdeBoasTree {
    fn new() -> Self;
    fn universe_len() -> u64;
    fn insert(&mut self, x: u64);
    fn erase(&mut self, x: u64);
    fn find(&self, x: u64) -> bool;
    fn any(&self) -> bool;
    fn lower_bound(&self, x: u64) -> Option<u64>;
}

pub struct VEBTree64 {
    flag: u64,
}

impl VanEmdeBoasTree for VEBTree64 {
    fn new() -> Self { VEBTree64 { flag: 0 } }
    fn universe_len() -> u64 { 64 }
    fn insert(&mut self, x: u64) {
        self.flag |= 1u64 << x;
    }
    fn erase(&mut self, x: u64) {
        self.flag &= !(1 << x);
    }
    fn find(&self, x: u64) -> bool {
        0 < (self.flag & (1 << x))
    }
    fn any(&self) -> bool {
        0 < self.flag
    }
    fn lower_bound(&self, x: u64) -> Option<u64> {
        println!("{:b}, {} = {:b}", self.flag, x, (self.flag & !((1 << x) - 1)));
        println!("{}", (self.flag & !((1 << x) - 1)).trailing_zeros());
        match (self.flag & !((1 << x) - 1)).trailing_zeros() {
            64 => None,
            l => Some(l as u64),
        }
    }
}

pub struct VEBTreeNode<T> {
    summary: T,
    cluster: Box<[T]>,
}

impl<T: VanEmdeBoasTree> VanEmdeBoasTree for VEBTreeNode<T> {
    fn new() -> Self {
        VEBTreeNode {
            summary: T::new(),
            cluster: (0..T::universe_len())
                .map(|_| T::new())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
    fn universe_len() -> u64 { T::universe_len().pow(2) }
    fn insert(&mut self, x: u64) {
        let (i, x) = (x / T::universe_len(), x % T::universe_len());
        self.summary.insert(i);
        self.cluster[i as usize].insert(x);
    }
    fn erase(&mut self, x: u64) {
        let (i, x) = (x / T::universe_len(), x % T::universe_len());
        self.cluster[i as usize].erase(x);
        if !self.cluster[i as usize].any() {
            self.summary.erase(i);
        }
    }
    fn find(&self, x: u64) -> bool {
        let (i, x) = (x / T::universe_len(), x % T::universe_len());
        self.cluster[i as usize].find(x)
    }
    fn any(&self) -> bool {
        self.summary.any()
    }
    fn lower_bound(&self, x: u64) -> Option<u64> {
        let (i, x) = (x / T::universe_len(), x % T::universe_len());
        println!("{}, {}", i, x);
        if self.summary.find(i) {
            if let Some(ans) = self.cluster[i as usize].lower_bound(x) {
                return Some(ans + i * T::universe_len());
            }
        }
        if i + 1 < T::universe_len() {
            self.summary
                .lower_bound(i + 1)
                .map_or(None, |l| {
                    self.cluster[l as usize].lower_bound(0).map(|x| x + l * T::universe_len())
                })
        }
        else { None }
    }
}

#[test]
fn veb_find_test() {
    let mut veb = VEBTreeNode::<VEBTree64>::new();
    veb.insert(2);
    veb.insert(3);
    veb.insert(4);
    veb.insert(5);
    veb.insert(7);
    veb.insert(14);
    veb.insert(15);
    assert!(VEBTreeNode::<VEBTree64>::universe_len() == 64 * 64);
    assert!(veb.find(2));
    assert!(veb.find(4));
    assert!(veb.find(5));
    assert!(!veb.find(6));
    assert!(veb.find(14));
    assert!(veb.find(15));
    assert!(!veb.find(32));
}

#[test]
fn veb_lower_bound_test() {
    let mut veb = VEBTreeNode::<VEBTree64>::new();
    veb.insert(2);
    veb.insert(3);
    veb.insert(4);
    veb.insert(5);
    veb.insert(7);
    veb.insert(14);
    veb.insert(15);
    veb.insert(64 * 32);
    assert!(VEBTreeNode::<VEBTree64>::universe_len() == 64 * 64);
    assert_eq!(veb.lower_bound(2), Some(2));
    assert_eq!(veb.lower_bound(0), Some(2));
    assert_eq!(veb.lower_bound(6), Some(7));
    assert_eq!(veb.lower_bound(32), Some(64 * 32));
    assert_eq!(veb.lower_bound(10), Some(14));
    assert_eq!(veb.lower_bound(64 * 32 + 1), None);
}

#[test]
fn veb_erase_test() {
    let mut veb = VEBTreeNode::<VEBTree64>::new();
    veb.insert(2);
    veb.insert(3);
    veb.insert(4);
    veb.insert(5);
    veb.insert(7);
    veb.insert(14);
    veb.insert(15);
    veb.insert(64 * 32);
    veb.erase(4);
    veb.erase(15);
    veb.erase(64 * 32);
    assert!(VEBTreeNode::<VEBTree64>::universe_len() == 64 * 64);
    assert!(veb.find(2));
    assert!(!veb.find(4));
    assert!(veb.find(5));
    assert!(!veb.find(6));
    assert!(veb.find(14));
    assert!(!veb.find(15));
    assert!(!veb.find(32));
    assert_eq!(veb.lower_bound(32), None);
    assert_eq!(veb.lower_bound(64 * 32 + 1), None);
}
