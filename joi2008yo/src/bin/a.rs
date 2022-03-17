use proconio::input;

/// おつり
/// 500円、100円、50円、10円、5円、1円
/// 1000円出してお釣りを最小枚数でもらう
/// ※下の硬貨で割り切れる場合はなりたつ
fn main() {
    input! {
		n: i32
	}
    let mut money = 1000 -n;
    let mut count = 0;
    [500, 100, 50, 10, 5, 1].iter().for_each(|v|
        while money >= *v {
            money -= *v;
            count+=1;
        });

    println!("{}", count)
}
