use proconio::{input};
use proconio::marker::Chars;

/// 文字列 A
/// A より小さいものを何でも出力
///
/// 1文字以上 100文字以下
/// ない場合は -1
///
///
fn main() {
    input! {
        t: Chars,
    }
    if t.len() == 1 && t[0] == 'a' {
        println!("-1");
        return;
    }
    println!("a");
}
