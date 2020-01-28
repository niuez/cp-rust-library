use std::io::{ stdout, BufWriter, Write };

#[macro_export]
macro_rules! ioset {
    ($inp:ident, $buf:ident) => {
        inset!($inp);
        outset!($buf);
    }
}

#[macro_export]
macro_rules! inset {
    (source = $s:expr, $iter:ident) => {
        let mut $iter = $s.split_whitespace();
    };
    ($iter:ident) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut $iter = s.split_whitespace();
    }
}

#[macro_export]
macro_rules! input {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt, $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input! { $iter, $($r)* }
    };
    ($iter:expr, ($var:expr) : $t:tt, $($r:tt)*) => {
        $var = read_value!($iter, $t);
        input! { $iter, $($r)* }
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
macro_rules! outset {
    ($buf:ident) => {
        let sout = stdout();
        let mut $buf = BufWriter::new(sout.lock());
    }
}

#[macro_export]
macro_rules! output {
    ($buf:expr, $($t:expr),*) => {
        write!($buf, $($t),*).unwrap();
    }
}

