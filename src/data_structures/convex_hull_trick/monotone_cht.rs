use super::{ Line, LineDecimal };

use std::collections::VecDeque;

pub struct MonotoneCHT<T: LineDecimal> {
    lines: VecDeque<Line<T>>,
}

impl<T: LineDecimal> MonotoneCHT<T> {
    pub fn new() -> Self { MonotoneCHT { lines: VecDeque::new() } }
    fn check(ln0: &Line<T>, ln1: &Line<T>, ln2: &Line<T>) -> bool {
        if ln0.a == ln1.a { ln0.b < ln1.b }
        else if ln1.a == ln2.a { ln1.b > ln2.b }
        else {
            (ln1.b - ln0.b) * (ln1.a - ln2.a) >= (ln2.b - ln1.b) * (ln0.a - ln1.a)
        }
    }
    pub fn push_front(&mut self, ln: Line<T>) {
        while 1 < self.lines.len() && MonotoneCHT::check(&ln, &self.lines[0], &self.lines[1]) {
            self.lines.pop_front();
        }
        self.lines.push_front(ln);
    }
    pub fn push_back(&mut self, ln: Line<T>) {
        let n = self.lines.len();
        while 1 < self.lines.len() && MonotoneCHT::check(&self.lines[n - 2], &self.lines[n - 1], &ln) {
            self.lines.pop_back();
        }
        self.lines.push_back(ln);
    }

    pub fn min_value(&self, x: T) -> T {
        let mut ok = 0;
        let mut ng = self.lines.len();
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let ln0 = &self.lines[mid - 1];
            let ln1 = &self.lines[mid];
            if  (ln1.b - ln0.b) <= x * (ln0.a - ln1.a) {
                ok = mid;
            }
            else {
                ng = mid;
            }
        }
        self.lines[ok].get(x)
    }

    pub fn incl_query(&self) -> MonotoneCHTInclQuery<T> { MonotoneCHTInclQuery { lines: &self.lines, i: 0 } }
}

pub struct MonotoneCHTInclQuery<'a, T: LineDecimal> {
    lines: &'a VecDeque<Line<T>>,
    i: usize,
}

impl<'a, T: LineDecimal> MonotoneCHTInclQuery<'a, T> {
    pub fn min_value(&mut self, x: T) -> T {
        while self.i + 1 < self.lines.len() && self.lines[self.i].get(x) >= self.lines[self.i + 1].get(x) {
            self.i += 1;
        }
        self.lines[self.i].get(x)
    }
}

#[test]
fn cht_test() {
    let mut cht = MonotoneCHT::new();
    cht.push_back(Line::new(2i64, 3));
    cht.push_back(Line::new(-1, 4));
    assert_eq!(cht.min_value(-1), 1);
    assert_eq!(cht.min_value(0), 3);
    assert_eq!(cht.min_value(1), 3);

    let mut incl = cht.incl_query();
    assert_eq!(incl.min_value(-1), 1);
    assert_eq!(incl.min_value(0), 3);
    assert_eq!(incl.min_value(1), 3);
}
