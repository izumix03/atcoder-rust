use proconio::input;
use proconio::marker::Chars;

// https://qiita.com/drken/items/e77685614f3c6bf86f44
fn dfs(x: usize, y: usize, c: &mut Vec<Vec<char>>, h: usize, w: usize) -> bool {
    if c[y][x] == '#' {
        return false;
    }
    if c[y][x] == 'g' {
        return true;
    }
    c[y][x] = '#';

    if x + 1 < w && dfs(x + 1, y, c, h, w) {
        return true;
    }
    if x > 0 && dfs(x - 1, y, c, h, w) {
        return true;
    }
    if y + 1 < h && dfs(x, y + 1, c, h, w) {
        return true;
    }
    if y > 0 && dfs(x, y - 1, c, h, w) {
        return true;
    }
    return false;
}

fn main() {
    input! {
		h: usize,
		w: usize,
		mut c: [Chars; h]
	}
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                if dfs(
                    j,
                    i,
                    &mut c,
                    h, w,
                ) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
