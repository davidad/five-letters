use bitvec::prelude::*;
use itertools::{*, EitherOrBoth::*};
use std::{cell::RefCell, fs::File, io::{BufRead, BufReader}, rc::Rc};
use kdam::prelude::*;

type WordBits = BitArray<[u32;1],Msb0>;

pub fn load(filename: &str) -> (Vec<String>, Vec<WordBits>) {
    let (words, bits) : (Vec<_>, Vec<_>) = sorted(BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(|word| word.chars().count() == 5))
        .filter_map(|word| {
            let mut b = bitarr![u32, Msb0; 0; 26];
            let indices = word.chars()
                .flat_map(|c| c.to_lowercase())
                .map(|c| {
                    assert!(c.is_ascii_lowercase());
                    (c as u8 - b'a') as usize
                });
            for i in indices {
                if b.replace(i,true) { return None }
            }
            Some((word,b))
        })
        .unzip();
    (words,bits)
}

pub fn remove_anagrams(wbs: (Vec<String>, Vec<WordBits>)) -> (Vec<String>, Vec<WordBits>) {
    let (words, bits) = wbs;
    words.into_iter().zip(bits.iter()).unique_by(|(_,b)| b.clone()).unzip()
}

pub fn intersect_with(wbs: (Vec<String>, Vec<WordBits>), filename: &str) -> (Vec<String>, Vec<WordBits>) {
    let others = sorted(BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(|word| word.chars().count() == 5));

    let (words, bits) = wbs;
    words.into_iter().zip(bits.iter()).merge_join_by(others, |(w,_),o| w.cmp(o))
        .filter_map(|m| match m {
            Left(_) => None,
            Right(_) => None,
            Both((w,b),_) => Some((w,b))
        })
        .unzip()
}

pub fn generate_neighborhoods(bits: &Vec<WordBits>) -> Vec<BitVec> {
    let n = bits.len();
    let mut result = vec![bitvec![0;n]; n];
    for x in tqdm!(0..n) {
        for y in (x+1)..n {
            if (bits[x] & bits[y]).not_any() {
                result[x].set(y, true);
            }
        }
    }
    result
}

pub fn find_cliques(neighborhoods: Vec<BitVec>) -> Vec<[u16;5]> {
    let mut result : Vec<[u16;5]> = Vec::new();
    for (i,xs) in tqdm!(neighborhoods.iter().enumerate().rev().skip(4)) {
        let remaining = xs;
        for j in remaining.iter_ones() {
            let remaining = remaining.clone() & &neighborhoods[j];
            if remaining.count_ones() < 3 { continue }
            for k in remaining.iter_ones() {
                let remaining = remaining.clone() & &neighborhoods[k];
                if remaining.count_ones() < 2 { continue }
                for l in remaining.iter_ones() {
                    let remaining = remaining.clone() & &neighborhoods[l];
                    for r in remaining.iter_ones() {
                        result.push([i as u16, j as u16, k as u16, l as u16, r as u16]);
                    }
                }
            }
        }
    }
    result
}

#[derive(Clone)]
pub struct DancingLinks {
    llink : Vec<u16>,
    rlink : Vec<u16>,
    top : Vec<i16>,
    ulink : Vec<u16>,
    dlink : Vec<u16>,
    front : usize,
    n : usize,
    prev_spacer : usize,
    appendix : usize,
}

impl DancingLinks {
    fn new(n: usize, m: usize, k: usize) -> Self {
        let mut llink = vec![0 as u16; m+1];
        let mut rlink = vec![0 as u16; m+1];
        let     top   = vec![0 as i16; m+1+n+1+k];
        let mut ulink = vec![0 as u16; m+1+n+1+k];
        let mut dlink = vec![0 as u16; m+1+n+1+k];

        for i in 0..=m {
            llink[i] = ((i as i32)-1).rem_euclid((m+1) as i32) as u16;
            rlink[i] = (i+1).rem_euclid(m+1) as u16;
            ulink[i] = i as u16;
            dlink[i] = i as u16;
        }

        DancingLinks {
            llink,
            rlink,
            top,
            ulink,
            dlink,
            front: m+1,
            n: 0,
            prev_spacer: 0,
            appendix: m+1,
        }
    }
    fn add_spacer(&mut self) { 
        self.top[self.front as usize] = -(self.n as i16);
        self.ulink[self.front as usize] = self.prev_spacer as u16 + 1;
        self.dlink[self.prev_spacer as usize] = self.front as u16 - 1;
        self.prev_spacer = self.front;
        self.front += 1;
        self.n += 1;
    }
    fn add_node(&mut self, column: u16) {
        let col = column as u16 + 1;
        self.top[self.front] = col as i16;
        let previous_bottom = self.ulink[col as usize] as usize;
        self.dlink[self.front] = col;
        self.ulink[self.front] = previous_bottom as u16;
        self.ulink[col as usize] = self.front as u16;
        self.dlink[previous_bottom] = self.front as u16;
        self.top[col as usize] += 1;
        self.front += 1;
    }
    fn start_appendix(&mut self) {
        self.appendix = self.front;
    }
    fn finalize(&mut self) {
        self.add_spacer();
        self.front -= 1;
        self.n -= 1;
    }

    fn cover(&mut self, i: usize) {
        let mut p = self.dlink[i] as usize;
        while p != i { self.hide(p); p = self.dlink[p] as usize };
        let l = self.llink[i as usize];
        let r = self.rlink[i as usize];
        self.llink[r as usize] = l;
        self.rlink[l as usize] = r;
    }
    fn hide(&mut self, p: usize) {
        let mut q = p + 1;
        while q != p {
            let x = self.top[q];
            let u = self.ulink[q];
            let d = self.dlink[q];
            if x <= 0 {
                q = u as usize;
            } else {
                self.dlink[u as usize] = d;
                self.ulink[d as usize] = u;
                self.top[x as usize] -= 1;
                q += 1;
            }
        }
    }
    fn hide_appendix(&mut self) {
        let mut q = self.appendix + 1;
        while q < self.front {
            let x = self.top[q];
            let u = self.ulink[q];
            let d = self.dlink[q];
            if x > 0 {
                self.dlink[u as usize] = d;
                self.ulink[d as usize] = u;
                self.top[x as usize] -= 1;
            }
            q += 1;
        }
    }
    fn uncover(&mut self, i: usize) {
        let l = self.llink[i];
        let r = self.rlink[i];
        self.rlink[l as usize] = i as u16;
        self.llink[r as usize] = i as u16;
        let mut p = self.ulink[i] as usize;
        while p != i {
            self.unhide(p);
            p = self.ulink[p] as usize;
        }
    }
    fn unhide(&mut self, p: usize) {
        let mut q = p - 1;
        while q != p {
            let x = self.top[q];
            let u = self.ulink[q];
            let d = self.dlink[q];
            if x <= 0 {
                q = d as usize;
            } else {
                self.dlink[u as usize] = q as u16;
                self.ulink[d as usize] = q as u16;
                self.top[x as usize] += 1;
                q -= 1;
            }
        }
    }
    fn unhide_appendix(&mut self) {
        let mut q = self.front - 1;
        while q > self.appendix {
            let x = self.top[q];
            let u = self.ulink[q];
            let d = self.dlink[q];
            if x > 0 {
                self.dlink[u as usize] = q as u16;
                self.ulink[d as usize] = q as u16;
                self.top[x as usize] += 1;
            }
            q -= 1;
        }
    }

    pub fn solve(&mut self) -> Vec<[u16;5]> {
        let mut results : Vec<[u16;5]> = Vec::new();
        let mut x : [u16;6] = [0; 6];
        
        fn recurse(o: &mut DancingLinks, results: &mut Vec<[u16;5]>, x: &mut [u16;6], mut progress: Option<Rc<RefCell<kdam::Bar>>>, l : usize) {
            let mut n_cols = 0;
            let mut min_k : isize = isize::MAX;
            let mut argmin_k = 0;
            let mut c = 0;
            while {c = o.rlink[c] as usize; c > 0} {
                n_cols += 1;
                if (o.top[c] as isize) < min_k {
                    min_k = o.top[c] as isize;
                    argmin_k = c;
                }
            }
            if n_cols == 0 {
                results.push(x.iter().map(|i| {
                    let mut j = *i-1;
                    while o.top[j as usize] > 0 {
                        j = j-1;
                    }
                    -o.top[j as usize] as u16
                }).filter(|i| *i < (o.n-26) as u16).collect::<Vec<_>>().try_into().unwrap());
            } else {
                if l == 0 {
                    progress = Some(Rc::new(RefCell::new(kdam::Bar::new(min_k as usize)))); 
                }
                let i = argmin_k;
                o.cover(i);
                x[l] = i as u16;
                while {x[l] = o.dlink[x[l] as usize] as u16; x[l] as usize != i} {
                    if l==0 {
                        let p = progress.clone().unwrap();
                        p.borrow_mut().update(1); p.borrow_mut().refresh();
                    }
                    {
                        if (x[l] as usize) > o.appendix {
                            o.hide_appendix();
                        }
                        let mut p : usize = x[l] as usize + 1;
                        while x[l] as usize != p {
                            let j = o.top[p];
                            if j <= 0 {
                                p = o.ulink[p] as usize;
                            } else {
                                o.cover(j as usize);
                                p += 1;
                            }
                        }
                    }
                    recurse(o, results, x, progress.clone(), l+1);
                    {
                        let mut p : usize = x[l] as usize - 1;
                        while x[l] as usize != p {
                            let j = o.top[p];
                            if j <= 0 {
                                p = o.dlink[p] as usize;
                            } else {
                                o.uncover(j as usize);
                                p -= 1;
                            }
                        }
                        if (x[l] as usize) > o.appendix {
                            o.unhide_appendix();
                        }
                    }
                }
                o.uncover(i);
            }
        }
        
        recurse(self, &mut results, &mut x, None, 0);
        results
    }
}

pub fn init_dancing_links(wb: &Vec<WordBits>) -> DancingLinks {
    let n : usize = wb.len();
    let m : usize = 26;
    let k : usize = wb.iter().map(|w| w.count_ones()).sum();

    let mut d = DancingLinks::new(n+26,m,k+26);
    for w in tqdm!(wb.iter()) {
        d.add_spacer();
        for k in w.iter_ones() {
            d.add_node(k as u16);
        }
    }
    d.start_appendix();
    for c in 0..26 {
        d.add_spacer();
        d.add_node(c as u16);
    }
    d.finalize();

    d
}

pub fn fmt_solutions(words: &Vec<String>, solutions: Vec<[u16;5]>) -> String {
    Itertools::intersperse(
        solutions.iter().map(|solution| {
            Itertools::intersperse(
                solution.iter()
                .map(|i| words[*i as usize].clone()),
                "\t".to_string())
                .collect::<String>()
        }),
        "\n".to_string()).collect::<String>()
}
