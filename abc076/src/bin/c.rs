use proconio::{input};
use proconio::marker::Chars;

fn main() {
    input! {
        sq: Chars,
        t: Chars,
    }
    ;

    let mut restoreable = false;
    let mut s = sq.clone();

    for (i, w) in sq.windows(t.len()).enumerate().rev() {
        if can_replace(w, &t) {
            for j in 0..(t.len()) {
                s[i + j] = t[j];
            }
            restoreable = true;
            break;
        }
    }

    if !restoreable {
        println!("UNRESTORABLE");
        return;
    }

    let s = s.iter()
        .map(|&c| if c == '?' { 'a' } else { c })
        .collect::<String>();
    println!("{}", s);
}


fn can_replace(s: &[char], t: &[char]) -> bool {
    for (&l, &r) in s.iter().zip(t) {
        if !(l == r || l == '?') {
            return false;
        }
    }
    true
}