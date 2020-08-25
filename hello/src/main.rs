struct TraPMember {
    id: i32,
    username: String,
    grade: i32,
}

fn main() {
    // 数値型
    let mut x = 5;
    let y = 5;
    let z = 2000000000000000i64;
    println!("x = {}", x); // x = 5
    println!("y = {}", y); // y = 5
    println!("z = {}", z); // z = 2000000000000000
    x = x + 1;
    // y = y + 1; cannot assign twice to immutable variable
    // println!("y = {}", y); // y = 6 defaultでimmutableなので再代入できない1
    println!("x = {}", x); // x = 6

    // 文字列型
    let s = "This is it. That is that that that that boy said."; // 文字列スライス 参照のみが出来る
    let str = String::from("This is String Type");
    println!("slice  : {}", s);
    println!("string : {}", str);

    // ユーザー定義型
    let lolico = TraPMember {
        id: 114514,
        username: String::from("@lolico"),
        grade: 0,
    };
    // メンバ変数へのアクセス
    println!(
        "lolico's ID -> {}, username -> {}, grade -> {}",
        lolico.id, lolico.username, lolico.grade
    );

    x = -3;
    // Rustにおいて, ifは文ではなく式なので評価が出来る
    let abs_x = if x >= 0 { x } else { -x };
    println!("abs(x) -> {}", abs_x);

    // loop は式なので値を返すことが出来る
    let mut hogehoge = 12345678901234567890u64;
    let mut count = 0;
    let res = loop {
        hogehoge /= 2;
        count += 1;
        if hogehoge == 0 {
            break count;
        }
    };
    println!("hogehoge : {} bit", res);

    'one: loop {
        'two: loop {
            'three: loop {
                println!("three");
                if res == 64 {
                    break 'two;
                } else if res == 63 {
                    break 'three;
                } else {
                    break 'one;
                }
            }
            println!("two");
        }
        println!("one");
        break;
    }
    // three -> one になる
}
