#![allow(unused_imports, dead_code)]
use std::{cmp::{Reverse, max, min}, collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}, f64::consts::PI, fmt::{Write, format}, io::{Read, stdin}, iter::Peekable, mem::swap, str::{FromStr, SplitWhitespace}};

fn main() {
    let mut buf = String::new();
    let mut out = String::new();

    stdin().read_to_string(&mut buf).unwrap();
    let mut scan = Scanner::new(buf.split_whitespace().peekable());

    let mut tc = 1;
    if tc != 1 { tc = scan.next::<usize>(); }
    for i in 0..tc {
        solve(&mut scan, &mut out, i);
    }
    print!("{out}");
}

// https://youtu.be/r89YusWbFZE
fn solve<'a>(scan: &mut Scanner<'a, Peekable<SplitWhitespace<'a>>>, out: &mut String, _tc: usize) {
    let n: usize = scan.next();
    let mut a = vec![vec![0usize; 20]; 20];
    for i in 0..20 {
        for j in 0..20 {
            a[i][j] = scan.next();
        }
    }

    let mut ans = 0;
    for i in 0..20 {
        for j in 0..(20 + 1 - n) {
            let mut cur = 1;
            for k in 0..n {
                cur *= a[i][j + k];
            }
            ans = max(ans, cur);
        }
    }
    
    for j in 0..20 {
        for i in 0..(20 + 1 - n) {
            let mut cur = 1;
            for k in 0..n {
                cur *= a[i + k][j];
            }
            ans = max(ans, cur);
        }
    }

    for i in 0..(20 + 1 - n) {
        for j in 0..(20 + 1 - n) {
            let mut cur = 1;
            for k in 0..n {
                cur *= a[i + k][j + k];
            }
            ans = max(ans, cur);
        }
    }

    for i in 0..(20 + 1 - n) {
        for j in 0..(20 + 1 - n) {
            let mut cur = 1;
            for k in 0..n {
                cur *= a[i + n - 1 - k][j + k];
            }
            ans = max(ans, cur);
        }
    }
    writeln!(out, "{ans}").unwrap();
}

struct Scanner<'a, I: Iterator<Item = &'a str>> {
    iter: I,
}

impl<'a, I: Iterator<Item = &'a str>> Scanner<'a, I> {
    fn new(from: I) -> Self {
        Self { iter: from }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}
