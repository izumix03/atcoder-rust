use proconio::{input};

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    let mut n = 0;

    while x * 2_i64.pow(n) <= y {
        n += 1;
    }

    println!("{}", n);
}
