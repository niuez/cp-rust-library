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
        read_value!($iter, String).chars(),collect::<Vec<char>>()
    };
    
    // any other
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
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
