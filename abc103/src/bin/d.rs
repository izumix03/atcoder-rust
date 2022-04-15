/// 区間スケジューリング
/// N個の島とN-1本の橋
/// i 番目 = 西から i番目とi+1番目の島を接続
/// M 個の要望
///   i: x と y をいけなくする
use proconio::{input};

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut ab: [(i32,i32);m]
    }

    ab.sort_by(|(_, b1), (_, b2)| b1.cmp(b2));

    let mut num = 0;
    let mut prev_island_pair = (0, 0);
    for pair in ab {
        if prev_island_pair.1 <= pair.0 {
            num += 1;
            prev_island_pair = pair;
        }
    }
    println!("{}", num);
}
