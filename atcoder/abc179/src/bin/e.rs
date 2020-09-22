use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {
        n:i64,
        mut x:i64,
        m:i64,
    }

    let mut amari = HashMap::new();
    // 0番目はあまりが0(Key: あまり, Value 順番)
    amari.insert(-1i64, 0i64);
    let mut res = 0i64;
    let mut ruiseki = vec![0i64; 100005];

    let mut ikkai = 0i64;
    let mut start = -1i64;
    let mut junban = 0i64;
    let mut syuuki = -1i64;
    loop {
        junban += 1;
        ruiseki[junban as usize] = ruiseki[(junban - 1) as usize] + x;

        if amari.contains_key(&x) {
            syuuki = junban - amari.get(&x).unwrap();
            ikkai = ruiseki[junban as usize] - ruiseki[(junban - syuuki) as usize];
            start = *amari.get(&x).unwrap();
            break;
        } else {
            amari.insert(x, junban);
            x = (x * x) % m;

            if junban == n {
                println!("{}", ruiseki[n as usize]);
                return;
            }
        }
    }

    res = ruiseki[(start - 1) as usize];
    // m/周期と余りで良いので
    res += (n - start + 1) / syuuki * ikkai;

    for _ in 0..((n - start + 1) % syuuki) {
        res += x;
        x = (x * x) % m;
    }

    /*println!(
        "ruiseki: {:?}, syuuki : {}, ikkai : {}, nokori : {}",
        ruiseki,
        syuuki,
        ikkai,
        n % syuuki
    );
    */
    println!("{}", res);
}
