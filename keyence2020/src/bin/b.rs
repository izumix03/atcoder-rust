/// N個のロボット
/// ロボットi = Xi に配置
/// ロボットi = Li 長さまで伸ばせる
/// ロボットを取り除いて重複がないようにする
/// 残せるロボットの最大を求めよ
/// https://atcoder.jp/contests/keyence2020/tasks/keyence2020_b
use proconio::{input};

// 区間の終わりに注目！
fn main() {
    input! {n:usize,xln:[(isize,isize);n]}
    let mut sections = vec![];
    let mut ans = 0;

    for (x, l) in xln {
        sections.push((x + l, x - l));
    }
    sections.sort();

    let mut cur = std::isize::MIN;
    for (end, start) in sections {
        if cur <= start {
            ans += 1;
            cur = end;
        }
    }
    println!("{}", ans);
}