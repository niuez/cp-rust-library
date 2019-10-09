use crate::data_structures::node_traits::*;

use std::cmp::Ordering::{ Equal, Greater, Less };

pub trait SplayArrayNode: Node + SizeNode {}
impl<N: Node + SizeNode> SplayArrayNode for N {}

fn depth_fix<N: SplayArrayNode>(mut n: &mut Link<N>, dir: usize) {
    if let Some(ref mut x) = n {
        depth_fix(x.as_mut().child_mut(dir), dir);
        x.as_mut().fix();
    }
}

fn splay<N: SplayArrayNode>(mut root: Box<N>, mut i: usize) -> Box<N> {
    let mut top_left: Link<N> = None;
    let mut top_right: Link<N> = None;
    {
        let mut left = &mut top_left;
        let mut right = &mut top_right;
        loop {
            let ([le, ri], rt) = {
                let mut x = root;
                let alpha = match i.cmp(&size(x.child(0))) {
                    Equal => { root = x; break }
                    Less => { 0 }
                    Greater => { i = i - size(x.child(0)) - 1; 1 }
                };
                let mut y = x.as_mut().take(alpha).unwrap();
                match i.cmp(&size(y.child(0))) {
                    Equal => {
                        if alpha == 0 { ([None, Some(x)], y) }
                        else { ([Some(x), None], y) }
                    }
                    cm => {
                        let beta = if cm == Less { 0 } else { i = i - size(y.child(0)) - 1; 1 };
                        let z = y.as_mut().take(beta).unwrap();
                        let mut res = [None, None];
                        if alpha == beta {
                            x.as_mut().set(alpha, y.as_mut().take(alpha ^ 1));
                            y.as_mut().set(alpha ^ 1, Some(x));
                            res[alpha ^ 1] = Some(y);
                        }
                        else {
                            res[alpha ^ 1] = Some(x);
                            res[beta ^ 1] = Some(y);
                        }
                        (res, z)
                    }
                }
            };
            root = rt;
            if let Some(l) = le {
                *left = Some(l);
                let t = left;
                left = t.as_mut().unwrap().as_mut().child_mut(1);
            }
            if let Some(r) = ri {
                *right = Some(r);
                let t = right;
                right = t.as_mut().unwrap().as_mut().child_mut(0);
            }
        }
        std::mem::swap(&mut root.as_mut().take(0), left);
        std::mem::swap(&mut root.as_mut().take(1), right);
    }
    depth_fix(&mut top_left, 1);
    depth_fix(&mut top_right, 0);
    root.as_mut().set(0, top_left);
    root.as_mut().set(1, top_right);
    root
}

pub fn merge<N: SplayArrayNode>(x: Link<N>, y: Link<N>) -> Link<N> {
    match x {
        Some(x) => {
            let sz = x.size();
            let mut x = splay(x, sz - 1);
            x.as_mut().set(1, y);
            Some(x)
        }
        None => y
    }
}

pub fn split<N: SplayArrayNode>(x: Link<N>, i: usize) -> (Link<N>, Link<N>) {
    assert!(i <= size(&x), "not validate spliting");
    if i == 0 { (None, x) }
    else if i == size(&x) { (x, None) }
    else {
        let mut x = splay(x.unwrap(), i);
        let y = x.as_mut().take(0);
        (y, Some(x))
    }
}
