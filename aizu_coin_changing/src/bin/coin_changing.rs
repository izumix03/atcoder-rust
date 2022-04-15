use std::io;
use std::collections::VecDeque;
use std::io::prelude::*;

/// m 種類のコイン
/// n 円払う
/// コインの最小枚数を答える
/// コインは何度でも使用可
fn main() {
    // input! {
	// 	n: usize,
	// 	m: usize,
	// 	coins: [usize; m]
	// }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    let _ = stdin.read_to_string(&mut buf);
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let _m: usize = iter.next().unwrap().parse().unwrap();
    let coins: Vec<usize> = iter.map(|s| s.parse().unwrap()).collect();

    println!("{:?}", dp(n, coins));
}

// 使った枚数
fn dp(n: usize, coins: Vec<usize>) -> u32 {
    let mut q = VecDeque::new();
    let mut visited = vec![0; n + 1];

    for coin in &coins {
        if n >= *coin {
            q.push_back(*coin);
            visited[*coin] = 1;
        }
    }

    while visited[n] == 0 {
        let now = q.pop_front();

        for coin in &coins {
            let next = now.unwrap() + coin;

            if next <= n && visited[next] == 0 {
                q.push_back(next);
                visited[next] = visited[now.unwrap()] + 1;
            }
        }
    }
    visited[n]
}