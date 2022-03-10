use proconio::input;
use itertools::Itertools;


/*
 花子さん n 枚のカード
 4 <= n <= 10
 1<= x <= 99
 2 <= k <= 4 を選んで整数を作成
 全部で何種類の整数を作れる？

 1,2,3,13,21 -> 3枚選択 -> 何パターンの整数?
 */
fn main() {
    input! {
		n: usize,
		k: usize,
		mut cards: [String; n]
	}

	println!("{}", cards.iter().permutations(k)
		.map(|x| x.iter().map(|xx| xx.to_string()).collect::<Vec<_>>().join(""))
		.unique().count());
}
