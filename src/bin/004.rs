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
    //let n: usize = scan.next();

    let mut ans = 0;
    for i in 1..=9 {
        for j in 0..=9 {
            for k in 0..=9 {
                let cur: i32 = i * 100_000 + j * 10_000 + k * 1_000 + k * 100 + j * 10 + i;
                for i in 1..=cur.isqrt() {
                    if cur % i != 0 { continue; }
                    let j = cur / i;
                    if i >= 1000 || j < 100 { break; }
                    if 100 <= i && i < 1000 && 100 <= j && j < 1000 {
                        ans = max(ans, cur);
                        break;
                    }
                }
            }
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
