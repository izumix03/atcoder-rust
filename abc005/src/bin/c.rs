use proconio::{input};

fn main() {
    input! {
        T: i32, // 1...100 待たせて良い時間
        N: usize, // 1...100 作る数
        burned: [i32; N],
        M: usize, // 1...100 ユーザー数
        users: [i32; M]
    };

    if M > N {
        println!("no");
        return
    }

    let mut index: i32 = -1;
    for user in users {
        let mut ok = false;

        for (i, burn) in burned.iter().enumerate() {
            if (i as i32) <= index {
                continue;
            }
            // 待たせるとだめ、という制約が必要
            if *burn <= user && user <= *burn + T.clone() {
                index = i as i32;
                ok = true;
                break;
            }
        }
        if !ok {
            println!("no");
            return
        }
    }
    println!("yes");
}
