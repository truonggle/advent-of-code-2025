use std::{cmp::Reverse, mem::swap};

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize, // For Part 2
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, mut i: usize) -> usize {
        while i != self.parent[i] {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (mut root_i, mut root_j) = (self.find(i), self.find(j));
        if root_i == root_j {
            return false;
        }

        // Canonical Ordering: Ensure `root_i` is always the smaller tree
        if self.size[root_i] > self.size[root_j] {
            swap(&mut root_i, &mut root_j);
        }

        self.parent[root_i] = root_j;
        self.size[root_j] += self.size[root_i];
        self.count -= 1;
        true
    }
}

fn get_all_edges(points: &[Point]) -> Vec<(u128, usize, usize)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let (p1, p2) = (points[i], points[j]);
            let dist_sq = (p1.x.abs_diff(p2.x) as u128).pow(2)
                + (p1.y.abs_diff(p2.y) as u128).pow(2)
                + (p1.z.abs_diff(p2.z) as u128).pow(2);
            edges.push((dist_sq, i, j));
        }
    }
    edges
}

fn part_2(points: &[Point]) {
    let mut edges = get_all_edges(&points);
    edges.sort_unstable();

    let mut dsu = DSU::new(points.len());
    for &(_, u, v) in &edges {
        if dsu.union(u, v) {
            if dsu.count == 1 {
                let ans = points[u].x as u128 * points[v].x as u128;
                print!("{ans}");
                return;
            }
        }
    }
}

fn part_1(points: &[Point]) {
    let mut edges = get_all_edges(points);
    if edges.len() > 1000 {
        edges.select_nth_unstable(1000);
        edges.truncate(1000);
    }
    edges.sort_unstable();

    let mut dsu = DSU::new(points.len());
    for &(_, u, v) in &edges {
        dsu.union(u, v);
    }

    let mut circuit_sizes: Vec<usize> = (0..points.len())
        .filter(|&i| i == dsu.parent[i])
        .map(|i| dsu.size[i])
        .collect();
    circuit_sizes.sort_unstable_by_key(|&s| Reverse(s));
    let ans: u128 = circuit_sizes.iter().take(3).map(|&s| s as u128).product();
    print!("{ans}")
}

fn main() {
    let points: Vec<Point> = include_str!("input.txt")
        .lines()
        .filter_map(|line| {
            let mut iter = line.split(',').map(|n| n.trim().parse().ok());
            Some(Point {
                x: iter.next().flatten()?,
                y: iter.next().flatten()?,
                z: iter.next().flatten()?,
            })
        })
        .collect();
    //part_1(&points);
    part_2(&points);
}
