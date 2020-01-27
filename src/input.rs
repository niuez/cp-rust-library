/// Reading from standard input
///
/// `input!` is useful for competition programming.
/// There are some forms.
///
/// - Tuple
///
/// ```
/// use cp_rust_library::*;
/// input! { source = "2 3", ab: (usize, usize), }
/// assert_eq!(ab.0, 2);
/// assert_eq!(ab.1, 3);
/// ```
///
/// - Array
/// ```
/// use cp_rust_library::*;
/// input! { source = "1 2 3 4", a: [usize; 4], }
/// assert_eq!(a, vec![1, 2, 3, 4]);
/// ```
///
/// - String -> Vec<char>
/// ```
/// use cp_rust_library::*;
/// input! { source = "qwerty", s: chars, }
/// assert_eq!('q', s[0]);
/// ```
///
/// - Other
/// ```
/// use cp_rust_library::*;
/// input! { source = "123", a: usize, }
/// assert_eq!(123, a);
/// ```
/// 
/// This macro will use parse::<$type>() to parse string.

use std::io::{ stdout, BufWriter, Write };

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_rec!{ iter, $($r)* }
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_rec!{ iter, $($r)* }
    };
}

#[macro_export]
macro_rules! input_rec {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt, $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_rec! { $iter, $($r)* }
    };
}

#[macro_export]
macro_rules! read_value {

    // tuple
    ($iter:expr, ( $($t:tt), * )) => {
        ( $(read_value!($iter, $t)), * )
    };
    
    // array
    ($iter:expr, [ $t:tt; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    
    // string
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    
    // any other
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[macro_export]
macro_rules! output {
    ($($t:expr),*) => {
        let stdout_out = std::io::stdout();
        let mut out_bufwriter = std::io::BufWriter::new(stdout_out.lock());
        write!(out_bufwriter, $($t),*).unwrap();
    }
}

#[test]
fn input_test() {
    input! {
        source = "2 4\n10 20 30 40",
        n: usize,
        m: usize,
        a: [usize; m],
    }
    assert_eq!(n, 2);
    assert_eq!(m, 4);
    assert_eq!(a, [10, 20, 30, 40]);
}
