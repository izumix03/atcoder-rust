use proconio::{input};

fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
        cd: [(usize,usize); n]
    }
    ;

    let mut graph = vec![vec![]; 2 * n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        for (j, &(c, d)) in cd.iter().enumerate() {
            if a < c && b < d {
                graph[i].push(n + j);
            }
        }
    }
    println!("{}", bipartite_matching(&graph));
}

pub fn bipartite_matching(g: &[Vec<usize>]) -> usize {

    fn dfs(v: usize, g: &[Vec<usize>], matched: &mut [Option<usize>], used: &mut [bool]) -> bool {
        used[v] = true;
        for &u in &g[v] {
            if matched[u].is_none()
                || !used[matched[u].unwrap()]
                && dfs(matched[u].unwrap(), g, matched, used) {
                matched[v] = Some(u);
                matched[u] = Some(v);
                return true;
            }
        }
        false
    }

    let mut res = 0;
    let mut matched = vec![None; g.len()];

    for v in 0..g.len() {
        if matched[v].is_none() {
            let mut used = vec![false; g.len()];
            if dfs(v, g, &mut matched, &mut used) {
                res += 1;
            }
        }
    }
    res
}