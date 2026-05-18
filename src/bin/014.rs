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

    let mut hmap = HashMap::new();
    hmap.insert(1, 1);
    let mut mx = 0;
    let mut ans = 0;
    for i in 2..n {
        if hmap.contains_key(&i) { continue; }

        let mut b = Vec::new();
        let mut cur = i;
        while cur != 1 {
            b.push(cur);
            cur = if cur & 1 == 0 { cur / 2 } else { 3 * cur + 1 };
            if hmap.contains_key(&cur) {
                b.push(cur);
                break;
            }
        }

        for i in (0..(b.len() - 1)).rev() {
            hmap.insert(b[i], hmap.get(&b[i + 1]).unwrap() + 1);
        }

        if *hmap.get(&i).unwrap() > mx {
            mx = *hmap.get(&i).unwrap();
            ans = i;
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
