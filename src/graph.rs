#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn shortest_dists(neighbor: &[Vec<(usize, i64)>]) -> Vec<Vec<i64>> {
    let n = neighbor.len();
    let mut d = vec![vec![i64::max_value() >> 1; n]; n];
    for v in 0..n {
        for &(w, c) in &neighbor[v] {
            d[v][w] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j])
            }
        }
    }
    d
}

#[test]
fn test_shortest_dists() {
    let neighbor = vec![
        vec![(1, 10), (2, 20), (3, 500)],
        vec![(4, 20)],
        vec![(5, 30)],
        vec![(5, -460)],
        vec![],
        vec![],
    ];
    let d = shortest_dists(&neighbor);
    assert_eq!(d[0][4], 30);
    assert_eq!(d[1][4], 20);
    assert_eq!(d[0][5], 40);
}

#[snippet]
#[snippet(include = "rev")]
fn shortest_dists_from(start: usize, neighbor: &[Vec<(usize, usize)>]) -> Vec<usize> {
    use std::collections::BinaryHeap;
    use std::iter::FromIterator;

    let n = neighbor.len();
    let mut dist = vec![usize::max_value() / 2; n];
    dist[start] = 0;
    let mut que: BinaryHeap<_> = FromIterator::from_iter(dist.iter().map(|&d| Rev(d)).zip(0..n));

    while let Some((Rev(d), u)) = que.pop() {
        for &(v, l) in &neighbor[u] {
            let alt = d + l;
            if dist[v] > alt {
                dist[v] = alt;
                que.push((Rev(alt), v));
            }
        }
    }

    dist
}

#[derive(PartialEq, Eq, Debug)]
struct Rev<T>(T);

impl<T:Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[test]
fn test_shortest_dists_from() {
    let neighbor = vec![
        vec![(1, 7), (2, 9), (5, 14)],
        vec![(0, 7), (2, 10), (3, 15)],
        vec![(0, 9), (1, 10), (3, 11), (5, 2)],
        vec![(1, 15), (2, 11), (4, 6)],
        vec![(3, 6), (5, 9)],
        vec![(0, 14), (2, 2), (4, 9)],
    ];
    assert_eq!(shortest_dists_from(0, &neighbor), vec![0, 7, 9, 20, 20, 11]);
}
