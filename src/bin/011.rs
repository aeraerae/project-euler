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

    let mut dp = vec![vec![0usize; 41]; 41];
    dp[0][0] = 1;
    for i in 1..=40 {
        for j in 0..=i {
            if j == 0 || j == i {
                dp[i][j] = 1;
                continue;
            }
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }

    writeln!(out, "{}", dp[n * 2][n]).unwrap();
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
