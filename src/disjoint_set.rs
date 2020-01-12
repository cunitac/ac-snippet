#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("disjoint_set")]
struct DisjointSet {
    prt: Vec<usize>,
    rnk: Vec<usize>,
}

#[snippet("disjoint_set")]
impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        DisjointSet {
            prt: (0..n).collect(),
            rnk: vec![0; n],
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        use std::cmp::Ordering;
        match self.rnk[x].cmp(&self.rnk[y]) {
            Ordering::Less => self.prt[x] = y,
            Ordering::Greater => self.prt[y] = x,
            Ordering::Equal => {
                self.prt[y] = x;
                self.rnk[x] += 1;
            }
        };
    }
    fn find(&mut self, x: usize) -> usize {
        if self.prt[x] == x {
            x
        } else {
            let xpt = self.prt[x];
            self.prt[x] = self.find(xpt);
            self.prt[x]
        }
    }
}

#[test]
fn test_disjoint_set() {
    let mut djs = DisjointSet::new(5);
    djs.unite(0, 1);
    djs.unite(1, 2);
    djs.unite(3, 4);
    assert_eq!(djs.find(0), djs.find(1));
    assert_eq!(djs.find(0), djs.find(2));
    assert_eq!(djs.find(3), djs.find(4));
    assert!(djs.find(0) != djs.find(3));
}
