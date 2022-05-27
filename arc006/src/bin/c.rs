use proconio::{input};

fn main() {
    input! {
        n: usize,
        mut w: [i32;n]
    }

    // 順序付き
    let mut boxes: Vec<i32> = Vec::new();

    for n in w {
        boxes.sort();
        let mut ok = false;
        for (pos, b) in boxes.iter().enumerate() {
            if *b >= n {
                boxes[pos] = n;
                ok = true;
                break;
            }
        }

        if !ok {
            boxes.push(n);
        }
    }

    println!("{}", boxes.len());
}
