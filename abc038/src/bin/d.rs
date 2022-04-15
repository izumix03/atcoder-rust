use proconio::{input};

fn main() {
    input! {
        n: usize,
        mut wh: [(usize,usize);n]
    }
    ;

    const INF: usize = 1 << 30;

    /// N 個あり
    /// i番目は h w サイズ
    /// 入れ子にしよう

    // 並び替え
    wh.sort_by_cached_key(|&(w, h)|
        (w as isize, (h as isize * (-1)))
    );
    // println!("{:?}", wh);

    let raster: Vec<usize> = wh.iter().map(|&(w, h) | h).collect();

    let mut dp = vec![INF; n+1];
    // println!("{:?}", dp);
    for x in raster {
        // 見つからない場合にソートを乱さない idx を返す
        let idx = dp.binary_search(&x).unwrap_or_else(|idx| idx);
        dp[idx] = x;
    }

    for (i, &x) in dp.iter().enumerate() {
        if x == INF {
            println!("{}", i);
            return
        }
    }
    // 最大を返す
}
